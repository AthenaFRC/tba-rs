use std::path::PathBuf;

use crate::app::scaffolding::TBASubcommand;

#[derive(clap::Parser, Debug)]
#[command(
	version,
	about,
	max_term_width = 120,
	arg_required_else_help = true,
	help_expected = true
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

	#[command(subcommand)]
	pub command: Option<TBASubcommand>,
}
