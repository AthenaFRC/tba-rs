pub fn home_dir() -> Result<std::path::PathBuf, String> {
	env_path("HOME")
		.or_else(|| env_path("USERPROFILE"))
		.ok_or("Could not determine home directory.".parse().unwrap())
}

pub fn env_path(name: &str) -> Option<std::path::PathBuf> {
	std::env::var_os(name)
		.filter(|value| !value.is_empty())
		.map(std::path::PathBuf::from)
}

pub fn quote_posix_path(path: &std::path::Path) -> Result<String, String> {
	let path = path
		.to_str()
		.ok_or("Failed to parse completion path as UTF-8")?;
	Ok(format!("'{}'", path.replace('\'', "'\\''")))
}

pub fn quote_powershell_path(path: &std::path::Path) -> Result<String, String> {
	let path = path
		.to_str()
		.ok_or("Failed to parse completion path as UTF-8")?;
	Ok(format!("'{}'", path.replace('\'', "''")))
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn powershell_paths_are_single_quote_escaped() {
		let quoted = quote_powershell_path(std::path::Path::new(
			"/home/tester's/completions.ps1",
		))
		.unwrap();

		assert_eq!(quoted, "'/home/tester''s/completions.ps1'");
	}

	#[test]
	fn posix_paths_are_single_quote_escaped() {
		let quoted = quote_posix_path(std::path::Path::new(concat!(
			"/home/tester's/_",
			env!("CARGO_PKG_NAME")
		)))
		.unwrap();

		assert_eq!(
			quoted,
			concat!("'/home/tester'\\''s/_", env!("CARGO_PKG_NAME"), "'")
		);
	}
}
