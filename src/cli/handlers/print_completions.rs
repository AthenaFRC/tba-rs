#[derive(clap::Args, Debug, Clone)]
pub struct CLIPrintCompletionsCommand {
	/// The shell for which to generate the autocompletion script.
	#[arg(value_enum)]
	pub shell: Option<clap_complete::Shell>,
}

pub fn print_completions(
	command: CLIPrintCompletionsCommand,
) -> Result<(), String> {
	use std::io::Write;
	let shell = crate::cli::determine_shell(command.shell)?;
	let completions = crate::cli::generate_completions(shell);
	match std::io::stdout().write_all(&completions) {
		Ok(_) => Ok(()),
		Err(e) => Err(format!("Failed to print completions to stdout: {e}")),
	}
}
