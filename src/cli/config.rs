use std::path::{
	Path,
	PathBuf,
};

const CONFIG_FILE_NAME: &str = ".tbarc";

#[derive(clap::Args, serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct TBAConfig {
	/// The path that this config file originated from, if any.
	#[arg(skip)]
	#[serde(skip)]
	pub path: Option<PathBuf>,

	#[arg(
		long,
		global = true,
		help = "The API key to use to authenticate to the TBA API."
	)]
	pub api_key: Option<String>,

	#[arg(long, global = true, help = "The base URL to use for the TBA API.")]
	pub base_url: Option<String>,

	#[arg(long, global = true, help = "The format to output results in.")]
	pub output_format: Option<crate::cli::OutputFormat>,

	#[arg(
		long,
		global = true,
		num_args = 0..=1,
		require_equals = true,
		default_missing_value = "true",
		hide_possible_values = true,
		help = "Whether to print ETag values when fetching data from the API."
	)]
	pub print_e_tag: Option<bool>,
}

impl TBAConfig {
	pub fn get_default_config_file_path() -> Result<PathBuf, String> {
		Ok(crate::cli::fs_util::home_dir()?.join(CONFIG_FILE_NAME))
	}

	pub fn get_apparent_default_config_file_path() -> String {
		crate::cli::fs_util::home_dir()
			.unwrap_or_else(|_| PathBuf::from("."))
			.join(CONFIG_FILE_NAME)
			.to_string_lossy()
			.to_string()
	}

	pub fn empty() -> Self {
		Self {
			path: None,
			api_key: None,
			base_url: None,
			output_format: None,
			print_e_tag: None,
		}
	}

	pub fn from_custom_config_file(
		config_file_path: &Path,
	) -> Result<Option<Self>, String> {
		if !config_file_path.exists() {
			return Ok(None);
		}
		let config_file_content = std::fs::read_to_string(config_file_path)
			.map_err(|e| format!("Failed to read config file: {}", e))?;
		let mut config: Self = match toml::from_str(&config_file_content) {
			Ok(config) => Ok(config),
			Err(e) => Err(format!("Failed to parse config file: {}", e)),
		}?;
		config.path = Some(config_file_path.to_path_buf());
		Ok(Some(config))
	}

	pub fn from_config_file() -> Result<Option<Self>, String> {
		Self::from_custom_config_file(&Self::get_default_config_file_path()?)
	}

	pub fn write_custom_config_file(
		&self,
		config_file_path: &Path,
	) -> Result<(), String> {
		let config_file_content = toml::to_string_pretty(&self)
			.map_err(|e| format!("Failed to serialize config: {}", e))?;
		std::fs::write(config_file_path, config_file_content)
			.map_err(|e| format!("Failed to write config file: {}", e))?;
		Ok(())
	}

	pub fn write_config_file(&self) -> Result<(), String> {
		self.write_custom_config_file(&Self::get_default_config_file_path()?)
	}

	/// Resolves settings with command-line values (`self`) taking precedence,
	/// followed by environment, config-file, and built-in values.
	pub fn resolve(
		self,
		environment: Self,
		config_file: Self,
		built_in: Self,
	) -> Self {
		self.or(environment).or(config_file).or(built_in)
	}

	pub fn or(self, other: Self) -> Self {
		Self {
			path: None,
			api_key: self.api_key.or(other.api_key),
			base_url: self.base_url.or(other.base_url),
			output_format: self.output_format.or(other.output_format),
			print_e_tag: self.print_e_tag.or(other.print_e_tag),
		}
	}

	pub fn is_from_default_config_file(&self) -> bool {
		match &self.path {
			Some(path) => {
				path.to_string_lossy()
					== Self::get_apparent_default_config_file_path()
			}
			None => false,
		}
	}
}

impl Default for TBAConfig {
	fn default() -> Self {
		Self {
			path: None,
			api_key: None,
			base_url: Some(crate::BASE_API_URL_DEFAULT.to_string()),
			output_format: Some(crate::cli::OutputFormat::default()),
			print_e_tag: Some(false),
		}
	}
}

#[cfg(test)]
mod tests {
	use super::TBAConfig;
	use crate::cli::OutputFormat;

	fn config_with_api_key(api_key: Option<&str>) -> TBAConfig {
		let mut config = TBAConfig::empty();
		config.api_key = api_key.map(str::to_owned);
		config
	}

	fn resolve_api_key(
		command_line: Option<&str>,
		environment: Option<&str>,
		config_file: Option<&str>,
		built_in: Option<&str>,
	) -> Option<String> {
		config_with_api_key(command_line)
			.resolve(
				config_with_api_key(environment),
				config_with_api_key(config_file),
				config_with_api_key(built_in),
			)
			.api_key
	}

	#[test]
	fn command_line_source_is_used() {
		assert_eq!(
			resolve_api_key(Some("cli"), None, None, None).as_deref(),
			Some("cli")
		);
	}

	#[test]
	fn environment_source_is_used() {
		assert_eq!(
			resolve_api_key(None, Some("environment"), None, None).as_deref(),
			Some("environment")
		);
	}

	#[test]
	fn config_file_source_is_used() {
		assert_eq!(
			resolve_api_key(None, None, Some("config"), None).as_deref(),
			Some("config")
		);
	}

	#[test]
	fn built_in_source_is_used() {
		assert_eq!(
			resolve_api_key(None, None, None, Some("built-in")).as_deref(),
			Some("built-in")
		);
	}

	#[test]
	fn command_line_overrides_environment() {
		assert_eq!(
			resolve_api_key(Some("cli"), Some("environment"), None, None)
				.as_deref(),
			Some("cli")
		);
	}

	#[test]
	fn command_line_overrides_config_file() {
		assert_eq!(
			resolve_api_key(Some("cli"), None, Some("config"), None).as_deref(),
			Some("cli")
		);
	}

	#[test]
	fn command_line_overrides_built_in() {
		assert_eq!(
			resolve_api_key(Some("cli"), None, None, Some("built-in"))
				.as_deref(),
			Some("cli")
		);
	}

	#[test]
	fn environment_overrides_config_file() {
		assert_eq!(
			resolve_api_key(None, Some("environment"), Some("config"), None)
				.as_deref(),
			Some("environment")
		);
	}

	#[test]
	fn environment_overrides_built_in() {
		assert_eq!(
			resolve_api_key(None, Some("environment"), None, Some("built-in"))
				.as_deref(),
			Some("environment")
		);
	}

	#[test]
	fn config_file_overrides_built_in() {
		assert_eq!(
			resolve_api_key(None, None, Some("config"), Some("built-in"))
				.as_deref(),
			Some("config")
		);
	}

	#[test]
	fn resolution_applies_to_every_setting() {
		let command_line = TBAConfig {
			api_key: Some("cli".to_string()),
			..TBAConfig::empty()
		};
		let environment = TBAConfig {
			base_url: Some("environment".to_string()),
			..TBAConfig::empty()
		};
		let config_file = TBAConfig {
			output_format: Some(OutputFormat::JSONPrettyTabs),
			..TBAConfig::empty()
		};
		let built_in = TBAConfig {
			print_e_tag: Some(false),
			..TBAConfig::empty()
		};

		let resolved = command_line.resolve(environment, config_file, built_in);

		assert_eq!(resolved.api_key.as_deref(), Some("cli"));
		assert_eq!(resolved.base_url.as_deref(), Some("environment"));
		assert!(matches!(
			resolved.output_format,
			Some(OutputFormat::JSONPrettyTabs)
		));
		assert_eq!(resolved.print_e_tag, Some(false));
	}
}
