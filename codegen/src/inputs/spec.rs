use std::{collections::BTreeMap, fs, path::Path};

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct OpenApiDocument {
	openapi: String,
	info: OpenApiInfo,
	components: Components,
}

#[derive(Debug, Deserialize)]
struct OpenApiInfo {
	version: String,
}

#[derive(Debug, Deserialize)]
struct Components {
	schemas: BTreeMap<String, Schema>,
}

#[derive(Debug, Deserialize)]
pub struct Schema {
	pub title: Option<String>,
	pub description: Option<String>,
	#[serde(rename = "type")]
	type_: Option<serde_json::Value>,
	#[serde(rename = "enum")]
	pub values: Option<Vec<serde_json::Value>>,
	#[serde(rename = "x-enum-varnames")]
	pub variant_names: Option<Vec<String>>,
}

impl OpenApiDocument {
	pub fn read(path: &Path) -> Result<Self, String> {
		let source = fs::read_to_string(path).map_err(|error| {
			format!("failed to read {}: {error}", path.display())
		})?;
		Self::parse(&source)
	}

	pub fn parse(source: &str) -> Result<Self, String> {
		serde_json::from_str(source).map_err(|error| {
			format!("failed to parse OpenAPI document: {error}")
		})
	}

	pub fn validate_expected_versions(
		&self,
		expected_openapi_version: &str,
		expected_api_version: &str,
	) -> Result<(), String> {
		if self.openapi != expected_openapi_version {
			return Err(format!(
				"expected OpenAPI version {expected_openapi_version}, found {}",
				self.openapi
			));
		}
		if self.info.version != expected_api_version {
			return Err(format!(
				"expected TBA API version {expected_api_version}, found {}",
				self.info.version
			));
		}
		Ok(())
	}

	pub fn schemas(&self) -> impl Iterator<Item = (&str, &Schema)> {
		self.components
			.schemas
			.iter()
			.map(|(name, schema)| (name.as_str(), schema))
	}
}

impl Schema {
	pub fn simple_type(&self) -> Option<&str> {
		self.type_.as_ref().and_then(serde_json::Value::as_str)
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn rejects_unexpected_api_versions() {
		let source = r#"{
			"openapi": "3.1.1",
			"info": { "version": "4.0.0" },
			"components": { "schemas": {} }
		}"#;

		let error = OpenApiDocument::parse(source)
			.unwrap()
			.validate_expected_versions("3.1.1", "3.16.0")
			.unwrap_err();
		assert!(error.contains("expected TBA API version 3.16.0"));
	}

	#[test]
	fn rejects_unexpected_openapi_versions() {
		let source = r#"{
			"openapi": "3.0.0",
			"info": { "version": "3.16.0" },
			"components": { "schemas": {} }
		}"#;

		let error = OpenApiDocument::parse(source)
			.unwrap()
			.validate_expected_versions("3.1.1", "3.16.0")
			.unwrap_err();
		assert!(error.contains("expected OpenAPI version 3.1.1"));
	}
}
