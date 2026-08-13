fn main() -> Result<(), Box<dyn std::error::Error>> {
	#[cfg(feature = "cli")]
	generate_completions()?;

	Ok(())
}

#[cfg(feature = "cli")]
use std::env;

#[cfg(feature = "cli")]
use clap::CommandFactory;
#[cfg(feature = "cli")]
use clap_complete::generate_to;
#[cfg(feature = "cli")]
#[path = "src/app/cli.rs"]
mod cli;
#[cfg(feature = "cli")]
#[path = "src/app/cli_endpoint.rs"]
mod cli_endpoint;
#[cfg(feature = "cli")]
#[path = "src/app/commands.rs"]
mod commands;

#[cfg(feature = "cli")]
fn generate_completions() -> Result<(), Box<dyn std::error::Error>> {
	let outdir = match env::var_os("OUT_DIR") {
		None => return Ok(()),
		Some(outdir) => outdir,
	};

	println!("cargo:rerun-if-changed=src/app/cli");
	println!("cargo:rerun-if-changed=src/app/commands.rs");
	println!("cargo:rerun-if-changed=src/app/cli_endpoint.rs");

	let outdir = std::path::PathBuf::from(outdir).join("completions");
	std::fs::create_dir_all(&outdir)?;
	let bin_name = "tba";
	let mut paths = Vec::new();

	for generator in [
		clap_complete::Shell::Bash,
		clap_complete::Shell::Elvish,
		clap_complete::Shell::Fish,
		clap_complete::Shell::PowerShell,
		clap_complete::Shell::Zsh,
	] {
		let mut command = cli::CLI::command();
		paths.push(generate_to(generator, &mut command, bin_name, &outdir)?);
	}

	for path in paths {
		println!(
			"cargo:warning=completion file generated: {}",
			path.display()
		);
	}

	Ok(())
}
