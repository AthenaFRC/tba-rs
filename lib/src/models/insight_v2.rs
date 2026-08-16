use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum InsightV2LeaderboardDataKeyType {
	Team,
	Event,
	Match,
	TeamPair,
	Alliance,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum InsightV2LeaderboardDataContextType {
	EventList,
	None,
	MatchAlliance,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum InsightV2LeaderboardDataRankingsItemKeys {
	Vec(Vec<String>),
	Vec2(Vec<Vec<String>>),
}

// This shape-only union uses match markers to avoid letting the optional
// event-list shape absorb a match/alliance context.
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum InsightV2LeaderboardDataRankingsItemContextsItem {
	MatchAlliance {
		match_key: String,
		alliance: Vec<String>,
	},
	EventList {
		event_keys: Option<Vec<String>>,
	},
}

#[derive(Deserialize)]
struct InsightV2LeaderboardMatchAllianceContext {
	match_key: String,
	alliance: Vec<String>,
}

#[derive(Deserialize)]
struct InsightV2LeaderboardEventListContext {
	event_keys: Option<Vec<String>>,
}

impl<'de> Deserialize<'de>
	for InsightV2LeaderboardDataRankingsItemContextsItem
{
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: serde::Deserializer<'de>,
	{
		use serde::de::Error;

		let value = serde_json::Value::deserialize(deserializer)?;
		let object = value.as_object().ok_or_else(|| {
			D::Error::custom("insight leaderboard context must be an object")
		})?;
		if object.contains_key("match_key") || object.contains_key("alliance") {
			let context: InsightV2LeaderboardMatchAllianceContext =
				serde_json::from_value(value).map_err(D::Error::custom)?;
			Ok(Self::MatchAlliance {
				match_key: context.match_key,
				alliance: context.alliance,
			})
		} else {
			let context: InsightV2LeaderboardEventListContext =
				serde_json::from_value(value).map_err(D::Error::custom)?;
			Ok(Self::EventList {
				event_keys: context.event_keys,
			})
		}
	}
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct InsightV2LeaderboardDataRankingsItem {
	pub keys: InsightV2LeaderboardDataRankingsItemKeys,
	pub value: f64,
	pub contexts: Option<Vec<InsightV2LeaderboardDataRankingsItemContextsItem>>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct InsightV2LeaderboardData {
	pub key_type: InsightV2LeaderboardDataKeyType,
	pub context_type: InsightV2LeaderboardDataContextType,
	pub rankings: Vec<InsightV2LeaderboardDataRankingsItem>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct InsightV2Leaderboard {
	pub name: String,
	pub display_name: String,
	pub year: i64,
	pub category: String,
	pub district_abbreviation: Option<String>,
	pub data: InsightV2LeaderboardData,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum InsightV2StreakDataEntriesItemKeyType {
	Team,
	Event,
	Match,
	TeamPair,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct InsightV2StreakDataEntriesItem {
	pub key: String,
	pub key_type: InsightV2StreakDataEntriesItemKeyType,
	pub streak_length: i64,
	pub start: String,
	pub end: String,
	pub is_active: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct InsightV2StreakData {
	pub entries: Vec<InsightV2StreakDataEntriesItem>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct InsightV2Streak {
	pub name: String,
	pub display_name: String,
	pub year: i64,
	pub category: String,
	pub district_abbreviation: Option<String>,
	pub data: InsightV2StreakData,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum InsightV2TimeseriesDataXType {
	Week,
	Year,
	Event,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum InsightV2TimeseriesDataPointContextType {
	None,
	MatchRecord,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum InsightV2TimeseriesDataSeriesItemPointsItemX {
	String(String),
	I64(i64),
	F64(f64),
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct InsightV2TimeseriesDataSeriesItemPointsItemContext {
	pub match_key: Option<String>,
	pub alliance: Option<Vec<String>>,
	pub post_result_time: Option<i64>,
	pub is_current: Option<bool>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct InsightV2TimeseriesDataSeriesItemPointsItem {
	pub x: InsightV2TimeseriesDataSeriesItemPointsItemX,
	pub y: f64,
	pub context: Option<InsightV2TimeseriesDataSeriesItemPointsItemContext>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct InsightV2TimeseriesDataSeriesItem {
	pub label: String,
	pub points: Vec<InsightV2TimeseriesDataSeriesItemPointsItem>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct InsightV2TimeseriesData {
	pub x_type: InsightV2TimeseriesDataXType,
	pub x_label: String,
	pub y_label: String,
	pub point_context_type: InsightV2TimeseriesDataPointContextType,
	pub series: Vec<InsightV2TimeseriesDataSeriesItem>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct InsightV2Timeseries {
	pub name: String,
	pub display_name: String,
	pub year: i64,
	pub category: String,
	pub district_abbreviation: Option<String>,
	pub data: InsightV2TimeseriesData,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum InsightV2 {
	InsightV2Leaderboard(InsightV2Leaderboard),
	InsightV2Streak(InsightV2Streak),
	InsightV2Timeseries(InsightV2Timeseries),
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum InsightV2BaseCategory {
	Leaderboard,
	Streak,
	Timeseries,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct InsightV2Base {
	pub name: String,
	pub display_name: String,
	pub year: i64,
	pub category: InsightV2BaseCategory,
	pub district_abbreviation: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct InsightV2LeaderboardExtras {
	pub category: Option<String>,
	pub data: InsightV2LeaderboardData,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct InsightV2StreakExtras {
	pub category: Option<String>,
	pub data: InsightV2StreakData,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct InsightV2TimeseriesExtras {
	pub category: Option<String>,
	pub data: InsightV2TimeseriesData,
}
