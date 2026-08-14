use clap::Parser;
use tba::app::{
	handlers::{
		generate_completions,
		get_endpoint,
		install_completions,
	},
	scaffolding::{
		CLI,
		Commands,
	},
};

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
	let cli: CLI = CLI::parse();

	match cli.command {
		Some(Commands::Get { request }) => get_endpoint(request).await,
		Some(Commands::Completions { shell }) => generate_completions(shell),
		Some(Commands::InstallCompletions) => install_completions(None),
		None => {}
	}
	
	Ok(())
}
