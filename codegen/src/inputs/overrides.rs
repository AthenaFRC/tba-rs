use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Overrides {
	#[serde(default)]
	pub manual_models: BTreeSet<String>,
	#[serde(default)]
	pub models: BTreeMap<String, ModelOverride>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct ModelOverride {
	#[serde(default)]
	pub variants: BTreeMap<String, String>,
	#[serde(default)]
	pub types: BTreeMap<String, String>,
}

impl Overrides {
	pub fn parse(source: &str) -> Result<Self, String> {
		toml::from_str(source).map_err(|error| {
			format!("failed to parse codegen overrides: {error}")
		})
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn parses_variant_renames() {
		let overrides = Overrides::parse(
			r#"[models.AllianceColor.variants]
			NO_ALLIANCE = "Empty"
			"#,
		)
		.unwrap();

		assert_eq!(
			overrides.models["AllianceColor"].variants["NO_ALLIANCE"],
			"Empty"
		);
		assert!(overrides.manual_models.is_empty());
	}

	#[test]
	fn parses_nested_type_renames() {
		let overrides = Overrides::parse(
			r#"[models.Example.types]
			ExampleCmpStatus = "ExampleCMPStatus"
			"#,
		)
		.unwrap();

		assert_eq!(
			overrides.models["Example"].types["ExampleCmpStatus"],
			"ExampleCMPStatus"
		);
	}
}
