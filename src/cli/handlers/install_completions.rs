use std::{
	env,
	fs,
	io::{
		self,
		ErrorKind,
	},
	path::{
		Path,
		PathBuf,
	},
};

use clap_complete::{
	Generator,
	Shell,
};

const COMMAND_NAME: &str = "tba";
const BLOCK_START: &str = "# >>> tba completions >>>";
const BLOCK_END: &str = "# <<< tba completions <<<";

pub fn install_completions(shell: Option<Shell>) {
	let shell = match shell.or_else(Shell::from_env).ok_or_else(|| {
		io::Error::new(
			ErrorKind::NotFound,
			"no shell specified and the current shell could not be determined",
		)
	}) {
		Ok(shell) => shell,
		Err(error) => {
			eprintln!("Error: {}", error);
			return;
		}
	};

	let installation = match installation_for(shell) {
		Ok(installation) => installation,
		Err(error) => {
			eprintln!("Error: {}", error);
			return;
		}
	};

	if let Some(parent) = installation.script.parent() {
		if let Err(error) = fs::create_dir_all(parent) {
			eprintln!("Error: {}", error);
			return;
		}
	}

	if let Err(error) = fs::write(
		&installation.script,
		crate::cli::generate_completions(shell),
	) {
		eprintln!("Error: {}", error);
		return;
	}

	if let Some(activation) = installation.activation {
		if let Err(error) =
			install_activation(&activation.file, &activation.command)
		{
			eprintln!("Error: {}", error);
			return;
		}
	}

	println!(
		"Installed {shell} completions to {}",
		installation.script.display()
	);
}

struct Installation {
	script: PathBuf,
	activation: Option<Activation>,
}

struct Activation {
	file: PathBuf,
	command: String,
}

fn installation_for(shell: Shell) -> io::Result<Installation> {
	let file_name = shell.file_name(COMMAND_NAME);
	let home = home_dir()?;
	let config_home =
		env_path("XDG_CONFIG_HOME").unwrap_or_else(|| home.join(".config"));
	let data_home =
		env_path("XDG_DATA_HOME").unwrap_or_else(|| home.join(".local/share"));

	let installation = match shell {
		Shell::Bash => {
			let user_dir = env::var_os("BASH_COMPLETION_USER_DIR")
				.as_deref()
				.and_then(|paths| {
					env::split_paths(paths)
						.find(|path| !path.as_os_str().is_empty())
				})
				.unwrap_or_else(|| data_home.join("bash-completion"));
			Installation {
				script: user_dir.join("completions").join(file_name),
				activation: None,
			}
		}
		Shell::Fish => Installation {
			script: config_home.join("fish/completions").join(file_name),
			activation: None,
		},
		Shell::Zsh => {
			let script = data_home.join("zsh/site-functions").join(file_name);
			let profile_dir =
				env_path("ZDOTDIR").unwrap_or_else(|| home.to_path_buf());
			let quoted_script = quote_posix_path(&script)?;
			Installation {
				script,
				activation: Some(Activation {
					file: profile_dir.join(".zshrc"),
					command: format!(
						"autoload -Uz compinit && compinit\nsource \
						 {quoted_script}"
					),
				}),
			}
		}
		Shell::Elvish => Installation {
			script: config_home.join("elvish/lib").join(file_name),
			activation: Some(Activation {
				file: config_home.join("elvish/rc.elv"),
				command: "use tba".to_owned(),
			}),
		},
		Shell::PowerShell => {
			let profile_dir = if cfg!(windows) {
				home.join("Documents/PowerShell")
			} else {
				config_home.join("powershell")
			};
			let script = profile_dir.join("completions").join(file_name);
			let quoted_script = quote_powershell_path(&script)?;
			Installation {
				script,
				activation: Some(Activation {
					file: profile_dir.join("profile.ps1"),
					command: format!(". {quoted_script}"),
				}),
			}
		}
		_ => {
			return Err(io::Error::new(
				ErrorKind::Unsupported,
				format!("installing completions for {shell} is not supported"),
			));
		}
	};

	Ok(installation)
}

fn install_activation(path: &Path, command: &str) -> io::Result<()> {
	let mut contents = match fs::read_to_string(path) {
		Ok(contents) => contents,
		Err(error) if error.kind() == ErrorKind::NotFound => String::new(),
		Err(error) => return Err(error),
	};

	if let Some(start) = contents.find(BLOCK_START) {
		let relative_end =
			contents[start..].find(BLOCK_END).ok_or_else(|| {
				io::Error::new(
					ErrorKind::InvalidData,
					format!(
						"{} contains an incomplete tba completions block",
						path.display()
					),
				)
			})?;
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
		fs::create_dir_all(parent)?;
	}
	fs::write(path, contents)
}

fn home_dir() -> io::Result<PathBuf> {
	env_path("HOME")
		.or_else(|| env_path("USERPROFILE"))
		.ok_or_else(|| {
			io::Error::new(
				ErrorKind::NotFound,
				"could not determine the home directory",
			)
		})
}

fn env_path(name: &str) -> Option<PathBuf> {
	env::var_os(name)
		.filter(|value| !value.is_empty())
		.map(PathBuf::from)
}

fn quote_posix_path(path: &Path) -> io::Result<String> {
	let path = path.to_str().ok_or_else(|| {
		io::Error::new(
			ErrorKind::InvalidData,
			"the completion path is not valid UTF-8",
		)
	})?;
	Ok(format!("'{}'", path.replace('\'', "'\\''")))
}

fn quote_powershell_path(path: &Path) -> io::Result<String> {
	let path = path.to_str().ok_or_else(|| {
		io::Error::new(
			ErrorKind::InvalidData,
			"the completion path is not valid UTF-8",
		)
	})?;
	Ok(format!("'{}'", path.replace('\'', "''")))
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
		let directory =
			env::temp_dir().join(format!("tba-completions-{unique}"));
		let profile = directory.join("profile");

		install_activation(&profile, "first command").unwrap();
		install_activation(&profile, "second command").unwrap();

		let contents = fs::read_to_string(&profile).unwrap();
		assert!(!contents.contains("first command"));
		assert_eq!(contents.matches(BLOCK_START).count(), 1);
		assert!(contents.contains("second command"));

		fs::remove_dir_all(directory).unwrap();
	}

	#[test]
	fn powershell_paths_are_single_quote_escaped() {
		let quoted =
			quote_powershell_path(Path::new("/home/tester's/completions.ps1"))
				.unwrap();

		assert_eq!(quoted, "'/home/tester''s/completions.ps1'");
	}

	#[test]
	fn posix_paths_are_single_quote_escaped() {
		let quoted =
			quote_posix_path(Path::new("/home/tester's/_tba")).unwrap();

		assert_eq!(quoted, "'/home/tester'\\''s/_tba'");
	}
}
