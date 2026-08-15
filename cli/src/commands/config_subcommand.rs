use crate::{TBAConfig, handlers::*};

#[derive(clap::Args, Debug)]
pub struct ConfigSubcommandArgs {
	#[command(subcommand)]
	pub subcommand: ConfigSubcommand,
}

#[derive(clap::Subcommand, Debug)]
pub enum ConfigSubcommand {
	#[command(about = "Modify the TBA CLI config file.")]
	Set {
		#[clap(flatten)]
		args: CLIConfigSetCommandArgs,
	},

	#[command(about = "Prints the current configuration.")]
	Show {
		#[clap(flatten)]
		args: CLIConfigShowCommandArgs,
	},
}

impl ConfigSubcommand {
	pub async fn execute(self, config: &TBAConfig) -> Result<(), String> {
		match self {
			ConfigSubcommand::Set { args } => config_set(args, config),
			ConfigSubcommand::Show { args } => config_show(args, config),
		}
	}
}
