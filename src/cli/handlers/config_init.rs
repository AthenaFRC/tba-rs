use crate::cli::TBAConfig;

#[derive(clap::Args, Debug, Clone)]
pub struct CLIConfigInitCommandArgs {
	#[arg(
		short,
		long,
		default_value = TBAConfig::get_apparent_default_config_file_path(),
		help = "The path at which to generate the config file."
	)]
	output_path: std::path::PathBuf,

	#[clap(flatten)]
	config: TBAConfig,
}

pub fn config_init(args: CLIConfigInitCommandArgs) -> Result<(), String> {
	args.config.write_custom_config_file(&args.output_path)?;
	println!(
		"Initialized config file at {}.",
		&args.output_path.to_string_lossy(),
	);
	Ok(())
}
