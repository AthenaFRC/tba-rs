use serde_json::{
	Serializer,
	ser::PrettyFormatter,
};

use crate::{
	API_KEY_ENV_VAR,
	APIClient,
	APIClientInitError,
	APIResult,
	BASE_API_URL_ENV_VAR,
	cli::{
		OutputFormat,
		TBAConfig,
	},
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
		long_help = &get_endpoint_base_url_help_message(true),
		help = &get_endpoint_base_url_help_message(false),
	)]
	base_url: Option<String>,

	#[arg(
		long,
		global = true,
		help = "The ETag value to send with the request."
	)]
	e_tag: Option<String>,

	#[arg(
		long,
		global = true,
		help = "The format to output the result in. [default: json]"
	)]
	output_format: Option<OutputFormat>,

	#[arg(
		long,
		global = true,
		num_args = 0..=1,
		require_equals = true,
		default_missing_value = "true",
		hide_possible_values = true,
		help = "Print the response's ETag header value."
	)]
	print_e_tag: Option<bool>,

	#[command(subcommand)]
	endpoint: GetSubcommand,
}

fn get_endpoint_base_url_help_message(long_help: bool) -> String {
	let delimiter = if long_help { "\n\n" } else { " " };
	format!(
		"The base URL to use for the TBA API.{delimiter}[default: {}]",
		crate::BASE_API_URL_DEFAULT,
	)
}

pub async fn get_endpoint(
	args: CLIGetCommandArgs,
	config: &TBAConfig,
) -> Result<(), String> {
	let command_line_config = TBAConfig {
		path: None,
		api_key: args.api_key,
		base_url: args.base_url,
		output_format: args.output_format,
		print_e_tag: args.print_e_tag,
	};
	let environment_config = TBAConfig {
		path: None,
		api_key: std::env::var(API_KEY_ENV_VAR).ok(),
		base_url: std::env::var(BASE_API_URL_ENV_VAR).ok(),
		output_format: None,
		print_e_tag: None,
	};
	let resolved_config = command_line_config.resolve(
		environment_config,
		config.clone(),
		TBAConfig::default(),
	);

	let client =
		APIClient::new_with(resolved_config.api_key, resolved_config.base_url)
			.await
			.map_err(|client_init_error| match client_init_error {
				APIClientInitError::ReqwestClientInitError(err) => {
					err.to_string()
				}
				APIClientInitError::APIKeyError(_) => format!(
					"API key must be provided via either 1) the '--api-key' \
					 flag or 2) the environment variable '{}', or 3) the \
					 config file.",
					API_KEY_ENV_VAR
				),
			})?;
	let api_result = args.endpoint.get(&client, args.e_tag).await;
	print_result(
		api_result,
		resolved_config.print_e_tag.unwrap_or(false),
		resolved_config.output_format.unwrap_or_default(),
	)
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

#[cfg(test)]
mod tests {
	use clap::{
		CommandFactory,
		Parser,
	};

	use crate::cli::{
		TBACommand,
		TBASubcommand,
	};

	#[test]
	fn absent_command_line_settings_remain_none() {
		let command = TBACommand::try_parse_from([
			"tba", "get", "team", "simple", "frc254",
		])
		.expect("command should parse");
		let Some(TBASubcommand::Get { args }) = command.subcommand else {
			panic!("expected get command");
		};

		assert!(args.api_key.is_none());
		assert!(args.base_url.is_none());
		assert!(args.output_format.is_none());
		assert!(args.print_e_tag.is_none());
	}

	#[test]
	fn base_url_help_displays_built_in_default() {
		let mut command = TBACommand::command();
		command.build();
		let get = command
			.find_subcommand_mut("get")
			.expect("get command should exist");
		let team = get
			.find_subcommand_mut("team")
			.expect("team command should exist");
		let simple = team
			.find_subcommand_mut("simple")
			.expect("simple command should exist");
		let help = simple.render_long_help().to_string();

		assert!(
			help.contains(crate::BASE_API_URL_DEFAULT),
			"unexpected help output: {help}"
		);
	}
}
