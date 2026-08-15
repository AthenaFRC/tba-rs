use crate::cli::TBAConfig;

#[derive(clap::Args, Debug, Clone)]
pub struct CLIConfigShowCommandArgs {
	#[arg(
		long,
		num_args = 0..=1,
		require_equals = true,
		default_missing_value = "true",
		help = "Whether to include default values in the output."
	)]
	include_defaults: bool,
}

pub fn config_show(args: CLIConfigShowCommandArgs) -> Result<(), String> {
	let config_file_path = TBAConfig::get_default_config_file_path()
		.map_err(|e| format!("Failed to get default config file path: {}", e))?;
	match TBAConfig::from_custom_config_file(&config_file_path) {
		Ok(Some(config)) => {
			let config_string = toml::to_string_pretty(&config)
				.map_err(|e| format!("Failed to serialize config: {}", e))?;
			println!("{config_string}");
			Ok(())
		}
		Ok(None) => Err(format!(
			"Failed to find config file at {}",
			config_file_path.to_string_lossy().to_string(),
		)),
		Err(e) => Err(format!("Failed to read config file: {e}")),
	}
}
