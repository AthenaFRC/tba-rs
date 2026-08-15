pub fn __tba_endpoint_cli_name(name: &'static str) -> String {
	name.trim_end_matches('_').replace('_', "-")
}

pub fn __tba_endpoint_doc_comment(lines: &[&'static str]) -> String {
	lines
		.iter()
		.flat_map(|line| line.split('\n'))
		.map(|line| line.strip_prefix(' ').unwrap_or(line).trim())
		.filter(|line| !line.is_empty())
		.collect::<Vec<_>>()
		.join(" ")
}
