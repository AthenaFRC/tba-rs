use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::{inputs::Overrides, util::VersionNumber, util::fs::workspace_root};

const CONFIG_FILE_NAME: &str = "codegen.toml";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Config {
	pub spec_file_path: PathBuf,
	pub expected_openapi_version: VersionNumber,
	pub expected_api_version: VersionNumber,
	pub generated_models_file_path: PathBuf,
	#[serde(default)]
	pub overrides: Overrides,
}

impl Config {
	pub fn get_default_config_file_path() -> PathBuf {
		workspace_root().join(CONFIG_FILE_NAME)
	}

	pub fn get_apparent_default_config_file_path() -> String {
		Self::get_default_config_file_path()
			.to_string_lossy()
			.into_owned()
	}

	pub fn from_custom_config_file(
		config_file_path: &Path,
	) -> Result<Option<Self>, String> {
		if !config_file_path.exists() {
			return Ok(None);
		}
		let source =
			std::fs::read_to_string(config_file_path).map_err(|error| {
				format!(
					"failed to read config file {}: {error}",
					config_file_path.display()
				)
			})?;
		Self::parse(&source, config_file_path).map(Some)
	}

	fn parse(source: &str, config_file_path: &Path) -> Result<Self, String> {
		let mut config: Self = toml::from_str(source).map_err(|error| {
			format!(
				"failed to parse config file {}: {error}",
				config_file_path.display()
			)
		})?;
		if config.spec_file_path.is_relative() {
			let parent =
				config_file_path.parent().unwrap_or_else(|| Path::new("."));
			config.spec_file_path = parent.join(config.spec_file_path);
		}
		if config.generated_models_file_path.is_relative() {
			let parent =
				config_file_path.parent().unwrap_or_else(|| Path::new("."));
			config.generated_models_file_path =
				parent.join(config.generated_models_file_path);
		}
		Ok(config)
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn parses_config_and_resolves_relative_spec_paths() {
		let config = Config::parse(
			r#"
				spec_file_path = "openapi/spec.json"
				expected_openapi_version = "3.1.1"
				expected_api_version = "3.16.0"
				generated_models_file_path = "lib/src/models/generated.rs"

				[overrides.models.AllianceColor.variants]
				NO_ALLIANCE = "Empty"
			"#,
			Path::new("/workspace/codegen.toml"),
		)
		.unwrap();

		assert_eq!(
			config.spec_file_path,
			Path::new("/workspace/openapi/spec.json")
		);
		assert_eq!(
			config.expected_openapi_version,
			VersionNumber::new(3, 1, 1)
		);
		assert_eq!(config.expected_api_version, VersionNumber::new(3, 16, 0));
		assert_eq!(
			config.generated_models_file_path,
			Path::new("/workspace/lib/src/models/generated.rs")
		);
		assert_eq!(
			config.overrides.models["AllianceColor"].variants["NO_ALLIANCE"],
			"Empty"
		);
	}
}
