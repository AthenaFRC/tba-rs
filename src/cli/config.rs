use std::path::{
	Path,
	PathBuf,
};

const CONFIG_FILE_NAME: &str = ".tbarc";

#[derive(
	clap::Args, serde::Serialize, serde::Deserialize, Default, Debug, Clone,
)]
pub struct TBAConfig {
	#[arg(
		long,
		global = true,
		help = "The API key to use to authenticate to the TBA API."
	)]
	api_key: Option<String>,

	#[arg(long, global = true, help = "The base URL to use for the TBA API.")]
	base_url: Option<String>,

	#[arg(
		long,
		global = true,
		help = "The format to output results in."
	)]
	output_format: Option<crate::cli::OutputFormat>,

	#[arg(
		long,
		global = true,
		num_args = 0..=1,
		require_equals = true,
		default_missing_value = "true",
		help = "Whether to print ETag values when fetching data from the API."
	)]
	print_e_tag: Option<bool>,
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

	pub fn from_custom_config_file(
		config_file_path: &Path,
	) -> Result<Option<TBAConfig>, String> {
		if !config_file_path.exists() {
			return Ok(None);
		}
		let config_file_content = std::fs::read_to_string(&config_file_path)
			.map_err(|e| format!("Failed to read config file: {}", e))?;
		match toml::from_str(&config_file_content) {
			Ok(config) => Ok(Some(config)),
			Err(e) => Err(format!("Failed to parse config file: {}", e)),
		}
	}

	pub fn from_config_file() -> Result<Option<TBAConfig>, String> {
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
}
