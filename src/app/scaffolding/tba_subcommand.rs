use crate::app::handlers::CLIGetRequest;

#[derive(clap::Subcommand, Debug)]
pub enum TBASubcommand {
	#[command(about = "Fetches information from the specified TBA API endpoint.")]
	Get {
		#[clap(flatten)]
		request: CLIGetRequest,
	},

	#[command(about = "Generates an autocompletion script for a specified shell.")]
	Completions {
		/// The shell for which to generate the autocompletion script.
		#[arg(value_enum)]
		shell: clap_complete::Shell,
	},

	#[command(about = "Attempts to install autocompletion scripts for the current shell.")]
	InstallCompletions,
}
