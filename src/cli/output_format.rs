#[derive(clap::ValueEnum, serde::Serialize, serde::Deserialize, Debug, Clone)]
pub enum OutputFormat {
	/// Standard JSON format, without line breaks or indentation.
	#[value(name = "json")]
	JSON,

	/// Pretty-printed JSON format, including line breaks and indentation via
	/// tabs.
	#[value(name = "json-pretty-tabs")]
	JSONPrettyTabs,

	/// Pretty-printed JSON format, including line breaks and indentation via 2
	/// spaces.
	#[value(name = "json-pretty-2spaces")]
	JSONPretty2Spaces,

	/// Pretty-printed JSON format, including line breaks and indentation via 4
	/// spaces.
	#[value(name = "json-pretty-4spaces")]
	JSONPretty4Spaces,

	/// JSON Lines format. Each line of the resulting output will be a valid
	/// JSON object.
	#[value(name = "jsonl")]
	JSONL,

	/// Comma separative value format.
	#[value(name = "csv")]
	CSV,

	/// Tab separated value format.
	#[value(name = "tsv")]
	TSV,
}
