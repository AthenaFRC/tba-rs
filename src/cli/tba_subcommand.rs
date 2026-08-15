use crate::cli::handlers::*;

#[derive(clap::Subcommand, Debug)]
pub enum TBASubcommand {
	#[command(
		about = "Fetches information from the specified TBA API endpoint."
	)]
	Get {
		#[clap(flatten)]
		command: CLIGetCommand,
	},

	#[command(
		about = "Generates an autocompletion script for a specified shell."
	)]
	Completions {
		#[clap(flatten)]
		command: CLIPrintCompletionsCommand,
	},

	#[command(about = "Attempts to install autocompletion scripts for the \
	                   current shell.")]
	InstallCompletions {
		#[clap(flatten)]
		command: CLIInstallCompletionsCommand,
	},
}

impl TBASubcommand {
	pub async fn execute(self) -> Result<(), String> {
		match self {
			TBASubcommand::Get { command } => get_endpoint(command).await,
			TBASubcommand::Completions { command } => {
				print_completions(command)
			}
			TBASubcommand::InstallCompletions { command } => {
				install_completions(command)
			}
		}
	}
}
