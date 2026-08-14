use clap::CommandFactory;

pub fn generate_completions(shell: clap_complete::Shell) -> Vec<u8> {
	let mut cmd = crate::app::scaffolding::CLI::command();
	let name = cmd.get_name().to_string();
	let mut buffer = Vec::new();

	clap_complete::generate(shell, &mut cmd, name, &mut buffer);

	buffer
}
