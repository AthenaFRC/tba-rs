use serde_json::{
	Value,
	json,
};
use tba::models::{
	InsightV2LeaderboardDataRankingsItemContextsItem,
	Media,
};

const MEDIA_FIXTURES: [(&str, &str); 6] = [
	(
		include_str!("fixtures/api_v3_16_0/media_avatar.json"),
		"avatar",
	),
	(
		include_str!("fixtures/api_v3_16_0/media_cd_photo_thread.json"),
		"cd_photo_thread",
	),
	(
		include_str!("fixtures/api_v3_16_0/media_cd_thread.json"),
		"cd_thread",
	),
	(
		include_str!("fixtures/api_v3_16_0/media_grab_cad.json"),
		"grab_cad",
	),
	(
		include_str!("fixtures/api_v3_16_0/media_no_details.json"),
		"no_details",
	),
	(
		include_str!("fixtures/api_v3_16_0/media_onshape.json"),
		"onshape",
	),
];

fn media_variant(media: &Media) -> &'static str {
	match media {
		Media::Avatar { .. } => "avatar",
		Media::CdPhotoThread { .. } => "cd_photo_thread",
		Media::CdThread { .. } => "cd_thread",
		Media::GrabCad { .. } => "grab_cad",
		Media::NoDetails { .. } => "no_details",
		Media::Onshape { .. } => "onshape",
	}
}

fn media_value(type_: &str, details: Option<Value>) -> Value {
	let mut value = json!({
		"type": type_,
		"foreign_key": "fixture-key",
		"preferred": null,
		"team_keys": ["frc254"],
		"direct_url": null,
		"view_url": null,
	});
	if let Some(details) = details {
		value["details"] = details;
	}
	value
}

#[test]
fn media_fixtures_select_variants_and_round_trip() {
	for (fixture, expected_variant) in MEDIA_FIXTURES {
		let input: Value = serde_json::from_str(fixture).unwrap();
		let media: Media = serde_json::from_value(input.clone()).unwrap();

		assert_eq!(media_variant(&media), expected_variant);
		assert_eq!(serde_json::to_value(media).unwrap(), input);
	}
}

#[test]
fn every_no_details_discriminator_round_trips_exactly() {
	const TYPES: [&str; 11] = [
		"youtube",
		"imgur",
		"facebook-profile",
		"youtube-channel",
		"twitter-profile",
		"github-profile",
		"instagram-profile",
		"periscope-profile",
		"gitlab-profile",
		"instagram-image",
		"external-link",
	];
	let fixture: Value = serde_json::from_str(include_str!(
		"fixtures/api_v3_16_0/media_no_details.json"
	))
	.unwrap();

	for type_ in TYPES {
		let mut input = fixture.clone();
		input["type"] = Value::String(type_.to_owned());
		let media: Media = serde_json::from_value(input.clone()).unwrap();

		match &media {
			Media::NoDetails {
				type_: parsed_type, ..
			} => assert_eq!(serde_json::to_value(parsed_type).unwrap(), type_),
			other => panic!("{type_} selected {}", media_variant(other)),
		}
		assert_eq!(serde_json::to_value(media).unwrap(), input);
	}
}

#[test]
fn media_discriminator_wins_with_any_optional_details_state() {
	let media_types = [
		("avatar", "avatar"),
		("cdphotothread", "cd_photo_thread"),
		("cd-thread", "cd_thread"),
		("grabcad", "grab_cad"),
		("youtube", "no_details"),
		("onshape", "onshape"),
	];

	for (type_, expected_variant) in media_types {
		for details in [None, Some(Value::Null)] {
			let media: Media =
				serde_json::from_value(media_value(type_, details)).unwrap();
			assert_eq!(media_variant(&media), expected_variant);
		}
	}
}

#[test]
fn media_rejects_invalid_discriminators() {
	let missing = media_value("avatar", None)
		.as_object()
		.unwrap()
		.iter()
		.filter(|(key, _)| key.as_str() != "type")
		.map(|(key, value)| (key.clone(), value.clone()))
		.collect::<serde_json::Map<_, _>>();
	let cases = [
		(Value::Object(missing), "missing media discriminator"),
		(media_value("avatar", None), ""),
		(
			media_value("future-media", None),
			"unknown media discriminator",
		),
	];

	for (mut input, expected_error) in cases {
		if expected_error.is_empty() {
			input["type"] = json!(42);
		}
		let error = serde_json::from_value::<Media>(input).unwrap_err();
		let expected_error = if expected_error.is_empty() {
			"must be a string"
		} else {
			expected_error
		};
		assert!(
			error.to_string().contains(expected_error),
			"unexpected error: {error}"
		);
	}
}

#[test]
fn insight_context_fixtures_select_variants_and_round_trip() {
	let event_input: Value = serde_json::from_str(include_str!(
		"fixtures/api_v3_16_0/insight_context_event_list.json"
	))
	.unwrap();
	let event_context: InsightV2LeaderboardDataRankingsItemContextsItem =
		serde_json::from_value(event_input.clone()).unwrap();
	assert!(matches!(
		event_context,
		InsightV2LeaderboardDataRankingsItemContextsItem::EventList { .. }
	));
	assert_eq!(serde_json::to_value(event_context).unwrap(), event_input);

	let match_input: Value = serde_json::from_str(include_str!(
		"fixtures/api_v3_16_0/insight_context_match_alliance.json"
	))
	.unwrap();
	let match_context: InsightV2LeaderboardDataRankingsItemContextsItem =
		serde_json::from_value(match_input.clone()).unwrap();
	assert!(matches!(
		match_context,
		InsightV2LeaderboardDataRankingsItemContextsItem::MatchAlliance { .. }
	));
	assert_eq!(serde_json::to_value(match_context).unwrap(), match_input);
}

#[test]
fn insight_contexts_tolerate_extra_fields() {
	let event_context: InsightV2LeaderboardDataRankingsItemContextsItem =
		serde_json::from_value(json!({
			"event_keys": ["2026miket"],
			"upstream_extension": true,
		}))
		.unwrap();
	assert!(matches!(
		event_context,
		InsightV2LeaderboardDataRankingsItemContextsItem::EventList { .. }
	));

	let match_context: InsightV2LeaderboardDataRankingsItemContextsItem =
		serde_json::from_value(json!({
			"match_key": "2026miket_qm1",
			"alliance": ["frc254"],
			"upstream_extension": true,
		}))
		.unwrap();
	assert!(matches!(
		match_context,
		InsightV2LeaderboardDataRankingsItemContextsItem::MatchAlliance { .. }
	));
}

#[test]
fn insight_contexts_reject_partial_match_markers() {
	for input in [
		json!({ "match_key": "2026miket_qm1" }),
		json!({ "alliance": ["frc254"] }),
	] {
		let error = serde_json::from_value::<
			InsightV2LeaderboardDataRankingsItemContextsItem,
		>(input)
		.unwrap_err();
		assert!(
			error.to_string().contains("missing field"),
			"unexpected error: {error}"
		);
	}
}

#[test]
fn insight_event_list_allows_omitted_event_keys() {
	let context: InsightV2LeaderboardDataRankingsItemContextsItem =
		serde_json::from_value(json!({})).unwrap();
	assert!(matches!(
		context,
		InsightV2LeaderboardDataRankingsItemContextsItem::EventList {
			event_keys: None
		}
	));
}
