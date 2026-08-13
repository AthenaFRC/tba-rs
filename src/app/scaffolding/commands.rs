use super::cli_endpoint::CLIEndpoint;
use crate::app::scaffolding::output_format::OutputFormat;

#[derive(clap::Subcommand, Debug)]
#[command(verbatim_doc_comment)]
pub enum Commands {
	/// Fetches information from the specified TBA API endpoint.
	Get {
		/// The API key to use to authenticate to the TBA API.
		#[arg(long)]
		api_key: Option<String>,

		/// The base URL to use for the TBA API.
		#[arg(long)]
		base_url: Option<String>,

		/// The ETag value to send with the request.
		#[arg(long)]
		e_tag: Option<String>,

		/// The format to output the result in.
		#[arg(short, long)]
		format: Option<OutputFormat>,

		/// The endpoint from which to fetch information.
		#[command(subcommand)]
		endpoint: CLIEndpoint,
	},

	/// Generates an autocompletion script for a specified shell.
	Completions {
		/// The shell for which to generate the autocompletion script.
		#[arg(value_enum)]
		shell: clap_complete::Shell,
	},

	/// Attempts to install autocompletion scripts for the current shell.
	InstallCompletions,
}
