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

pub fn config_show(
	args: CLIConfigShowCommandArgs,
	config: &TBAConfig,
) -> Result<(), String> {
	let config = if args.include_defaults {
		config.clone().or(TBAConfig::default())
	} else {
		config.clone()
	};
	let config_string = toml::to_string_pretty(&config)
		.map_err(|e| format!("Failed to serialize config: {}", e))?;
	print!("{config_string}");
	Ok(())
}
