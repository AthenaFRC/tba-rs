use crate::commands::codegen_subcommand::CodegenSubcommand;
use crate::inputs::Config;

#[derive(clap::Parser, Debug)]
#[command(
	about,
	version,
	propagate_version = true,
	arg_required_else_help = true,
	help_expected = true,
	disable_version_flag = true,
	disable_help_flag = true,
	disable_help_subcommand = true
)]
pub struct CodegenCommand {
	#[arg(
		short,
		long,
		value_name = "FILE",
		global = true,
		default_value = Config::get_apparent_default_config_file_path(),
		help = "Sets the custom config file to use."
	)]
	pub config: std::path::PathBuf,

	#[arg(
		short,
		long,
		global = true,
		action = clap::ArgAction::Version,
		help = "Prints version information."
	)]
	pub version: Option<bool>,

	#[arg(
		short,
		long,
		global = true,
		action = clap::ArgAction::Help,
		help = "Prints help information."
	)]
	pub help: Option<bool>,

	#[command(subcommand)]
	pub subcommand: Option<CodegenSubcommand>,
}

#[cfg(test)]
mod tests {
	use clap::Parser;

	use super::*;

	#[test]
	fn parses_a_global_custom_config_path() {
		let command = CodegenCommand::try_parse_from([
			"codegen",
			"check",
			"--config",
			"custom-codegen.toml",
		])
		.unwrap();

		assert_eq!(
			command.config,
			std::path::PathBuf::from("custom-codegen.toml")
		);
		assert!(matches!(command.subcommand, Some(CodegenSubcommand::Check)));
	}
}
