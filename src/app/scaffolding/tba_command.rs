use std::path::PathBuf;

use crate::app::scaffolding::TBASubcommand;

#[derive(clap::Parser, Debug)]
#[command(version, about)]
#[command(max_term_width = 120)]
#[command(verbatim_doc_comment)]
#[command(arg_required_else_help = true)]
pub struct TBACommand {
	/// Sets custom config file
	#[arg(short, long, value_name = "FILE")]
	pub config: Option<PathBuf>,

	#[command(subcommand)]
	pub command: Option<TBASubcommand>,
}
