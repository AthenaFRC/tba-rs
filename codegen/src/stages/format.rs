use std::{
	io::Write,
	path::Path,
	process::{Command, Stdio},
};

pub fn format(source: &str, workspace_root: &Path) -> Result<String, String> {
	let mut child = Command::new("rustfmt")
		.args(["--emit", "stdout", "--edition", "2024", "--config-path"])
		.arg(workspace_root)
		.stdin(Stdio::piped())
		.stdout(Stdio::piped())
		.stderr(Stdio::piped())
		.spawn()
		.map_err(|error| format!("failed to start rustfmt: {error}"))?;

	child
		.stdin
		.take()
		.expect("rustfmt stdin was configured")
		.write_all(source.as_bytes())
		.map_err(|error| {
			format!("failed to send generated source to rustfmt: {error}")
		})?;
	let output = child
		.wait_with_output()
		.map_err(|error| format!("failed to wait for rustfmt: {error}"))?;
	if !output.status.success() {
		let stderr = String::from_utf8_lossy(&output.stderr).trim().to_owned();
		return Err(format!("rustfmt rejected generated source: {stderr}"));
	}

	String::from_utf8(output.stdout)
		.map_err(|error| format!("rustfmt produced invalid UTF-8: {error}"))
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn applies_the_workspace_rustfmt_configuration() {
		let source = "fn example() {\n    if true {\n        println!(\"ok\");\n    }\n}\n";
		let formatted =
			format(source, &crate::util::fs::workspace_root()).unwrap();
		assert!(formatted.contains("\n\tif true {\n\t\tprintln!"));
	}
}
