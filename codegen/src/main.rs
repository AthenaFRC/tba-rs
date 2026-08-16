use std::process::ExitCode;

use clap::{CommandFactory, Parser};
use codegen::{commands::CodegenCommand, inputs::Config};

fn main() -> ExitCode {
	match execute(CodegenCommand::parse()) {
		Ok(()) => ExitCode::SUCCESS,
		Err(error) => {
			eprintln!("Error: {error}");
			ExitCode::FAILURE
		}
	}
}

fn execute(command: CodegenCommand) -> Result<(), String> {
	let config =
		Config::from_custom_config_file(&command.config)?.ok_or_else(|| {
			format!(
				"failed to find config file at {}",
				command.config.display()
			)
		})?;

	match command.subcommand {
		Some(subcommand) => subcommand.execute(&config),
		None => {
			CodegenCommand::command()
				.print_help()
				.map_err(|error| format!("failed to print help: {error}"))?;
			Err("a subcommand is required".into())
		}
	}
}
