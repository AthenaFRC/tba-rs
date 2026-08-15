#[derive(clap::Args, Debug, Clone)]
pub struct CLIPrintCompletionsCommandArgs {
	/// The shell for which to generate the autocompletion script.
	#[arg(value_enum)]
	pub shell: Option<clap_complete::Shell>,
}

pub fn print_completions(
	args: CLIPrintCompletionsCommandArgs,
) -> Result<(), String> {
	use std::io::Write;
	let shell = crate::cli::util::determine_shell(args.shell)?;
	let completions = crate::cli::util::generate_completions(shell);
	match std::io::stdout().write_all(&completions) {
		Ok(_) => Ok(()),
		Err(e) => Err(format!("Failed to print completions to stdout: {e}")),
	}
}
