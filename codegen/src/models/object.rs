use std::collections::{BTreeMap, BTreeSet};

use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use super::{StringEnum, doc_lines, rust_type::RustType};
use crate::{
	inputs::ModelOverride,
	inputs::spec::{AdditionalProperties, Schema},
	util::strings,
};

#[derive(Debug, PartialEq, Eq)]
pub struct Object {
	name: String,
	definitions: Vec<Definition>,
}

#[derive(Debug, PartialEq, Eq)]
enum Definition {
	Struct(StructDefinition),
	Alias(AliasDefinition),
	StringEnum(StringEnum),
	UntaggedEnum(UntaggedEnumDefinition),
}

#[derive(Debug, PartialEq, Eq)]
struct StructDefinition {
	name: String,
	description: Option<String>,
	fields: Vec<StructField>,
}

#[derive(Debug, PartialEq, Eq)]
struct StructField {
	name: String,
	wire_name: String,
	description: Option<String>,
	type_: RustType,
}

#[derive(Debug, PartialEq, Eq)]
struct AliasDefinition {
	name: String,
	description: Option<String>,
	type_: RustType,
}

#[derive(Debug, PartialEq, Eq)]
struct UntaggedEnumDefinition {
	name: String,
	description: Option<String>,
	variants: Vec<UntaggedEnumVariant>,
}

#[derive(Debug, PartialEq, Eq)]
struct UntaggedEnumVariant {
	name: String,
	type_: RustType,
}

impl Object {
	pub fn parse(schema_name: &str, schema: &Schema) -> Result<Self, String> {
		Self::parse_with_override(schema_name, schema, None)
	}

	pub fn parse_with_override(
		schema_name: &str,
		schema: &Schema,
		model_override: Option<&ModelOverride>,
	) -> Result<Self, String> {
		if !is_object(schema) {
			return Err(format!(
				"model `{schema_name}` is not an object schema"
			));
		}
		let source_name = schema.title.as_deref().unwrap_or(schema_name);
		let mut parser = ObjectParser::new(model_override);
		let name = parser.resolve_type_name(source_name)?;
		parser.define_object(&name, schema)?;
		parser.ensure_overrides_used(schema_name)?;
		Ok(Self {
			name,
			definitions: parser.definitions,
		})
	}

	pub fn name(&self) -> &str {
		&self.name
	}

	pub fn render(&self) -> TokenStream {
		let definitions = self.definitions.iter().map(Definition::render);
		quote!(#(#definitions)*)
	}
}

impl Definition {
	fn render(&self) -> TokenStream {
		match self {
			Self::Struct(definition) => definition.render(),
			Self::Alias(definition) => definition.render(),
			Self::StringEnum(definition) => definition.render(),
			Self::UntaggedEnum(definition) => definition.render(),
		}
	}
}

impl StructDefinition {
	fn render(&self) -> TokenStream {
		let name = format_ident!("{}", self.name);
		let docs = doc_lines(&self.description);
		let fields = self.fields.iter().map(StructField::render);
		quote! {
			#(#[doc = #docs])*
			#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
			pub struct #name {
				#(#fields)*
			}
		}
	}
}

impl StructField {
	fn render(&self) -> TokenStream {
		let name = format_ident!("{}", self.name);
		let type_ = self.type_.render();
		let docs = doc_lines(&self.description);
		let rename = (self.name != self.wire_name).then(|| {
			let wire_name = &self.wire_name;
			quote!(#[serde(rename = #wire_name)])
		});
		quote! {
			#(#[doc = #docs])*
			#rename
			pub #name: #type_,
		}
	}
}

impl AliasDefinition {
	fn render(&self) -> TokenStream {
		let name = format_ident!("{}", self.name);
		let type_ = self.type_.render();
		let docs = doc_lines(&self.description);
		quote! {
			#(#[doc = #docs])*
			pub type #name = #type_;
		}
	}
}

impl UntaggedEnumDefinition {
	fn render(&self) -> TokenStream {
		let name = format_ident!("{}", self.name);
		let docs = doc_lines(&self.description);
		let variants = self.variants.iter().map(|variant| {
			let name = format_ident!("{}", variant.name);
			let type_ = variant.type_.render();
			quote!(#name(#type_),)
		});
		quote! {
			#(#[doc = #docs])*
			#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
			#[serde(untagged)]
			pub enum #name {
				#(#variants)*
			}
		}
	}
}

struct ObjectParser {
	definitions: Vec<Definition>,
	defined_names: BTreeSet<String>,
	type_names: BTreeMap<String, String>,
	unused_type_names: BTreeSet<String>,
	unused_variants: BTreeSet<String>,
}

impl ObjectParser {
	fn new(model_override: Option<&ModelOverride>) -> Self {
		let type_names = model_override
			.map(|model_override| model_override.types.clone())
			.unwrap_or_default();
		Self {
			definitions: Vec::new(),
			defined_names: BTreeSet::new(),
			unused_type_names: type_names.keys().cloned().collect(),
			type_names,
			unused_variants: model_override
				.into_iter()
				.flat_map(|model_override| {
					model_override.variants.keys().cloned()
				})
				.collect(),
		}
	}

