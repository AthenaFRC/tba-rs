use clap::Parser;
use tba::app::CLI;

// tba

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
	let cli: CLI = CLI::parse();

	if let Some(name) = cli.command.as_ref() {
		println!("Command: {:#?}", name);
	}

	if let Some(path) = cli.config.as_deref() {
		println!("Config: {}", path.display());
	}

	// if let Err(error) = dotenvy::dotenv() {
	// 	println!("Failed to load environment variables: {}", error);
	// }
	//
	// let api: APIClient = APIClient::new().await?;
	//
	// println!(
	// 	"API Status: {:#?}",
	// 	api::team::team_media_by_year(&api, "frc1711", 2024, None).await
	// );

	Ok(())
}
