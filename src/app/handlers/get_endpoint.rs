use serde_json::{
	Serializer,
	ser::PrettyFormatter,
};

use crate::{
	app::scaffolding::{
		OutputFormat,
	},
	endpoints::GetSubcommand,
	APIClient,
	APIClientInitializationError,
	APIResult,
	API_KEY_ENV_VAR,
};

#[derive(clap::Args, Debug, Clone)]
pub struct CLIGetRequest {
	/// The API key to use to authenticate to the TBA API.
	#[arg(long, global = true)]
	api_key: Option<String>,
	
	/// The base URL to use for the TBA API.
	#[arg(long, global = true)]
	base_url: Option<String>,
	
	/// The ETag value to send with the request.
	#[arg(long, global = true)]
	e_tag: Option<String>,
	
	/// The format to output the result in.
	#[arg(short, long, global = true, default_value = "json")]
	format: OutputFormat,
	
	/// The endpoint from which to fetch information.
	#[command(subcommand)]
	endpoint: GetSubcommand,
}

pub async fn get_endpoint(request: CLIGetRequest) {
	let format = request.format;
	let client = match APIClient::new_with(request.api_key, request.base_url).await {
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
	let api_result = request.endpoint.get(&client, request.e_tag).await;
	print_result(api_result, true, format).unwrap_or_else(|err| {
		eprintln!("Error: {}", err);
	});
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
