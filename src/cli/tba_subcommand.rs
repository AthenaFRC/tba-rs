use crate::cli::handlers::*;

#[derive(clap::Subcommand, Debug)]
pub enum TBASubcommand {
	#[command(
		about = "Fetches information from the specified TBA API endpoint."
	)]
	Get {
		#[clap(flatten)]
		args: CLIGetCommandArgs,
	},

	#[command(
		about = "Generates an autocompletion script for a specified shell."
	)]
	Completions {
		#[clap(flatten)]
		args: CLIPrintCompletionsCommandArgs,
	},

	#[command(about = "Attempts to install autocompletion scripts for the \
	                   current shell.")]
	InstallCompletions {
		#[clap(flatten)]
		args: CLIInstallCompletionsCommandArgs,
	},
}

impl TBASubcommand {
	pub async fn execute(self) -> Result<(), String> {
		match self {
			TBASubcommand::Get { args } => get_endpoint(args).await,
			TBASubcommand::Completions { args } => print_completions(args),
			TBASubcommand::InstallCompletions { args } => {
				install_completions(args)
			}
		}
	}
}
