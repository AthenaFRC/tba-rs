use std::fs;

use crate::{inputs::Config, stages::generate_models};

pub fn write_generated_models(config: &Config) -> Result<(), String> {
	let path = &config.generated_models_file_path;
	let generated = generate_models(config)?;
	if fs::read_to_string(path).ok().as_deref() == Some(&generated) {
		println!("{} is already up to date", path.display());
		return Ok(());
	}
	let parent = path
		.parent()
		.ok_or_else(|| format!("{} has no parent directory", path.display()))?;
	fs::create_dir_all(parent).map_err(|error| {
		format!("failed to create {}: {error}", parent.display())
	})?;
	fs::write(path, generated).map_err(|error| {
		format!("failed to write {}: {error}", path.display())
	})?;
	println!("generated {}", path.display());
	Ok(())
}
