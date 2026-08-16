use std::path::{Path, PathBuf};

pub fn workspace_root() -> PathBuf {
	Path::new(env!("CARGO_MANIFEST_DIR"))
		.parent()
		.expect("codegen package must be inside the workspace")
		.to_owned()
}
