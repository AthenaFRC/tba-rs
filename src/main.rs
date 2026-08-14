use std::io::Write;

use clap::Parser;
use tba::app::{
	handlers::{
		get_endpoint,
		install_completions,
	},
	scaffolding::{
		TBACommand,
		TBASubcommand,
		generate_completions,
	},
};

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
	let cli: TBACommand = TBACommand::parse();

	match cli.command {
		Some(TBASubcommand::Get { request }) => get_endpoint(request).await,
		Some(TBASubcommand::Completions { shell }) => {
			let completions = generate_completions(shell);
			if let Err(error) = std::io::stdout().write_all(&completions) {
				eprintln!("Error: {}", error);
			}
		}
		Some(TBASubcommand::InstallCompletions) => install_completions(None),
		None => {}
	}

	Ok(())
}
