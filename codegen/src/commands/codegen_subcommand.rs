use crate::{
	handlers::{check_generated_models, write_generated_models},
	inputs::Config,
};

#[derive(clap::Subcommand, Debug)]
pub enum CodegenSubcommand {
	#[command(
		about = "Generates Rust models based off of the configured OpenAPI spec."
	)]
	Generate,

	#[command(
		about = "Checks whether or not existing Rust models are up-to-date."
	)]
	Check,
}

impl CodegenSubcommand {
	pub fn execute(self, config: &Config) -> Result<(), String> {
		match self {
			CodegenSubcommand::Generate => write_generated_models(config),
			CodegenSubcommand::Check => check_generated_models(config),
		}
	}
}
