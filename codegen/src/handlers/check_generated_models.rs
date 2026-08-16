use std::{fs, path::Path};

use crate::{inputs::Config, stages::generate_models};

pub fn check_generated_models(config: &Config) -> Result<(), String> {
	let path = &config.generated_models_file_path;
	let generated = generate_models(config)?;
	check_generated_file(path, &generated)?;
	println!("{} is up to date", path.display());
	Ok(())
}

fn check_generated_file(path: &Path, generated: &str) -> Result<(), String> {
	let existing = fs::read_to_string(path).map_err(|error| {
		format!(
			"failed to read {}: {error}; run the generator first",
			path.display()
		)
	})?;
	if existing != generated {
		return Err(format!(
			"{} is out of date; run `cargo regen-models`",
			path.display()
		));
	}
	Ok(())
}

#[cfg(test)]
mod tests {
	use std::time::{SystemTime, UNIX_EPOCH};

	use super::*;

	fn temporary_path() -> std::path::PathBuf {
		let unique = SystemTime::now()
			.duration_since(UNIX_EPOCH)
			.unwrap()
			.as_nanos();
		std::env::temp_dir()
			.join(format!("codegen-{}-{unique}.rs", std::process::id()))
	}

	#[test]
	fn check_detects_stale_output() {
		let path = temporary_path();
		fs::write(&path, "old").unwrap();
		assert!(check_generated_file(&path, "new").is_err());
		fs::write(&path, "new").unwrap();
		assert!(check_generated_file(&path, "new").is_ok());
		fs::remove_file(path).unwrap();
	}
}
