use crate::{
	inputs::{Config, spec::OpenApiDocument},
	stages::{extract_models, format, render},
	util::fs::workspace_root,
};

pub fn generate_models(config: &Config) -> Result<String, String> {
	let workspace_root = workspace_root();
	let expected_openapi_version = config.expected_openapi_version.to_string();
	let expected_api_version = config.expected_api_version.to_string();
	let document = OpenApiDocument::read(&config.spec_file_path)?;
	document.validate_expected_versions(
		&expected_openapi_version,
		&expected_api_version,
	)?;
	let models = extract_models(&document, &config.overrides)?;
	let spec_path = config
		.spec_file_path
		.strip_prefix(&workspace_root)
		.unwrap_or(&config.spec_file_path)
		.to_string_lossy()
		.replace('\\', "/");
	let rendered = render(&models, &spec_path, &expected_api_version)?;
	format(&rendered, &workspace_root)
}

#[cfg(test)]
mod tests {
	use super::*;

	fn workspace_config() -> Config {
		Config::from_custom_config_file(&Config::get_default_config_file_path())
			.unwrap()
			.expect("the workspace codegen config should exist")
	}

	#[test]
	fn pinned_inputs_generate_expected_model_families() {
		let config = workspace_config();
		let generated = generate_models(&config).unwrap();

		assert!(generated.contains("pub struct APIStatus"));
		assert!(generated.contains("pub struct Event"));
		assert!(generated.contains("pub struct Match"));
		assert!(generated.contains("pub struct Team"));
		assert!(generated.contains("pub type EventCOPRs"));
		assert!(generated.contains("pub enum MatchScoreBreakdown"));
		assert!(generated.contains("pub enum AllianceColor"));
		assert!(generated.contains("pub enum AwardType"));
		assert!(generated.contains("pub enum EventType"));
		assert!(generated.contains("pub enum PlayoffType"));
		assert!(generated.contains("pub enum RegionalAdvancementCMPStatus"));
		assert!(generated.contains("pub enum WebcastStatus"));
		assert!(!generated.contains("pub enum Media"));
		assert!(!generated.contains("pub enum InsightV2"));
	}

	#[test]
	fn configured_api_version_is_enforced() {
		let config = Config {
			expected_api_version: crate::util::VersionNumber::new(4, 0, 0),
			..workspace_config()
		};

		let error = generate_models(&config).unwrap_err();
		assert!(error.contains("expected TBA API version 4.0.0"));
	}
}
