use crate::cli::{
	TBAConfig,
	handlers::*,
};

#[derive(clap::Args, Debug)]
pub struct ConfigSubcommandArgs {
	#[command(subcommand)]
	pub subcommand: ConfigSubcommand,
}

#[derive(clap::Subcommand, Debug)]
pub enum ConfigSubcommand {
	#[command(about = "Initializes the TBA CLI config file.")]
	Init {
		#[clap(flatten)]
		args: CLIConfigInitCommandArgs,
	},
	
	#[command(about = "Prints the current configuration.")]
	Show {
		#[clap(flatten)]
		args: CLIConfigShowCommandArgs,
	}
}

impl ConfigSubcommand {
	pub async fn execute(self, config: &TBAConfig) -> Result<(), String> {
		match self {
			ConfigSubcommand::Init { args } => config_init(args, config),
			ConfigSubcommand::Show { args } => config_show(args, config),
		}
	}
}
