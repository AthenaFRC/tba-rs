use crate::cli::TBAConfig;

#[derive(clap::Args, Debug, Clone)]
pub struct CLIConfigSetCommandArgs {
	#[arg(
		short,
		long,
		long_help = &config_set_output_path_long_help_message(),
		help = &config_set_output_path_help_message(),
	)]
	output_path: Option<std::path::PathBuf>,

	#[clap(flatten)]
	config: TBAConfig,
}

const OUTPUT_PATH_HELP_MESSAGE: &str = "The path to the config file to use.";

fn config_set_output_path_help_message() -> String {
	format!(
		"{OUTPUT_PATH_HELP_MESSAGE} [default: {}]",
		TBAConfig::get_apparent_default_config_file_path()
	)
}

fn config_set_output_path_long_help_message() -> String {
	format!(
		"{OUTPUT_PATH_HELP_MESSAGE}\n\n[default: {}]",
		TBAConfig::get_apparent_default_config_file_path()
	)
}

pub fn config_set(
	args: CLIConfigSetCommandArgs,
	config: &TBAConfig,
) -> Result<(), String> {
	let config_file_path = args
		.output_path
		.or_else(|| config.path.clone())
		.unwrap_or(TBAConfig::get_default_config_file_path()?);
	args.config
		.or(config.clone())
		.write_custom_config_file(&config_file_path)?;
	println!(
		"Initialized config file at {}.",
		config_file_path.to_string_lossy().to_string(),
	);
	Ok(())
}
