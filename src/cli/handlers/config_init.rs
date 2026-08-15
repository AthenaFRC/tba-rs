use crate::cli::TBAConfig;

#[derive(clap::Args, Debug, Clone)]
pub struct CLIConfigInitCommandArgs {
	#[arg(
		short,
		long,
		long_help = &config_init_long_help_message(),
		help = &config_init_help_message(),
	)]
	output_path: Option<std::path::PathBuf>,

	#[clap(flatten)]
	config: TBAConfig,
}

fn config_init_help_message() -> String {
	format!(
		"The path at which to generate the config file. [default: {}]",
		TBAConfig::get_apparent_default_config_file_path()
	)
}

fn config_init_long_help_message() -> String {
	format!(
		"The path at which to generate the config file.\n\n[default: {}]",
		TBAConfig::get_apparent_default_config_file_path()
	)
}

pub fn config_init(
	args: CLIConfigInitCommandArgs,
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
