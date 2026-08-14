use std::io::Write;

use crate::app::scaffolding::get_completions;

pub fn generate_completions(shell: clap_complete::Shell) {
	let completions = get_completions(shell);

	if let Err(error) = std::io::stdout().write_all(&completions) {
		eprintln!("Error: {}", error);
	}
}
