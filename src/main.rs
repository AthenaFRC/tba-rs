use clap::{
	CommandFactory,
	Parser,
};
use tba::cli::{
	TBACommand,
	TBAConfig,
};

#[tokio::main]
pub async fn main() {
	let tba_command: TBACommand = TBACommand::parse();
	let config = match TBAConfig::from_custom_config_file(&tba_command.config) {
		Ok(Some(config)) => config,
		Ok(None) => {
			if tba_command.config.to_string_lossy()
				== TBAConfig::get_apparent_default_config_file_path()
			{
				let mut result = TBAConfig::empty();
				result.path = Some(tba_command.config.clone());
				result
			} else {
				eprintln!(
					"Failed to find config file at {}",
					tba_command.config.to_string_lossy()
				);
				std::process::exit(1);
			}
		}
		Err(e) => {
			eprintln!("Failed to read config file: {e}");
			std::process::exit(1);
		}
	};
	let subcommand_result = match tba_command.subcommand {
		Some(subcommand) => subcommand.execute(&config).await,
		None => {
			TBACommand::command()
				.print_help()
				.expect("Failed to print help message.");
			std::process::exit(1);
		}
	};

	if let Err(error) = subcommand_result {
		eprintln!("Error: {}", error);
		std::process::exit(1);
	}
}
