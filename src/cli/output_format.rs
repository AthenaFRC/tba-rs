#[derive(
	clap::ValueEnum, serde::Serialize, serde::Deserialize, Debug, Clone,
)]
#[clap(rename_all = "kebab-case")]
#[serde(rename_all = "kebab-case")]
pub enum OutputFormat {
	/// Standard JSON format, without line breaks or indentation.
	JSON,

	/// Pretty-printed JSON format, including line breaks and indentation via
	/// tabs.
	JSONPrettyTabs,

	/// Pretty-printed JSON format, including line breaks and indentation via 2
	/// spaces.
	#[value(name = "json-pretty-2spaces")]
	#[serde(rename = "json-pretty-2spaces")]
	JSONPretty2Spaces,

	/// Pretty-printed JSON format, including line breaks and indentation via 4
	/// spaces.
	#[value(name = "json-pretty-4spaces")]
	#[serde(rename = "json-pretty-4spaces")]
	JSONPretty4Spaces,

	/// JSON Lines format. Each line of the resulting output will be a valid
	/// JSON object.
	JSONL,

	/// Comma separative value format.
	CSV,

	/// Tab separated value format.
	TSV,
}

impl Default for OutputFormat {
	fn default() -> Self {
		OutputFormat::JSON
	}
}
