use crate::app::handlers::CLIGetRequest;

#[derive(clap::Subcommand, Debug)]
#[command(verbatim_doc_comment)]
pub enum TBASubcommand {
	/// Fetches information from the specified TBA API endpoint.
	Get {
		#[clap(flatten)]
		request: CLIGetRequest,
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
