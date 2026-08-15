pub(crate) const RESPONSE_BODY_EXCERPT_MAX_BYTES: usize = 1024;

pub(crate) fn response_body_excerpt(body: &[u8]) -> String {
	let excerpt_end = body.len().min(RESPONSE_BODY_EXCERPT_MAX_BYTES);
	let excerpt = String::from_utf8_lossy(&body[..excerpt_end]);
	let mut excerpt = format_json_fragment(&excerpt);
	let was_truncated = body.len() > RESPONSE_BODY_EXCERPT_MAX_BYTES
		|| excerpt.len() > RESPONSE_BODY_EXCERPT_MAX_BYTES;
	truncate_at_char_boundary(&mut excerpt, RESPONSE_BODY_EXCERPT_MAX_BYTES);
	if was_truncated {
		excerpt.push('…');
	}
	excerpt
}

// Decode errors commonly contain a compact JSON object or array, but the
// bounded excerpt often ends before that JSON does. Format its structure
// without parsing it so incomplete and malformed responses remain readable.
fn format_json_fragment(fragment: &str) -> String {
	let fragment = fragment.trim_start();
	if !matches!(fragment.chars().next(), Some('{' | '[')) {
		return escape_control_characters(fragment);
	}

	let mut formatted = String::with_capacity(fragment.len());
	let mut indentation = 0usize;
	let mut in_string = false;
	let mut escaped = false;
	let mut previous_significant = None;

	for character in fragment.chars() {
		if in_string {
			push_escaped_control_character(&mut formatted, character);
			if escaped {
				escaped = false;
			} else if character == '\\' {
				escaped = true;
			} else if character == '"' {
				in_string = false;
			}
			continue;
		}

		match character {
			'"' => {
				in_string = true;
				formatted.push(character);
			}
			'{' | '[' => {
				formatted.push(character);
				indentation += 1;
				push_indented_newline(&mut formatted, indentation);
			}
			'}' | ']' => {
				indentation = indentation.saturating_sub(1);
				if matches!(
					(previous_significant, character),
					(Some('{'), '}') | (Some('['), ']')
				) {
					while formatted
						.chars()
						.next_back()
						.is_some_and(char::is_whitespace)
					{
						formatted.pop();
					}
					formatted.push(character);
				} else {
					push_indented_newline(&mut formatted, indentation);
					formatted.push(character);
				}
			}
			',' => {
				formatted.push(character);
				push_indented_newline(&mut formatted, indentation);
			}
			':' => formatted.push_str(": "),
			character if character.is_whitespace() => {}
			character => {
				push_escaped_control_character(&mut formatted, character)
			}
		}
		if !character.is_whitespace() {
			previous_significant = Some(character);
		}
	}

	formatted.trim_end().to_owned()
}

fn push_indented_newline(formatted: &mut String, indentation: usize) {
	formatted.push('\n');
	// Bounding indentation prevents a malicious response containing only
	// opening delimiters from causing quadratic allocation in this error path.
	formatted.extend(std::iter::repeat_n(' ', indentation.min(32) * 2));
}

fn escape_control_characters(value: &str) -> String {
	let mut escaped = String::with_capacity(value.len());
	for character in value.chars() {
		push_escaped_control_character(&mut escaped, character);
	}
	escaped
}

fn push_escaped_control_character(value: &mut String, character: char) {
	if character.is_control() {
		value.extend(character.escape_default());
	} else {
		value.push(character);
	}
}

fn truncate_at_char_boundary(value: &mut String, max_bytes: usize) {
	if value.len() <= max_bytes {
		return;
	}
	let mut end = max_bytes;
	while !value.is_char_boundary(end) {
		end -= 1;
	}
	value.truncate(end);
}

#[cfg(test)]
mod tests {
	use super::response_body_excerpt;

	#[test]
	fn partial_json_excerpt_is_formatted_without_parsing() {
		let body = br#"{"outer":{"items":[{"id":1,"name":"one"},{"id":2"#;

		assert_eq!(
			response_body_excerpt(body),
			concat!(
				"{\n",
				"  \"outer\": {\n",
				"    \"items\": [\n",
				"      {\n",
				"        \"id\": 1,\n",
				"        \"name\": \"one\"\n",
				"      },\n",
				"      {\n",
				"        \"id\": 2",
			)
		);
		assert_eq!(
			response_body_excerpt(br#"{"empty":{},"items":[]}"#),
			"{\n  \"empty\": {},\n  \"items\": []\n}"
		);
	}
}
