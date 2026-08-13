use serde_json::ser::PrettyFormatter;
use serde_json::Serializer;
use crate::{app::scaffolding::{
	CLIEndpoint,
	OutputFormat,
}, APIClient, APIResult, api};

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

fn print_result<T: serde::Serialize>(
	result: APIResult<T>,
	include_e_tag: bool,
	format: OutputFormat,
) -> Result<(), Box<dyn std::error::Error>> {
	match result {
		APIResult::NotModified => println!("Not modified. (ETag matched)"),
		APIResult::Unauthorized => eprintln!("Authorization failed. Ensure your API key is valid."),
		APIResult::Err(message) => eprintln!("Error: {}", message),
		APIResult::Ok { result, e_tag } => {
			println!("{}", match format {
				OutputFormat::JSON => serde_json::to_string(&result)?,
				OutputFormat::JSONPrettyTabs => get_pretty_json_string(&result, b"\t")?,
				OutputFormat::JSONPretty2Spaces => get_pretty_json_string(&result, b"  ")?,
				OutputFormat::JSONPretty4Spaces => get_pretty_json_string(&result, b"    ")?,
				OutputFormat::JSONL => "".to_string(), // TODO: Implement JSONL output
				OutputFormat::CSV => "".to_string(), // TODO: Implement CSV output
				OutputFormat::TSV => "".to_string(), // TODO: Implement TSV output
			});
			if include_e_tag { println!("ETag: {}", e_tag); }
		},
	};
	Ok(())
}

fn get_pretty_json_string<T: serde::Serialize>(
	value: &T,
	indentation: &[u8],
) -> Result<String, Box<dyn std::error::Error>> {
	let formatter = PrettyFormatter::with_indent(indentation);
	let mut buffer = Vec::new();
	let mut serializer = Serializer::with_formatter(&mut buffer, formatter);
	value.serialize(&mut serializer)?;
	Ok(String::from_utf8(buffer)?)
}
