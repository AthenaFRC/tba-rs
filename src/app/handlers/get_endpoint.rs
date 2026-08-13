use crate::{
	APIClient,
	api,
	app::scaffolding::{
		CLIEndpoint,
		OutputFormat,
	},
};

pub async fn get_endpoint(
	endpoint: CLIEndpoint,
	api_key: Option<String>,
	base_url: Option<String>,
	e_tag: Option<String>,
	format: Option<OutputFormat>,
) -> Result<(), Box<dyn std::error::Error>> {
	let client = APIClient::new_with(api_key, base_url).await?;
	let format = format.unwrap_or(OutputFormat::JSON);

	// let result = match endpoint {
	// 	CLIEndpoint::DistrictDCMPHistory {
	// 		district_abbreviation,
	// 	} => {
	// 		api::district::district_dcmp_history(
	// 			&client,
	// 			district_abbreviation,
	// 			e_tag,
	// 		)
	// 		.await
	// 	}
	// 	CLIEndpoint::DistrictHistory {
	// 		district_abbreviation,
	// 	} => {
	// 		api::district::district_history(
	// 			&client,
	// 			district_abbreviation,
	// 			e_tag,
	// 		)
	// 		.await
	// 	}
	// 	CLIEndpoint::DistrictInsights {
	// 		district_abbreviation,
	// 	} => {
	// 		api::district::district_insights(
	// 			&client,
	// 			district_abbreviation,
	// 			e_tag,
	// 		)
	// 		.await
	// 	}
	// 	CLIEndpoint::DistrictAdvancement { district_key } => {
	// 		api::district::district_advancement(&client, district_key, e_tag)
	// 			.await
	// 	}
	// };

	Ok(())
}
