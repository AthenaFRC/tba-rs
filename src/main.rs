use tba_rs::{
	APIClient,
	api,
};

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
	if let Err(error) = dotenvy::dotenv() {
		println!("Failed to load environment variables: {}", error);
	}

	let api: APIClient = APIClient::new().await?;

	println!(
		"API Status: {:#?}",
		api::team::team_media_by_year(&api, "frc1711", 2024, None).await
	);

	Ok(())
}
