use super::cli_endpoint::CLIEndpoint;

#[derive(clap::Subcommand, Debug)]
#[command(verbatim_doc_comment)]
pub enum Commands {
	/// Fetches information from the specific TBA API endpoint.
	Get {
		/// The endpoint from which to fetch information.
		#[command(subcommand)]
		endpoint: CLIEndpoint,
	},
}