	fn resolve_type_name(&mut self, source: &str) -> Result<String, String> {
		let default_name = strings::type_name(source)?;
		match self.type_names.get(&default_name) {
			Some(overridden_name) => {
				self.unused_type_names.remove(&default_name);
				strings::type_name(overridden_name)
			}
			None => Ok(default_name),
		}
	}

	fn ensure_overrides_used(&self, schema_name: &str) -> Result<(), String> {
		let unused = self
			.unused_type_names
			.iter()
			.map(|name| format!("{schema_name}.types.{name}"))
			.chain(
				self.unused_variants
					.iter()
					.map(|name| format!("{schema_name}.variants.{name}")),
			)
			.collect::<Vec<_>>();
		if unused.is_empty() {
			Ok(())
		} else {
			Err(format!("unused codegen overrides: {}", unused.join(", ")))
		}
	}

	fn define_object(
		&mut self,
		name: &str,
		schema: &Schema,
	) -> Result<RustType, String> {
		if !self.defined_names.insert(name.to_owned()) {
			return Ok(RustType::Named(name.to_owned()));
		}

		let definition = if !schema.properties.is_empty() {
			Definition::Struct(self.parse_struct(name, schema)?)
		} else if let Some(additional_properties) =
			&schema.additional_properties
		{
			let value_type = self.parse_additional_properties(
				&format!("{name}Value"),
				additional_properties,
			)?;
			Definition::Alias(AliasDefinition {
				name: name.to_owned(),
				description: schema.description.clone(),
				type_: RustType::Map(Box::new(value_type)),
			})
		} else {
			Definition::Alias(AliasDefinition {
				name: name.to_owned(),
				description: schema.description.clone(),
				type_: RustType::UnknownJsonObject,
			})
		};
		self.definitions.push(definition);
		Ok(RustType::Named(name.to_owned()))
	}

	fn parse_struct(
		&mut self,
		name: &str,
		schema: &Schema,
	) -> Result<StructDefinition, String> {
		let fields = schema
			.properties
			.iter()
			.map(|(wire_name, field_schema)| {
				let field_name = strings::field_name(wire_name)?;
				let type_name =
					format!("{name}{}", strings::type_name(wire_name)?);
				let mut type_ = self.parse_type(&type_name, field_schema)?;
				if !schema.required.contains(wire_name)
					|| field_schema.is_nullable()
				{
					type_ = type_.optional();
				}
				Ok(StructField {
					name: field_name,
					wire_name: wire_name.clone(),
					description: field_schema.description.clone(),
					type_,
				})
			})
			.collect::<Result<Vec<_>, String>>()?;
		Ok(StructDefinition {
			name: name.to_owned(),
			description: schema.description.clone(),
			fields,
		})
	}

	fn parse_type(
		&mut self,
		name: &str,
		schema: &Schema,
	) -> Result<RustType, String> {
		if let Some(reference) = &schema.reference {
			return Ok(RustType::Named(reference_type_name(reference)?));
		}
		let name = self.resolve_type_name(name)?;

		if !schema.one_of.is_empty() {
			let alternatives = schema
				.one_of
				.iter()
				.filter(|alternative| !alternative.is_null())
				.collect::<Vec<_>>();
			return match alternatives.as_slice() {
				[] => Err(format!("union `{name}` contains only null")),
				[alternative] => self.parse_type(&name, alternative),
				_ => self.define_union(
					&name,
					schema.description.clone(),
					&alternatives,
				),
			};
		}

		if !schema.all_of.is_empty() {
			return match schema.all_of.as_slice() {
				[inner] => self.parse_type(&name, inner),
				_ => Err(format!(
					"inline schema `{name}` uses unsupported multi-part allOf"
				)),
			};
		}

		if schema.values.is_some() {
			if !self.defined_names.insert(name.to_owned()) {
				return Ok(RustType::Named(name.to_owned()));
			}
			let definition = StringEnum::parse(&name, schema, None)?;
			self.definitions.push(Definition::StringEnum(definition));
			return Ok(RustType::Named(name.to_owned()));
		}

		if schema.has_type("string") {
			Ok(RustType::String)
		} else if schema.has_type("integer") {
			Ok(RustType::I64)
		} else if schema.has_type("number") {
			Ok(RustType::F64)
		} else if schema.has_type("boolean") {
			Ok(RustType::Bool)
		} else if schema.has_type("array") {
			let items = schema
				.items
				.as_ref()
				.ok_or_else(|| format!("array schema `{name}` has no items"))?;
			Ok(RustType::Vec(Box::new(
				self.parse_type(&format!("{name}Item"), items)?,
			)))
		} else if is_object(schema) {
			if !schema.properties.is_empty() {
				self.define_object(&name, schema)
			} else if let Some(additional_properties) =
				&schema.additional_properties
			{
				let value_type = self.parse_additional_properties(
					&format!("{name}Value"),
					additional_properties,
				)?;
				Ok(RustType::Map(Box::new(value_type)))
			} else {
				Ok(RustType::UnknownJsonObject)
			}
		} else {
			Err(format!("schema `{name}` has an unsupported type"))
		}
	}

