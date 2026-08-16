mod integer_enum;
mod string_enum;

use std::collections::BTreeSet;

pub use integer_enum::IntegerEnum;
pub use string_enum::StringEnum;

#[derive(Debug, PartialEq, Eq)]
pub enum Model {
	IntegerEnum(IntegerEnum),
	StringEnum(StringEnum),
}

impl Model {
	pub fn name(&self) -> &str {
		match self {
			Self::IntegerEnum(model) => model.name(),
			Self::StringEnum(model) => model.name(),
		}
	}

	pub fn render(&self) -> proc_macro2::TokenStream {
		match self {
			Self::IntegerEnum(model) => model.render(),
			Self::StringEnum(model) => model.render(),
		}
	}
}

fn validate_lengths(
	schema_name: &str,
	values: &[serde_json::Value],
	variant_names: &[String],
) -> Result<(), String> {
	if values.len() != variant_names.len() {
		return Err(format!(
			"enum `{schema_name}` has {} values but {} variant names",
			values.len(),
			variant_names.len()
		));
	}
	Ok(())
}

fn validate_unique_variant<T>(
	schema_name: &str,
	name: &str,
	value: &T,
	seen_names: &mut BTreeSet<String>,
	seen_values: &mut BTreeSet<T>,
) -> Result<(), String>
where
	T: Clone + Ord + std::fmt::Display,
{
	if !seen_names.insert(name.to_owned()) {
		return Err(format!(
			"enum `{schema_name}` has duplicate Rust variant `{name}`"
		));
	}
	if !seen_values.insert(value.clone()) {
		return Err(format!(
			"enum `{schema_name}` has duplicate value `{value}`"
		));
	}
	Ok(())
}

fn doc_lines(description: &Option<String>) -> Vec<String> {
	description
		.iter()
		.flat_map(|description| description.lines())
		.map(|line| format!(" {}", line.trim()))
		.collect()
}
