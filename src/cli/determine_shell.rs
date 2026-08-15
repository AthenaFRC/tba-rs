/// Attempts to determine the current shell being used by the caller.
///
/// # Arguments
/// * `shell` - An optional `clap_complete::Shell` value that, if provided, will
///   be used as the shell to generate completions for. If not provided, the
///   function will attempt to determine the current shell from the environment.
///
/// # Examples
/// ```rust
/// use tba::cli::determine_shell::determine_shell;
/// if let Some(shell) = determine_shell(None) {
/// 	println!("Current shell: {:?}", shell);
/// }
/// ```
pub fn determine_shell(
	shell: Option<clap_complete::Shell>,
) -> Result<clap_complete::Shell, String> {
	shell
		.ok_or_else(clap_complete::Shell::from_env)
		.map_err(|_| {
			"no shell specified and the current shell could not be determined"
				.to_string()
		})
}
