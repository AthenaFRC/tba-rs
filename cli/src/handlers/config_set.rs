use crate::TBAConfig;

#[derive(clap::Args, Debug, Clone)]
pub struct CLIConfigSetCommandArgs {
	#[arg(
		short,
		long,
		long_help = &config_set_output_path_help_message(true),
		help = &config_set_output_path_help_message(false),
	)]
	output_path: Option<std::path::PathBuf>,

	#[clap(flatten)]
	config: TBAConfig,
}

fn config_set_output_path_help_message(long_help: bool) -> String {
	let delimiter = if long_help { "\n\n" } else { " " };
	format!(
		"The path to the config file to use.{delimiter}[default: {}]",
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
		config_file_path.to_string_lossy(),
	);
	Ok(())
}
