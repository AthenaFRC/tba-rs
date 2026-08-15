use std::{
	env,
	fs,
	path::{
		Path,
		PathBuf,
	},
};

use clap_complete::{
	Generator,
	Shell,
};

use crate::cli::fs_util::*;

#[derive(clap::Args, Debug, Clone)]
pub struct CLIInstallCompletionsCommand {
	/// The shell for which to install the autocompletion script.
	#[arg(value_enum)]
	pub shell: Option<Shell>,
}

const COMMAND_NAME: &str = env!("CARGO_PKG_NAME");
const BLOCK_START: &str =
	concat!("# >>> ", env!("CARGO_PKG_NAME"), " completions >>>");
const BLOCK_END: &str =
	concat!("# <<< ", env!("CARGO_PKG_NAME"), " completions <<<");

pub fn install_completions(
	command: CLIInstallCompletionsCommand,
) -> Result<(), String> {
	let shell = crate::cli::determine_shell(command.shell)?;
	let installation = installation_for(shell)?;

	if let Some(parent) = installation.script.parent() {
		fs::create_dir_all(parent).map_err(|e| {
			format!("Failed to create completion script directories: {}", e)
		})?;
	}

	fs::write(
		&installation.script,
		crate::cli::generate_completions(shell),
	)
	.map_err(|error| format!("Failed to write completion script: {}", error))?;

	if let Some(activation) = installation.activation {
		install_activation(&activation.file, &activation.command).map_err(
			|e| format!("Failed to install activation script: {}", e),
		)?;
	}

	println!(
		"Installed {shell} completions to {}",
		installation.script.display()
	);

	Ok(())
}

struct Installation {
	script: PathBuf,
	activation: Option<Activation>,
}

struct Activation {
	file: PathBuf,
	command: String,
}

fn installation_for(shell: Shell) -> Result<Installation, String> {
	let file_name = shell.file_name(COMMAND_NAME);
	let home = home_dir()?;
	let config_home =
		env_path("XDG_CONFIG_HOME").unwrap_or_else(|| home.join(".config"));
	let data_home =
		env_path("XDG_DATA_HOME").unwrap_or_else(|| home.join(".local/share"));

	match shell {
		Shell::Bash => {
			let user_dir = env::var_os("BASH_COMPLETION_USER_DIR")
				.as_deref()
				.and_then(|paths| {
					env::split_paths(paths)
						.find(|path| !path.as_os_str().is_empty())
				})
				.unwrap_or_else(|| data_home.join("bash-completion"));
			Ok(Installation {
				script: user_dir.join("completions").join(file_name),
				activation: None,
			})
		}
		Shell::Fish => Ok(Installation {
			script: config_home.join("fish/completions").join(file_name),
			activation: None,
		}),
		Shell::Zsh => {
			let script = data_home.join("zsh/site-functions").join(file_name);
			let profile_dir =
				env_path("ZDOTDIR").unwrap_or_else(|| home.to_path_buf());
			let quoted_script = quote_posix_path(&script)?;
			Ok(Installation {
				script,
				activation: Some(Activation {
					file: profile_dir.join(".zshrc"),
					command: format!(
						"autoload -Uz compinit && compinit\nsource \
						 {quoted_script}"
					),
				}),
			})
		}
		Shell::Elvish => Ok(Installation {
			script: config_home.join("elvish/lib").join(file_name),
			activation: Some(Activation {
				file: config_home.join("elvish/rc.elv"),
				command: format!("use {}", COMMAND_NAME),
			}),
		}),
		Shell::PowerShell => {
			let profile_dir = if cfg!(windows) {
				home.join("Documents/PowerShell")
			} else {
				config_home.join("powershell")
			};
			let script = profile_dir.join("completions").join(file_name);
			let quoted_script = quote_powershell_path(&script)?;
			Ok(Installation {
				script,
				activation: Some(Activation {
					file: profile_dir.join("profile.ps1"),
					command: format!(". {quoted_script}"),
				}),
			})
		}
		_ => Err(format!(
			"Automatic completion install is not supported for {shell}."
		)),
	}
}

fn install_activation(path: &Path, command: &str) -> Result<(), String> {
	let mut contents = match fs::read_to_string(path) {
		Ok(contents) => contents,
		Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
			String::new()
		}
		Err(error) => Err(format!(
			"Failed to read install script activation file: {error}"
		))?,
	};

	if let Some(start) = contents.find(BLOCK_START) {
		let relative_end = contents[start..].find(BLOCK_END).ok_or(format!(
			"{} contains an incomplete {COMMAND_NAME} completions block.",
			path.display()
		))?;
		let mut end = start + relative_end + BLOCK_END.len();
		if contents.as_bytes().get(end) == Some(&b'\n') {
			end += 1;
		}
		contents.replace_range(start..end, "");
	}

	if !contents.is_empty() && !contents.ends_with('\n') {
		contents.push('\n');
	}
	contents.push_str(BLOCK_START);
	contents.push('\n');
	contents.push_str(command);
	contents.push('\n');
	contents.push_str(BLOCK_END);
	contents.push('\n');

	if let Some(parent) = path.parent() {
		fs::create_dir_all(parent).map_err(|e| {
			format!("Failed to create activations directory: {}", e)
		})?;
	}
	fs::write(path, contents)
		.map_err(|e| format!("Failed to write activation file: {}", e))
}

#[cfg(test)]
mod tests {
	use std::time::{
		SystemTime,
		UNIX_EPOCH,
	};

	use super::*;

	#[test]
	fn activation_block_is_replaced_instead_of_duplicated() {
		let unique = SystemTime::now()
			.duration_since(UNIX_EPOCH)
			.unwrap()
			.as_nanos();
		let directory = env::temp_dir()
			.join(format!("{COMMAND_NAME}-completions-{unique}"));
		let profile = directory.join("profile");

		install_activation(&profile, "first command").unwrap();
		install_activation(&profile, "second command").unwrap();

		let contents = fs::read_to_string(&profile).unwrap();
		assert!(!contents.contains("first command"));
		assert_eq!(contents.matches(BLOCK_START).count(), 1);
		assert!(contents.contains("second command"));

		fs::remove_dir_all(directory).unwrap();
	}
}
