use tba::models::{
	AllianceColor, AwardType, CompLevel, DoubleElimRound, EventType,
	PlayoffType, Position2016, WebcastStatus,
};

#[test]
fn named_integer_enums_use_their_numeric_wire_values() {
	let award: AwardType = serde_json::from_str("1").unwrap();
	let event: EventType = serde_json::from_str("-1").unwrap();
	let playoff: PlayoffType = serde_json::from_str("10").unwrap();

	assert_eq!(award, AwardType::Winner);
	assert_eq!(event, EventType::Unlabled);
	assert_eq!(playoff, PlayoffType::DoubleElim8Team);
	assert_eq!(serde_json::to_string(&award).unwrap(), "1");
	assert_eq!(serde_json::to_string(&event).unwrap(), "-1");
	assert_eq!(serde_json::to_string(&playoff).unwrap(), "10");
}

#[test]
fn named_integer_enums_preserve_unknown_values() {
	let award: AwardType = serde_json::from_str("9000").unwrap();

	assert_eq!(award, AwardType::Unknown(9000));
	assert_eq!(award.value(), 9000);
	assert_eq!(i64::from(award), 9000);
	assert_eq!(serde_json::to_string(&award).unwrap(), "9000");
}

#[test]
fn named_string_enums_use_exact_wire_values() {
	let empty: AllianceColor = serde_json::from_str("\"\"").unwrap();
	let online: WebcastStatus = serde_json::from_str(r#""online""#).unwrap();

	assert_eq!(empty, AllianceColor::Empty);
	assert_eq!(online, WebcastStatus::Online);
	assert_eq!(serde_json::to_string(&empty).unwrap(), "\"\"");
	assert_eq!(serde_json::to_string(&online).unwrap(), r#""online""#);
	assert!(serde_json::from_str::<AllianceColor>(r#""green""#).is_err());
}

#[test]
fn generated_string_enum_overrides_preserve_the_public_variants() {
	let qualification: CompLevel = serde_json::from_str(r#""qm""#).unwrap();
	let empty_position: Position2016 = serde_json::from_str("\"\"").unwrap();
	let cheval: Position2016 =
		serde_json::from_str(r#""A_ChevalDeFrise""#).unwrap();
	let round: DoubleElimRound = serde_json::from_str(r#""Round 1""#).unwrap();

	assert_eq!(qualification, CompLevel::QualificationMatch);
	assert_eq!(empty_position, Position2016::Empty);
	assert_eq!(cheval, Position2016::AChevalDeFrise);
	assert_eq!(round, DoubleElimRound::Round1);
	assert_eq!(serde_json::to_string(&qualification).unwrap(), r#""qm""#);
	assert_eq!(
		serde_json::to_string(&cheval).unwrap(),
		r#""A_ChevalDeFrise""#
	);
	assert_eq!(serde_json::to_string(&round).unwrap(), r#""Round 1""#);
}
