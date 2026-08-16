use std::collections::BTreeSet;

use proc_macro2::{Literal, TokenStream};
use quote::{format_ident, quote};

use super::{doc_lines, validate_lengths, validate_unique_variant};
use crate::inputs::spec::Schema;
use crate::util::strings;

#[derive(Debug, PartialEq, Eq)]
pub struct IntegerEnum {
	name: String,
	description: Option<String>,
	variants: Vec<IntegerEnumVariant>,
}

#[derive(Debug, PartialEq, Eq)]
struct IntegerEnumVariant {
	name: String,
	value: i64,
}

impl IntegerEnum {
	pub fn parse(schema_name: &str, schema: &Schema) -> Result<Self, String> {
		if schema.simple_type() != Some("integer") {
			return Err(format!(
				"integer enum `{schema_name}` does not have type `integer`"
			));
		}
		let type_name = schema.title.as_deref().unwrap_or(schema_name);
		let name = strings::type_name(type_name)?;
		let values = schema.values.as_ref().ok_or_else(|| {
			format!("integer enum `{schema_name}` has no values")
		})?;
		let variant_names = schema.variant_names.as_ref().ok_or_else(|| {
			format!("integer enum `{schema_name}` has no x-enum-varnames")
		})?;
		validate_lengths(schema_name, values, variant_names)?;

		let mut seen_names = BTreeSet::new();
		let mut seen_values = BTreeSet::new();
		let variants = variant_names
			.iter()
			.zip(values)
			.map(|(schema_variant_name, value)| {
				let name = strings::variant_name(schema_variant_name)?;
				let value = value.as_i64().ok_or_else(|| {
					format!(
						"integer enum `{schema_name}` contains non-i64 value {value}"
					)
				})?;
				validate_unique_variant(
					schema_name,
					&name,
					&value,
					&mut seen_names,
					&mut seen_values,
				)?;
				Ok(IntegerEnumVariant { name, value })
			})
			.collect::<Result<Vec<_>, String>>()?;

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
		let value_literals = self
			.variants
			.iter()
			.map(|variant| Literal::i64_unsuffixed(variant.value))
			.collect::<Vec<_>>();
		let value_comments = self
			.variants
			.iter()
			.map(|variant| format!(" Integer: {}", variant.value))
			.collect::<Vec<_>>();

		quote! {
			#(#[doc = #docs])*
			#[doc = ""]
			#[doc = " Unknown numeric values are retained to remain forward-compatible."]
			#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
			pub enum #name {
				#(
					#[doc = #value_comments]
					#variant_names,
				)*
				#[doc = " A value introduced by a newer API version."]
				Unknown(i64),
			}

			impl #name {
				pub const fn value(self) -> i64 {
					match self {
						#(Self::#variant_names => #value_literals,)*
						Self::Unknown(value) => value,
					}
				}
			}

			impl From<i64> for #name {
				fn from(value: i64) -> Self {
					match value {
						#(#value_literals => Self::#variant_names,)*
						value => Self::Unknown(value),
					}
				}
			}

			impl From<#name> for i64 {
				fn from(value: #name) -> Self {
					value.value()
				}
			}

			impl serde::Serialize for #name {
				fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
				where
					S: serde::Serializer,
				{
					serializer.serialize_i64((*self).value())
				}
			}

			impl<'de> serde::Deserialize<'de> for #name {
				fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
				where
					D: serde::Deserializer<'de>,
				{
					<i64 as serde::Deserialize>::deserialize(deserializer)
						.map(Self::from)
				}
			}
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn parses_and_renders_an_open_integer_enum() {
		let schema: Schema = serde_json::from_str(
			r#"{
				"type": "integer",
				"enum": [1],
				"x-enum-varnames": ["ONE"]
			}"#,
		)
		.unwrap();

		let model = IntegerEnum::parse("NumberKind", &schema).unwrap();
		let rendered = model.render().to_string();
		assert_eq!(model.name(), "NumberKind");
		assert!(rendered.contains("pub enum NumberKind"));
		assert!(rendered.contains("1 => Self :: One"));
		assert!(rendered.contains("Unknown (i64)"));
	}

	#[test]
	fn rejects_mismatched_enum_metadata() {
		let schema: Schema = serde_json::from_str(
			r#"{
				"type": "integer",
				"enum": [1, 2],
				"x-enum-varnames": ["ONE"]
			}"#,
		)
		.unwrap();

		let error = IntegerEnum::parse("Broken", &schema).unwrap_err();
		assert!(error.contains("2 values but 1 variant names"));
	}
}
