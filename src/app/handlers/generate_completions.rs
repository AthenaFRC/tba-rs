use std::io::Write;

use crate::app::scaffolding::get_completions;

pub fn generate_completions(
	shell: clap_complete::Shell,
) -> Result<(), Box<dyn std::error::Error>> {
	let completions = get_completions(shell);

	std::io::stdout().write_all(&completions)?;

	Ok(())
}
