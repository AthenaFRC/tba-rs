use std::collections::BTreeSet;

use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use super::{doc_lines, validate_lengths, validate_unique_variant};
use crate::inputs::ModelOverride;
use crate::inputs::spec::Schema;
use crate::util::strings;

#[derive(Debug, PartialEq, Eq)]
pub struct StringEnum {
	name: String,
	description: Option<String>,
	variants: Vec<StringEnumVariant>,
}

#[derive(Debug, PartialEq, Eq)]
struct StringEnumVariant {
	name: String,
	value: String,
}

impl StringEnum {
	pub fn parse(
		schema_name: &str,
		schema: &Schema,
		model_override: Option<&ModelOverride>,
	) -> Result<Self, String> {
		if schema.simple_type() != Some("string") {
			return Err(format!(
				"string enum `{schema_name}` does not have type `string`"
			));
		}
		let type_name = schema.title.as_deref().unwrap_or(schema_name);
		let name = strings::type_name(type_name)?;
		let values = schema.values.as_ref().ok_or_else(|| {
			format!("string enum `{schema_name}` has no values")
		})?;
		let schema_variant_names = match &schema.variant_names {
			Some(variant_names) => {
				validate_lengths(schema_name, values, variant_names)?;
				variant_names.iter().map(String::as_str).collect::<Vec<_>>()
			}
			None => values
				.iter()
				.map(|value| {
					value.as_str().ok_or_else(|| {
						format!(
							"string enum `{schema_name}` contains non-string value {value}"
						)
					})
				})
				.collect::<Result<Vec<_>, _>>()?,
		};
		let has_schema_variant_names = schema.variant_names.is_some();
		let mut unused_overrides = model_override
			.into_iter()
			.flat_map(|model| model.variants.keys().cloned())
			.collect::<BTreeSet<_>>();

		let mut seen_names = BTreeSet::new();
		let mut seen_values = BTreeSet::new();
		let variants = schema_variant_names
			.iter()
			.zip(values)
			.map(|(schema_variant_name, value)| {
				let overridden_name = model_override
					.and_then(|model| model.variants.get(*schema_variant_name));
				let name = match overridden_name {
					Some(name) => strings::type_name(name)?,
					None if has_schema_variant_names => {
						strings::variant_name(schema_variant_name)?
					}
					None => strings::wire_variant_name(schema_variant_name)?,
				};
				let value = value.as_str().ok_or_else(|| {
					format!(
						"string enum `{schema_name}` contains non-string value {value}"
					)
				})?;
				validate_unique_variant(
					schema_name,
					&name,
					&value,
					&mut seen_names,
					&mut seen_values,
				)?;
				unused_overrides.remove(*schema_variant_name);
				Ok(StringEnumVariant {
					name,
					value: value.to_owned(),
				})
			})
			.collect::<Result<Vec<_>, String>>()?;

		if !unused_overrides.is_empty() {
			return Err(unused_override_error(schema_name, unused_overrides));
		}

		Ok(Self {
			name,
			description: schema.description.clone(),
			variants,
		})
	}

	pub fn name(&self) -> &str {
		&self.name
	}

	pub fn render(&self) -> TokenStream {
		let name = format_ident!("{}", self.name);
		let docs = doc_lines(&self.description);
		let variant_names = self
			.variants
			.iter()
			.map(|variant| format_ident!("{}", variant.name))
			.collect::<Vec<_>>();
		let values = self
			.variants
			.iter()
			.map(|variant| &variant.value)
			.collect::<Vec<_>>();

		quote! {
			#(#[doc = #docs])*
			#[derive(
				Debug,
				Clone,
				PartialEq,
				Eq,
				serde::Deserialize,
				serde::Serialize,
			)]
			pub enum #name {
				#(
					#[serde(rename = #values)]
					#variant_names,
				)*
			}
		}
	}
}

fn unused_override_error(
	schema_name: &str,
	unused_overrides: BTreeSet<String>,
) -> String {
	let overrides = unused_overrides
		.into_iter()
		.map(|variant| format!("{schema_name}.{variant}"))
		.collect::<Vec<_>>()
		.join(", ");
	format!("unused codegen overrides: {overrides}")
}

#[cfg(test)]
mod tests {
	use crate::inputs::Overrides;

	use super::*;

	#[test]
	fn parses_override_and_renders_exact_wire_value() {
		let schema: Schema = serde_json::from_str(
			r#"{
				"type": "string",
				"enum": ["a\"b"],
				"x-enum-varnames": ["QUOTED"]
			}"#,
		)
		.unwrap();
		let overrides = Overrides::parse(
			r#"[models.Example.variants]
			QUOTED = "Escaped"
			"#,
		)
		.unwrap();

		let model = StringEnum::parse(
			"Example",
			&schema,
			overrides.models.get("Example"),
		)
		.unwrap();
		let rendered = model.render().to_string();
		assert_eq!(model.name(), "Example");
		assert!(rendered.contains("serde (rename = \"a\\\"b\")"));
		assert!(rendered.contains("Escaped"));
	}

	#[test]
	fn rejects_a_stale_variant_override() {
		let schema: Schema =
			serde_json::from_str(r#"{ "type": "string", "enum": ["known"] }"#)
				.unwrap();
		let overrides = Overrides::parse(
			r#"[models.Example.variants]
			missing = "Missing"
			"#,
		)
		.unwrap();

		let error = StringEnum::parse(
			"Example",
			&schema,
			overrides.models.get("Example"),
		)
		.unwrap_err();
		assert!(error.contains("Example.missing"));
	}
}
