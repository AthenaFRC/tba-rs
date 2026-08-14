use clap::{
	CommandFactory,
	Parser,
};
use tba::cli::TBACommand;

#[tokio::main]
pub async fn main() {
	let tba_command: TBACommand = TBACommand::parse();

	let subcommand_result = match tba_command.subcommand {
		Some(subcommand) => subcommand.execute().await,
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
