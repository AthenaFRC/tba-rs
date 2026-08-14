#[derive(clap::Args, Debug, Clone)]
pub struct CLIPrintCompletionsCommand {
	
	/// The shell for which to generate the autocompletion script.
	#[arg(value_enum)]
	pub shell: clap_complete::Shell,
	
}

pub fn print_completions(
	command: CLIPrintCompletionsCommand,
) -> Result<(), String> {
	
	use std::io::Write;
	let completions = crate::cli::generate_completions(command.shell);
	if let Err(error) = std::io::stdout().write_all(&completions) {
		eprintln!("Error: {}", error);
	}
	Ok(())
	
}