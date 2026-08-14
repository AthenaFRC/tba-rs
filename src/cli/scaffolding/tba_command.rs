use std::path::PathBuf;

use crate::cli::scaffolding::TBASubcommand;

#[derive(clap::Parser, Debug)]
#[command(
	about,
	version,
	propagate_version = true,
	max_term_width = 120,
	arg_required_else_help = true,
	help_expected = true,
	disable_version_flag = true,
	disable_help_flag = true,
	disable_help_subcommand = true
)]
pub struct TBACommand {
	#[arg(
		short,
		long,
		value_name = "FILE",
		global = true,
		help = "Sets the custom config file to use."
	)]
	pub config: Option<PathBuf>,

	#[arg(
		short,
		long,
		global = true,
		action = clap::ArgAction::Version,
		help = "Prints version information."
	)]
	version: Option<bool>,

	#[arg(
		short,
		long,
		global = true,
		action = clap::ArgAction::Help,
		help = "Prints help information."
	)]
	help: Option<bool>,

	#[command(subcommand)]
	pub command: Option<TBASubcommand>,
}
