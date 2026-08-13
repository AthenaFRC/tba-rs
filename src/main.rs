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
		Some(Commands::Get {
			api_key,
			base_url,
			e_tag,
			format,
			endpoint,
		}) => get_endpoint(endpoint, api_key, base_url, e_tag, format).await?,
		Some(Commands::Completions { shell }) => generate_completions(shell)?,
		Some(Commands::InstallCompletions) => install_completions(None)?,
		None => {}
	}

	Ok(())
}
