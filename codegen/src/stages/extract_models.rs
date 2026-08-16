use crate::inputs::spec::OpenApiDocument;
use crate::{
	inputs::Overrides,
	models::{IntegerEnum, Model, StringEnum},
};

pub fn extract_models(
	document: &OpenApiDocument,
	overrides: &Overrides,
) -> Result<Vec<Model>, String> {
	let mut unused_override_models =
		overrides.models.keys().collect::<Vec<_>>();
	let mut models = Vec::new();

	for (schema_name, schema) in document.schemas() {
		let model = match schema.simple_type() {
			Some("integer") if schema.values.is_some() => Some(
				Model::IntegerEnum(IntegerEnum::parse(schema_name, schema)?),
			),
			Some("string") if schema.values.is_some() => {
				let model_override = overrides.models.get(schema_name);
				let model =
					StringEnum::parse(schema_name, schema, model_override)?;
				if model_override.is_some() {
					unused_override_models
						.retain(|name| name.as_str() != schema_name);
				}
				Some(Model::StringEnum(model))
			}
			_ => None,
		};
		if let Some(model) = model {
			models.push(model);
		}
	}

	if !unused_override_models.is_empty() {
		let overrides = unused_override_models
			.into_iter()
			.flat_map(|model| {
				overrides.models[model]
					.variants
					.keys()
					.map(move |variant| format!("{model}.{variant}"))
			})
			.collect::<Vec<_>>()
			.join(", ");
		return Err(format!("unused codegen overrides: {overrides}"));
	}
	if models.is_empty() {
		return Err("OpenAPI document contains no supported models".into());
	}

	models.sort_by(|left, right| left.name().cmp(right.name()));
	Ok(models)
}

#[cfg(test)]
mod tests {
	use super::*;

	fn document(schemas: &str) -> OpenApiDocument {
		OpenApiDocument::parse(&format!(
			r#"{{
					"openapi": "3.1.1",
					"info": {{ "version": "3.16.0" }},
					"components": {{ "schemas": {{ {schemas} }} }}
				}}"#
		))
		.unwrap()
	}

	#[test]
	fn dispatches_integer_and_string_enums() {
		let document = document(
			r#"
			"NumberKind": {
				"type": "integer",
				"enum": [1],
				"x-enum-varnames": ["ONE"]
			},
			"TextKind": {
				"type": "string",
				"enum": ["Wire-Value"]
			}"#,
		);

		let models = extract_models(&document, &Overrides::default()).unwrap();
		assert_eq!(models.len(), 2);
		assert_eq!(models[0].name(), "NumberKind");
		assert_eq!(models[1].name(), "TextKind");
		assert!(matches!(&models[0], Model::IntegerEnum(_)));
		assert!(matches!(&models[1], Model::StringEnum(_)));
	}

	#[test]
	fn rejects_an_override_for_an_unsupported_model() {
		let document = document(
			r#""Object": {
				"type": "object"
			}"#,
		);
		let overrides = Overrides::parse(
			r#"[models.Object.variants]
			MISSING = "Empty"
			"#,
		)
		.unwrap();

		let error = extract_models(&document, &overrides).unwrap_err();
		assert!(error.contains("Object.MISSING"));
	}
}
