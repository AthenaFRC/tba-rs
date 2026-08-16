const RUST_KEYWORDS: &[&str] = &[
	"Self", "abstract", "as", "async", "await", "become", "box", "break",
	"const", "continue", "crate", "do", "dyn", "else", "enum", "extern",
	"false", "final", "fn", "for", "gen", "if", "impl", "in", "let", "loop",
	"macro", "match", "mod", "move", "mut", "override", "priv", "pub", "ref",
	"return", "self", "static", "struct", "super", "trait", "true", "try",
	"type", "typeof", "union", "unsafe", "unsized", "use", "virtual", "where",
	"while", "yield",
];

pub fn type_name(name: &str) -> Result<String, String> {
	if name.is_empty()
		|| !name.chars().all(|character| {
			character.is_ascii_alphanumeric() || character == '_'
		}) {
		return Err(format!("cannot convert `{name}` into a Rust type name"));
	}
	let result = name
		.split('_')
		.filter(|part| !part.is_empty())
		.map(capitalize)
		.collect::<String>();
	validate_identifier(name, result)
}

pub fn variant_name(name: &str) -> Result<String, String> {
	type_name(&name.to_ascii_lowercase())
}

pub fn wire_variant_name(value: &str) -> Result<String, String> {
	let normalized = value
		.chars()
		.map(|character| {
			if character.is_ascii_alphanumeric() {
				character
			} else {
				'_'
			}
		})
		.collect::<String>();
	type_name(&normalized)
}

fn capitalize(value: &str) -> String {
	let mut characters = value.chars();
	match characters.next() {
		Some(first) => {
			first.to_ascii_uppercase().to_string() + characters.as_str()
		}
		None => String::new(),
	}
}

fn validate_identifier(source: &str, result: String) -> Result<String, String> {
	if !result
		.chars()
		.next()
		.is_some_and(|character| character.is_ascii_alphabetic())
		|| RUST_KEYWORDS.contains(&result.as_str())
	{
		return Err(format!(
			"`{source}` produces invalid Rust identifier `{result}`"
		));
	}
	Ok(result)
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn preserves_component_casing_and_normalizes_variants() {
		assert_eq!(type_name("API_Status").unwrap(), "APIStatus");
		assert_eq!(
			variant_name("DOUBLE_ELIM_8_TEAM").unwrap(),
			"DoubleElim8Team"
		);
		assert_eq!(
			wire_variant_name("A_ChevalDeFrise").unwrap(),
			"AChevalDeFrise"
		);
		assert_eq!(wire_variant_name("Round 1").unwrap(), "Round1");
	}

	#[test]
	fn rejects_invalid_identifiers() {
		assert!(type_name("3d_model").is_err());
		assert!(variant_name("SELF").is_err());
		assert!(type_name("with-hyphen").is_err());
	}
}