	fn parse_additional_properties(
		&mut self,
		name: &str,
		additional_properties: &AdditionalProperties,
	) -> Result<RustType, String> {
		match additional_properties {
			AdditionalProperties::Bool(true) => Ok(RustType::JsonValue),
			AdditionalProperties::Bool(false) => Ok(RustType::JsonValue),
			AdditionalProperties::Schema(schema) => {
				self.parse_type(name, schema)
			}
		}
	}

	fn define_union(
		&mut self,
		name: &str,
		description: Option<String>,
		alternatives: &[&Schema],
	) -> Result<RustType, String> {
		if !self.defined_names.insert(name.to_owned()) {
			return Ok(RustType::Named(name.to_owned()));
		}
		let mut used_names = BTreeSet::new();
		let mut variants = Vec::new();
		for (index, alternative) in alternatives.iter().enumerate() {
			let type_ = self.parse_type(
				&format!("{name}Variant{}", index + 1),
				alternative,
			)?;
			let base_name = type_.union_variant_name();
			let mut variant_name = base_name.clone();
			let mut suffix = 2;
			while !used_names.insert(variant_name.clone()) {
				variant_name = format!("{base_name}{suffix}");
				suffix += 1;
			}
			variants.push(UntaggedEnumVariant {
				name: variant_name,
				type_,
			});
		}
		self.definitions.push(Definition::UntaggedEnum(
			UntaggedEnumDefinition {
				name: name.to_owned(),
				description,
				variants,
			},
		));
		Ok(RustType::Named(name.to_owned()))
	}
}

fn is_object(schema: &Schema) -> bool {
	schema.has_type("object")
		|| !schema.properties.is_empty()
		|| schema.additional_properties.is_some()
}

fn reference_type_name(reference: &str) -> Result<String, String> {
	let name = reference
		.strip_prefix("#/components/schemas/")
		.ok_or_else(|| format!("unsupported schema reference `{reference}`"))?;
	strings::type_name(name)
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn parses_nested_objects_maps_arrays_and_nullability() {
		let schema: Schema = serde_json::from_str(
			r#"{
				"type": "object",
				"required": ["key", "items", "values"],
				"properties": {
					"key": { "type": "string" },
					"optionalValue": { "type": "integer" },
					"items": {
						"type": "array",
						"items": {
							"type": "object",
							"required": ["enabled"],
							"properties": { "enabled": { "type": "boolean" } }
						}
					},
					"values": {
						"type": "object",
						"additionalProperties": { "type": "number" }
					}
				}
			}"#,
		)
		.unwrap();

		let model = Object::parse("Example", &schema).unwrap();
		let rendered = model.render().to_string();
		assert!(rendered.contains("pub struct Example"));
		assert!(rendered.contains("pub struct ExampleItemsItem"));
		assert!(rendered.contains("optional_value : Option < i64 >"));
		assert!(rendered.contains("HashMap < String , f64 >"));
		assert!(rendered.contains("serde (rename = \"optionalValue\")"));
	}

	#[test]
	fn parses_a_nullable_reference_union() {
		let schema: Schema = serde_json::from_str(
			r##"{
				"type": "object",
				"required": ["district"],
				"properties": {
					"district": {
						"oneOf": [
							{ "$ref": "#/components/schemas/District" },
							{ "type": "null" }
						]
					}
				}
			}"##,
		)
		.unwrap();
		let rendered = Object::parse("Event", &schema)
			.unwrap()
			.render()
			.to_string();
		assert!(rendered.contains("district : Option < District >"));
	}

	#[test]
	fn parses_shape_based_unions() {
		let schema: Schema = serde_json::from_str(
			r#"{
				"type": "object",
				"required": ["value"],
				"properties": {
					"value": {
						"oneOf": [
							{ "type": "string" },
							{ "type": "integer" },
							{
								"type": "array",
								"items": { "type": "string" }
							}
						]
					}
				}
			}"#,
		)
		.unwrap();

		let rendered = Object::parse("Example", &schema)
			.unwrap()
			.render()
			.to_string();
		assert!(rendered.contains("pub enum ExampleValue"));
		assert!(rendered.contains("String (String)"));
		assert!(rendered.contains("I64 (i64)"));
		assert!(rendered.contains("Vec (Vec < String >)"));
		assert!(rendered.contains("# [serde (untagged)]"));
	}
}
