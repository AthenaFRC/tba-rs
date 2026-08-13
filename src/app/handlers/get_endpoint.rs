use serde_json::{
	Serializer,
	ser::PrettyFormatter,
};

use crate::{app::scaffolding::{
	CLIEndpoint,
	OutputFormat,
}, APIClient, APIClientInitializationError, APIResult, API_KEY_ENV_VAR};

pub async fn get_endpoint(
	endpoint: CLIEndpoint,
	api_key: Option<String>,
	base_url: Option<String>,
	e_tag: Option<String>,
	format: Option<OutputFormat>,
) {
	let format = format.unwrap_or(OutputFormat::JSON);
	let client = match APIClient::new_with(api_key, base_url).await {
		Ok(client) => client,
		Err(APIClientInitializationError::ReqwestClientInitializationError(err)) => {
			eprintln!("Error: Failed to initialize Reqwest client: {}", err);
			return;
		},
		Err(APIClientInitializationError::APIKeyError(_)) => {
			eprintln!("Error: API key must be provided either via the '--api-key' flag or in \
					 the environment variable '{}'", API_KEY_ENV_VAR);
			return;
		},
	};
	
	if let Err(response) = endpoint.get(&client, e_tag, format).await {
		eprintln!("Error: {}", response);
	}
}

pub(crate) fn print_result<T: serde::Serialize>(
	result: APIResult<T>,
	include_e_tag: bool,
	format: OutputFormat,
) -> Result<(), String> {
	match result {
		APIResult::NotModified => println!("Not modified. (ETag matched)"),
		APIResult::Unauthorized => {
			eprintln!("Authorization failed. Ensure your API key is valid.")
		}
		APIResult::Err(message) => eprintln!("Error: {}", message),
		APIResult::Ok { result, e_tag } => {
			println!(
				"{}",
				match format {
					OutputFormat::JSON => serde_json::to_string(&result)
						.map_err(|e| format!("Failed to serialize JSON: {}", e))?,
					OutputFormat::JSONPrettyTabs => {
						get_pretty_json_string(&result, b"\t")?
					},
					OutputFormat::JSONPretty2Spaces => {
						get_pretty_json_string(&result, b"  ")?
					},
					OutputFormat::JSONPretty4Spaces => {
						get_pretty_json_string(&result, b"    ")?
					},
					OutputFormat::JSONL => {
						return Err("JSONL output is not implemented".to_string());
					},
					OutputFormat::CSV => {
						return Err("CSV output is not implemented".to_string());
					},
					OutputFormat::TSV => {
						return Err("TSV output is not implemented".to_string());
					},
				}
			);
			if include_e_tag {
				println!("ETag: {}", e_tag);
			}
		}
	};
	Ok(())
}

fn get_pretty_json_string<T: serde::Serialize>(
	value: &T,
	indentation: &[u8],
) -> Result<String, String> {
	let formatter = PrettyFormatter::with_indent(indentation);
	let mut buffer = Vec::new();
	let mut serializer = Serializer::with_formatter(&mut buffer, formatter);
	value.serialize(&mut serializer)
		.map_err(|e| format!("Failed to serialize JSON: {}", e))?;
	Ok(String::from_utf8(buffer)
		.map_err(|e| format!("Failed to convert JSON to string: {}", e))?)
}
