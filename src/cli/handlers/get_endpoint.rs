use serde_json::{
	Serializer,
	ser::PrettyFormatter,
};

use crate::{
	API_KEY_ENV_VAR,
	APIClient,
	APIClientInitError,
	APIResult,
	cli::OutputFormat,
	endpoints::GetSubcommand,
};

#[derive(clap::Args, Debug, Clone)]
pub struct CLIGetCommandArgs {
	#[arg(
		long,
		global = true,
		help = "The API key to use to authenticate to the TBA API."
	)]
	api_key: Option<String>,

	#[arg(
		long,
		global = true,
		default_value = crate::BASE_API_URL_DEFAULT,
		help = "The base URL to use for the TBA API.",
	)]
	base_url: Option<String>,

	#[arg(
		long,
		global = true,
		help = "The ETag value to send with the request."
	)]
	e_tag: Option<String>,

	#[arg(
		short,
		long,
		global = true,
		default_value = "json",
		help = "The format to output the result in."
	)]
	output_format: OutputFormat,

	#[arg(
		long,
		global = true,
		default_value_t = false,
		help = "Whether to print the ETag value."
	)]
	print_e_tag: bool,

	#[command(subcommand)]
	endpoint: GetSubcommand,
}

pub async fn get_endpoint(args: CLIGetCommandArgs) -> Result<(), String> {
	let client = APIClient::new_with(args.api_key, args.base_url)
		.await
		.map_err(|client_init_error| match client_init_error {
			APIClientInitError::ReqwestClientInitError(err) => err.to_string(),
			APIClientInitError::APIKeyError(_) => format!(
				"API key must be provided either via the '--api-key' flag or \
				 in the environment variable '{}'",
				API_KEY_ENV_VAR
			),
		})?;
	let api_result = args.endpoint.get(&client, args.e_tag).await;
	print_result(api_result, args.print_e_tag, args.output_format)
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
						.map_err(|e| format!(
							"Failed to serialize JSON: {}",
							e
						))?,
					OutputFormat::JSONPrettyTabs => {
						get_pretty_json_string(&result, b"\t")?
					}
					OutputFormat::JSONPretty2Spaces => {
						get_pretty_json_string(&result, b"  ")?
					}
					OutputFormat::JSONPretty4Spaces => {
						get_pretty_json_string(&result, b"    ")?
					}
					OutputFormat::JSONL => {
						return Err(
							"JSONL output is not implemented".to_string()
						);
					}
					OutputFormat::CSV => {
						return Err("CSV output is not implemented".to_string());
					}
					OutputFormat::TSV => {
						return Err("TSV output is not implemented".to_string());
					}
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
	value
		.serialize(&mut serializer)
		.map_err(|e| format!("Failed to serialize JSON: {}", e))?;
	String::from_utf8(buffer)
		.map_err(|e| format!("Failed to convert JSON to string: {}", e))
}
