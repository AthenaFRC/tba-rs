use std::path::PathBuf;

use super::commands::Commands;

#[derive(clap::Parser, Debug)]
#[command(version, about)]
#[command(max_term_width = 120)]
#[command(verbatim_doc_comment)]
pub struct CLI {
	/// Sets custom config file
	#[arg(short, long, value_name = "FILE")]
	pub config: Option<PathBuf>,

	#[command(subcommand)]
	pub command: Option<Commands>,
}
