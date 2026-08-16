use crate::inputs::spec::OpenApiDocument;
use crate::{
	inputs::Overrides,
	models::{IntegerEnum, Model, Object, StringEnum},
};

pub fn extract_models(
	document: &OpenApiDocument,
	overrides: &Overrides,
) -> Result<Vec<Model>, String> {
	let mut unused_override_models =
		overrides.models.keys().collect::<Vec<_>>();
	let mut unused_manual_models = overrides.manual_models.clone();
	let mut models = Vec::new();

	for (schema_name, schema) in document.schemas() {
		if overrides.manual_models.contains(schema_name) {
			unused_manual_models.remove(schema_name);
			continue;
		}
		let model = match schema.simple_type() {
			Some("integer") if schema.values.is_some() => Some(
				Model::IntegerEnum(IntegerEnum::parse(schema_name, schema)?),
			),
			Some("string") if schema.values.is_some() => {
				let model_override = overrides.models.get(schema_name);
				if model_override.is_some_and(|model_override| {
					!model_override.types.is_empty()
				}) {
					return Err(format!(
						"type-name overrides are not supported for string enum `{schema_name}`"
					));
				}
				let model =
					StringEnum::parse(schema_name, schema, model_override)?;
				if model_override.is_some() {
					unused_override_models
						.retain(|name| name.as_str() != schema_name);
				}
				Some(Model::StringEnum(model))
			}
			_ if schema.has_type("object")
				|| !schema.properties.is_empty()
				|| schema.additional_properties.is_some() =>
			{
				let model_override = overrides.models.get(schema_name);
				let model = Object::parse_with_override(
					schema_name,
					schema,
					model_override,
				)?;
				if model_override.is_some() {
					unused_override_models
						.retain(|name| name.as_str() != schema_name);
				}
				Some(Model::Object(model))
			}
			_ => {
				return Err(format!(
					"schema `{schema_name}` uses an unsupported top-level shape"
				));
			}
		};
		if let Some(model) = model {
			models.push(model);
		}
	}

	if !unused_override_models.is_empty() {
		let overrides = unused_override_models
			.into_iter()
			.flat_map(|model| {
				let model_override = &overrides.models[model];
				let entries = model_override
					.variants
					.keys()
					.map(|variant| format!("{model}.variants.{variant}"))
					.chain(
						model_override
							.types
							.keys()
							.map(|name| format!("{model}.types.{name}")),
					)
					.collect::<Vec<_>>();
				if entries.is_empty() {
					vec![model.to_string()]
				} else {
					entries
				}
			})
			.collect::<Vec<_>>()
			.join(", ");
		return Err(format!("unused codegen overrides: {overrides}"));
	}
	if !unused_manual_models.is_empty() {
		return Err(format!(
			"manual model overrides do not match schemas: {}",
			unused_manual_models
				.into_iter()
				.collect::<Vec<_>>()
				.join(", ")
		));
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
	fn dispatches_object_models() {
		let document = document(
			r#""Example": {
				"type": "object",
				"required": ["key"],
				"properties": { "key": { "type": "string" } }
			}"#,
		);

		let models = extract_models(&document, &Overrides::default()).unwrap();
		assert!(matches!(&models[0], Model::Object(_)));
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
		assert!(error.contains("Object.variants.MISSING"));
	}

	#[test]
	fn applies_and_stale_checks_nested_type_overrides() {
		let document = document(
			r#""Example": {
				"type": "object",
				"required": ["cmp_status"],
				"properties": {
					"cmp_status": {
						"type": "string",
						"enum": ["Ready"]
					}
				}
			}"#,
		);
		let overrides = Overrides::parse(
			r#"[models.Example.types]
			ExampleCmpStatus = "ExampleCMPStatus"
			"#,
		)
		.unwrap();

		let models = extract_models(&document, &overrides).unwrap();
		let rendered = models[0].render().to_string();
		assert!(rendered.contains("pub enum ExampleCMPStatus"));
		assert!(rendered.contains("cmp_status : ExampleCMPStatus"));

		let stale = Overrides::parse(
			r#"[models.Example.types]
			Missing = "Renamed"
			"#,
		)
		.unwrap();
		assert!(extract_models(&document, &stale).is_err());
	}

	#[test]
	fn skips_and_stale_checks_manual_models() {
		let only_manual = document(
			r#""Manual": { "oneOf": [{ "type": "string" }, { "type": "integer" }] }"#,
		);
		let overrides =
			Overrides::parse("manual_models = [\"Manual\"]").unwrap();
		assert!(extract_models(&only_manual, &overrides).is_err());

		let document = document(
			r#"
			"Generated": {
				"type": "object",
				"required": ["key"],
				"properties": { "key": { "type": "string" } }
			},
			"Manual": { "oneOf": [{ "type": "string" }, { "type": "integer" }] }
			"#,
		);
		let models = extract_models(&document, &overrides).unwrap();
		assert_eq!(models.len(), 1);
	}
}
