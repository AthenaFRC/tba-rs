// This file is generated from The Blue Alliance OpenAPI v3 spec.
// Regenerate it from the TBA API v3 OpenAPI spec when the API changes:
// https://www.thebluealliance.com/swagger/api_v3.json

#![allow(clippy::struct_excessive_bools)]

use serde::{
	Deserialize,
	Serialize,
};

pub type UnknownJsonObject =
	std::collections::HashMap<String, serde_json::Value>;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct APIError {
	#[serde(rename = "Error")]
	pub error: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct APIStatusAppVersion {
	pub min_app_version: i64,
	pub latest_app_version: i64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct APIStatus {
	pub current_season: i64,
	pub max_season: i64,
	pub is_datafeed_down: bool,
	pub down_events: Vec<String>,
	pub ios: APIStatusAppVersion,
	pub android: APIStatusAppVersion,
	pub max_team_page: i64,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum AllianceColor {
	Red,
	Blue,
	#[serde(rename = "")]
	Empty,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub enum AutoChargeStationRobot2023 {
	Docked,
	None,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub enum AutoLineRobot2024 {
	No,
	Yes,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub enum AutoRobot2018 {
	None,
	AutoRun,
}

pub type AwardType = i64;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AwardRecipient {
	pub team_key: Option<String>,
	pub awardee: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Award {
	pub name: String,
	pub award_type: AwardType,
	pub event_key: String,
	pub recipient_list: Vec<AwardRecipient>,
	pub year: i64,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub enum Bay2019 {
	None,
	Panel,
	PanelAndCargo,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub enum BridgeState2023 {
	Level,
	NotLevel,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub enum CompLevel {
	#[serde(rename = "qm")]
	QualificationMatch,
	#[serde(rename = "ef")]
	EighthFinal,
	#[serde(rename = "qf")]
	QuarterFinal,
	#[serde(rename = "sf")]
	SemiFinal,
	#[serde(rename = "f")]
	Final,
}

pub type EventType = i64;

pub type PlayoffType = i64;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DistrictOfficialAdvancementCounts {
	pub dcmp: i64,
	pub cmp: i64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct District {
	pub abbreviation: String,
	pub display_name: String,
	pub key: String,
	pub year: i64,
	pub official_advancement_counts: DistrictOfficialAdvancementCounts,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DistrictInsightRegionData {
	pub yearly_active_team_count: std::collections::HashMap<String, i64>,
	pub yearly_event_count: std::collections::HashMap<String, i64>,
	pub yearly_gained_teams: std::collections::HashMap<String, Vec<String>>,
	pub yearly_lost_teams: std::collections::HashMap<String, Vec<String>>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DistrictAdvancement {
	pub dcmp: bool,
	pub cmp: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DistrictDCMPHistoryEntry {
	pub awards: Option<Vec<Award>>,
	pub event: Option<Event>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DistrictInsightDistrictData {
	pub region_data:
		Option<std::collections::HashMap<String, DistrictInsightRegionData>>,
	pub district_wide_data: Option<DistrictInsightRegionData>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WLTRecord {
	pub losses: i64,
	pub wins: i64,
	pub ties: i64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DistrictInsightTeamDataValue {
	pub district_seasons: i64,
	pub total_district_points: i64,
	pub total_pre_dcmp_district_points: i64,
	pub district_event_wins: i64,
	pub dcmp_wins: i64,
	pub team_awards: i64,
	pub individual_awards: i64,
	pub quals_record: WLTRecord,
	pub elims_record: WLTRecord,
	pub in_district_extra_play_count: i64,
	pub total_matches_played: i64,
	pub dcmp_appearances: i64,
	pub cmp_appearances: i64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DistrictInsight {
	pub district_data: DistrictInsightDistrictData,
	pub team_data:
		Option<std::collections::HashMap<String, DistrictInsightTeamDataValue>>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DistrictRankingEventPointsItem {
	pub district_cmp: bool,
	pub total: i64,
	pub alliance_points: i64,
	pub elim_points: i64,
	pub award_points: i64,
	pub event_key: String,
	pub qual_points: i64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DistrictRanking {
	pub team_key: String,
	pub rank: i64,
	pub rookie_bonus: i64,
	pub point_total: i64,
	pub event_points: Vec<DistrictRankingEventPointsItem>,
	pub adjustments: Option<i64>,
	pub other_bonus: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub enum DoubleElimRound {
	Finals,
	#[serde(rename = "Round 1")]
	Round1,
	#[serde(rename = "Round 2")]
	Round2,
	#[serde(rename = "Round 3")]
	Round3,
	#[serde(rename = "Round 4")]
	Round4,
	#[serde(rename = "Round 5")]
	Round5,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EliminationAllianceBackup {
	#[serde(rename = "in")]
	pub in_: String,
	pub out: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum EliminationAllianceStatusStatus {
	Eliminated,
	Playing,
	Won,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EliminationAllianceStatus {
	pub playoff_average: Option<f64>,
	pub playoff_type: Option<PlayoffType>,
	pub level: CompLevel,
	pub record: Option<WLTRecord>,
	pub current_level_record: Option<WLTRecord>,
	pub status: EliminationAllianceStatusStatus,
	pub advanced_to_round_robin_finals: Option<bool>,
	pub double_elim_round: Option<DoubleElimRound>,
	pub round_robin_rank: Option<i64>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EliminationAlliance {
	pub name: Option<String>,
	pub backup: Option<EliminationAllianceBackup>,
	pub declines: Vec<String>,
	pub picks: Vec<String>,
	pub status: Option<EliminationAllianceStatus>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub enum EndGameChargeStationRobot2023 {
	Docked,
	None,
	Park,
	Parked,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub enum EndGameRobot2024 {
	CenterStage,
	None,
	Parked,
	StageLeft,
	StageRight,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub enum EndGameRobot2025 {
	DeepCage,
	None,
	Parked,
	ShallowCage,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub enum TowerRobot2026 {
	Level1,
	Level2,
	Level3,
	None,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HubScore2026 {
	pub auto_count: i64,
	pub auto_points: i64,
	pub endgame_count: i64,
	pub endgame_points: i64,
	pub shift1_count: i64,
	pub shift1_points: i64,
	pub shift2_count: i64,
	pub shift2_points: i64,
	pub shift3_count: i64,
	pub shift3_points: i64,
	pub shift4_count: i64,
	pub shift4_points: i64,
	pub teleop_count: i64,
	pub teleop_points: i64,
	pub total_count: i64,
	pub total_points: i64,
	pub transition_count: i64,
	pub transition_points: i64,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub enum EndgameRobot2018 {
	Climbing,
	Levitate,
	None,
	Parking,
	Unknown,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub enum EndgameRobot2019 {
	HabLevel1,
	HabLevel2,
	HabLevel3,
	None,
	Unknown,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub enum EndgameRobot2020 {
	Hang,
	None,
	Park,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub enum EndgameRobot2022 {
	High,
	Low,
	Mid,
	None,
	Traversal,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub enum EndgameRungIsLevel2020 {
	IsLevel,
	NotLevel,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum WebcastType {
	Youtube,
	Twitch,
	Ustream,
	Iframe,
	Html5,
	Rtmp,
	Livestream,
	DirectLink,
	Mms,
	Justin,
	Stemtv,
	Dacast,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum WebcastStatus {
	Unknown,
	Online,
	Offline,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Webcast {
	#[serde(rename = "type")]
	pub type_: WebcastType,
	pub channel: String,
	pub date: Option<String>,
	pub file: Option<String>,
	pub status: Option<WebcastStatus>,
	pub stream_title: Option<String>,
	pub viewer_count: Option<i64>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Event {
	pub key: String,
	pub name: String,
	pub event_code: String,
	pub event_type: EventType,
	pub district: Option<District>,
	pub city: Option<String>,
	pub state_prov: Option<String>,
	pub country: Option<String>,
	pub start_date: String,
	pub end_date: String,
	pub year: i64,
	pub short_name: Option<String>,
	pub event_type_string: String,
	pub week: Option<i64>,
	pub address: Option<String>,
	pub postal_code: Option<String>,
	pub gmaps_place_id: Option<String>,
	pub gmaps_url: Option<String>,
	pub lat: Option<f64>,
	pub lng: Option<f64>,
	pub location_name: Option<String>,
	pub timezone: Option<String>,
	pub website: Option<String>,
	pub first_event_id: Option<String>,
	pub first_event_code: Option<String>,
	pub webcasts: Vec<Webcast>,
	pub division_keys: Vec<String>,
	pub parent_event_key: Option<String>,
	pub playoff_type: Option<PlayoffType>,
	pub playoff_type_string: Option<String>,
	pub remap_teams: Option<std::collections::HashMap<String, String>>,
}

pub type EventCOPRs =
	std::collections::HashMap<String, std::collections::HashMap<String, f64>>;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EventDistrictPointsPointsValue {
	pub total: i64,
	pub alliance_points: i64,
	pub elim_points: i64,
	pub award_points: i64,
	pub qual_points: i64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EventDistrictPointsTiebreakersValue {
	pub highest_match_scores: Option<Vec<i64>>,
	pub qual_wins: Option<i64>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EventDistrictPoints {
	pub points:
		std::collections::HashMap<String, EventDistrictPointsPointsValue>,
	pub tiebreakers: Option<
		std::collections::HashMap<String, EventDistrictPointsTiebreakersValue>,
	>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EventInsights {
	pub qual: Option<UnknownJsonObject>,
	pub playoff: Option<UnknownJsonObject>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EventInsights2016 {
	#[serde(rename = "LowBar")]
	pub low_bar: Vec<f64>,
	#[serde(rename = "A_ChevalDeFrise")]
	pub a_cheval_de_frise: Vec<f64>,
	#[serde(rename = "A_Portcullis")]
	pub a_portcullis: Vec<f64>,
	#[serde(rename = "B_Ramparts")]
	pub b_ramparts: Vec<f64>,
	#[serde(rename = "B_Moat")]
	pub b_moat: Vec<f64>,
	#[serde(rename = "C_SallyPort")]
	pub c_sally_port: Vec<f64>,
	#[serde(rename = "C_Drawbridge")]
	pub c_drawbridge: Vec<f64>,
	#[serde(rename = "D_RoughTerrain")]
	pub d_rough_terrain: Vec<f64>,
	#[serde(rename = "D_RockWall")]
	pub d_rock_wall: Vec<f64>,
	pub average_high_goals: f64,
	pub average_low_goals: f64,
	pub breaches: Vec<f64>,
	pub scales: Vec<f64>,
	pub challenges: Vec<f64>,
	pub captures: Vec<f64>,
	pub average_win_score: f64,
	pub average_win_margin: f64,
	pub average_score: f64,
	pub average_auto_score: f64,
	pub average_crossing_score: f64,
	pub average_boulder_score: f64,
	pub average_tower_score: f64,
	pub average_foul_score: f64,
	pub high_score: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EventInsights2017 {
	pub average_foul_score: f64,
	pub average_fuel_points: f64,
	pub average_fuel_points_auto: f64,
	pub average_fuel_points_teleop: f64,
	pub average_high_goals: f64,
	pub average_high_goals_auto: f64,
	pub average_high_goals_teleop: f64,
	pub average_low_goals: f64,
	pub average_low_goals_auto: f64,
	pub average_low_goals_teleop: f64,
	pub average_mobility_points_auto: f64,
	pub average_points_auto: f64,
	pub average_points_teleop: f64,
	pub average_rotor_points: f64,
	pub average_rotor_points_auto: f64,
	pub average_rotor_points_teleop: f64,
	pub average_score: f64,
	pub average_takeoff_points_teleop: f64,
	pub average_win_margin: f64,
	pub average_win_score: f64,
	pub high_kpa: Vec<String>,
	pub high_score: Vec<String>,
	pub kpa_achieved: Vec<f64>,
	pub mobility_counts: Vec<f64>,
	pub rotor_1_engaged: Vec<f64>,
	pub rotor_1_engaged_auto: Vec<f64>,
	pub rotor_2_engaged: Vec<f64>,
	pub rotor_2_engaged_auto: Vec<f64>,
	pub rotor_3_engaged: Vec<f64>,
	pub rotor_4_engaged: Vec<f64>,
	pub takeoff_counts: Vec<f64>,
	pub unicorn_matches: Vec<f64>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EventInsights2018 {
	pub auto_quest_achieved: Vec<f64>,
	pub average_boost_played: f64,
	pub average_endgame_points: f64,
	pub average_force_played: f64,
	pub average_foul_score: f64,
	pub average_points_auto: f64,
	pub average_points_teleop: f64,
	pub average_run_points_auto: f64,
	pub average_scale_ownership_points: f64,
	pub average_scale_ownership_points_auto: f64,
	pub average_scale_ownership_points_teleop: f64,
	pub average_score: f64,
	pub average_switch_ownership_points: f64,
	pub average_switch_ownership_points_auto: f64,
	pub average_switch_ownership_points_teleop: f64,
	pub average_vault_points: f64,
	pub average_win_margin: f64,
	pub average_win_score: f64,
	pub boost_played_counts: Vec<f64>,
	pub climb_counts: Vec<f64>,
	pub face_the_boss_achieved: Vec<f64>,
	pub force_played_counts: Vec<f64>,
	pub high_score: Vec<String>,
	pub levitate_played_counts: Vec<f64>,
	pub run_counts_auto: Vec<f64>,
	pub scale_neutral_percentage: f64,
	pub scale_neutral_percentage_auto: f64,
	pub scale_neutral_percentage_teleop: f64,
	pub switch_owned_counts_auto: Vec<f64>,
	pub unicorn_matches: Vec<f64>,
	pub winning_opp_switch_denial_percentage_teleop: f64,
	pub winning_own_switch_ownership_percentage: f64,
	pub winning_own_switch_ownership_percentage_auto: f64,
	pub winning_own_switch_ownership_percentage_teleop: f64,
	pub winning_scale_ownership_percentage: f64,
	pub winning_scale_ownership_percentage_auto: f64,
	pub winning_scale_ownership_percentage_teleop: f64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EventOPRs {
	pub oprs: Option<std::collections::HashMap<String, f64>>,
	pub dprs: Option<std::collections::HashMap<String, f64>>,
	pub ccwms: Option<std::collections::HashMap<String, f64>>,
}

pub type EventPredictions = UnknownJsonObject;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EventRankingRankingsItem {
	pub matches_played: i64,
	pub qual_average: Option<f64>,
	pub extra_stats: Vec<f64>,
	pub sort_orders: Vec<f64>,
	pub record: Option<WLTRecord>,
	pub rank: i64,
	pub dq: i64,
	pub team_key: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EventRankingExtraStatsInfoItem {
	pub precision: f64,
	pub name: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EventRankingSortOrderInfoItem {
	pub precision: i64,
	pub name: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EventRanking {
	pub rankings: Vec<EventRankingRankingsItem>,
	pub extra_stats_info: Vec<EventRankingExtraStatsInfoItem>,
	pub sort_order_info: Option<Vec<EventRankingSortOrderInfoItem>>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EventSimple {
	pub key: String,
	pub name: String,
	pub event_code: String,
	pub event_type: EventType,
	pub district: Option<District>,
	pub city: Option<String>,
	pub state_prov: Option<String>,
	pub country: Option<String>,
	pub start_date: String,
	pub end_date: String,
	pub year: i64,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub enum HabLine2019 {
	CrossedHabLineInSandstorm,
	CrossedHabLineInTeleop,
	None,
	Unknown,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct History {
	pub events: Vec<Event>,
	pub awards: Vec<Award>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub enum InitLineRobot2020 {
	Exited,
	None,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LeaderboardInsightDataRankingsItem {
	pub value: f64,
	pub keys: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum LeaderboardInsightDataKeyType {
	Team,
	Event,
	Match,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LeaderboardInsightData {
	pub rankings: Vec<LeaderboardInsightDataRankingsItem>,
	pub key_type: LeaderboardInsightDataKeyType,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LeaderboardInsight {
	pub data: LeaderboardInsightData,
	pub name: String,
	pub year: i64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MatchAlliance {
	pub score: i64,
	pub team_keys: Vec<String>,
	pub surrogate_team_keys: Vec<String>,
	pub dq_team_keys: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MatchAlliances {
	pub red: MatchAlliance,
	pub blue: MatchAlliance,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MatchScoreBreakdown2015Alliance {
	pub auto: Option<String>,
	pub auto_points: Option<i64>,
	pub teleop_points: Option<i64>,
	pub container_points: Option<i64>,
	pub tote_points: Option<i64>,
	pub litter_points: Option<i64>,
	pub foul: Option<String>,
	pub foul_points: Option<i64>,
	pub adjust_points: Option<i64>,
	pub total_points: Option<i64>,
	pub foul_count: Option<i64>,
	pub tote_count_far: Option<i64>,
	pub tote_count_near: Option<i64>,
	pub tote_set: Option<bool>,
	pub tote_stack: Option<bool>,
	pub container_count_level1: Option<i64>,
	pub container_count_level2: Option<i64>,
	pub container_count_level3: Option<i64>,
	pub container_count_level4: Option<i64>,
	pub container_count_level5: Option<i64>,
	pub container_count_level6: Option<i64>,
	pub container_set: Option<bool>,
	pub litter_count_container: Option<i64>,
	pub litter_count_landfill: Option<i64>,
	pub litter_count_unprocessed: Option<i64>,
	pub robot_set: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub enum MatchScoreBreakdown2015Coopertition {
	None,
	Unknown,
	Stack,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MatchScoreBreakdown2015 {
	pub blue: MatchScoreBreakdown2015Alliance,
	pub red: MatchScoreBreakdown2015Alliance,
	pub coopertition: MatchScoreBreakdown2015Coopertition,
	pub coopertition_points: i64,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub enum RobotAuto2016WithUnknown {
	Crossed,
	None,
	Reached,
	Unknown,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub enum RobotAuto2016WithoutUnknown {
	Crossed,
	Reached,
	None,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub enum TowerFace2016 {
	Both,
	Challenged,
	None,
	Scaled,
	Unknown,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub enum Position2016 {
	#[serde(rename = "")]
	Empty,
	#[serde(rename = "A_ChevalDeFrise")]
	AChevalDeFrise,
	#[serde(rename = "A_Portcullis")]
	APortcullis,
	#[serde(rename = "B_Moat")]
	BMoat,
	#[serde(rename = "B_Ramparts")]
	BRamparts,
	#[serde(rename = "C_Drawbridge")]
	CDrawbridge,
	#[serde(rename = "C_SallyPort")]
	CSallyPort,
	#[serde(rename = "D_RockWall")]
	DRockWall,
	#[serde(rename = "D_RoughTerrain")]
	DRoughTerrain,
	NotSpecified,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MatchScoreBreakdown2016Alliance {
	pub auto_points: i64,
	pub teleop_points: Option<i64>,
	pub breach_points: i64,
	pub foul_points: i64,
	pub capture_points: i64,
	pub adjust_points: Option<i64>,
	pub total_points: i64,
	#[serde(rename = "tba_rpEarned")]
	pub tba_rp_earned: Option<i64>,
	pub robot1_auto: Option<RobotAuto2016WithUnknown>,
	pub robot2_auto: Option<RobotAuto2016WithoutUnknown>,
	pub robot3_auto: Option<RobotAuto2016WithUnknown>,
	pub auto_reach_points: i64,
	pub auto_crossing_points: i64,
	pub auto_boulders_low: Option<i64>,
	pub auto_boulders_high: Option<i64>,
	pub auto_boulder_points: i64,
	pub teleop_crossing_points: i64,
	pub teleop_boulders_low: i64,
	pub teleop_boulders_high: i64,
	pub teleop_boulder_points: i64,
	pub teleop_defenses_breached: bool,
	pub teleop_challenge_points: i64,
	pub teleop_scale_points: i64,
	pub teleop_tower_captured: bool,
	pub tower_face_a: Option<TowerFace2016>,
	pub tower_face_b: Option<TowerFace2016>,
	pub tower_face_c: Option<TowerFace2016>,
	pub tower_end_strength: Option<i64>,
	pub tech_foul_count: Option<i64>,
	pub foul_count: Option<i64>,
	pub position2: Position2016,
	pub position3: Position2016,
	pub position4: Position2016,
	pub position5: Position2016,
	pub position1crossings: i64,
	pub position2crossings: i64,
	pub position3crossings: i64,
	pub position4crossings: i64,
	pub position5crossings: i64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MatchScoreBreakdown2016 {
	pub blue: MatchScoreBreakdown2016Alliance,
	pub red: MatchScoreBreakdown2016Alliance,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub enum RobotAuto2017 {
	Mobility,
	None,
	Unknown,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub enum Touchpad2017 {
	None,
	ReadyForTakeoff,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MatchScoreBreakdown2017Alliance {
	pub auto_points: i64,
	pub teleop_points: i64,
	pub foul_points: i64,
	pub adjust_points: Option<i64>,
	pub total_points: i64,
	pub robot1_auto: Option<RobotAuto2017>,
	pub robot2_auto: Option<RobotAuto2017>,
	pub robot3_auto: Option<RobotAuto2017>,
	pub rotor1_auto: bool,
	pub rotor2_auto: bool,
	pub auto_fuel_low: i64,
	pub auto_fuel_high: i64,
	pub auto_mobility_points: i64,
	pub auto_rotor_points: i64,
	pub auto_fuel_points: i64,
	pub teleop_fuel_points: i64,
	pub teleop_fuel_low: i64,
	pub teleop_fuel_high: i64,
	pub teleop_rotor_points: i64,
	pub k_pa_ranking_point_achieved: bool,
	pub teleop_takeoff_points: i64,
	pub k_pa_bonus_points: i64,
	pub rotor_bonus_points: i64,
	pub rotor1_engaged: bool,
	pub rotor2_engaged: bool,
	pub rotor3_engaged: bool,
	pub rotor4_engaged: bool,
	pub rotor_ranking_point_achieved: bool,
	#[serde(rename = "tba_rpEarned")]
	pub tba_rp_earned: Option<i64>,
	pub tech_foul_count: Option<i64>,
	pub foul_count: Option<i64>,
	pub touchpad_near: Option<Touchpad2017>,
	pub touchpad_middle: Option<Touchpad2017>,
	pub touchpad_far: Option<Touchpad2017>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MatchScoreBreakdown2017 {
	pub blue: MatchScoreBreakdown2017Alliance,
	pub red: MatchScoreBreakdown2017Alliance,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub enum MatchScoreBreakdown2018AllianceTbaGameData {
	#[serde(rename = "")]
	Empty,
	LLL,
	LRL,
	RLR,
	RRR,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MatchScoreBreakdown2018Alliance {
	pub adjust_points: Option<i64>,
	pub auto_ownership_points: i64,
	pub auto_points: i64,
	pub auto_quest_ranking_point: Option<bool>,
	pub auto_robot1: Option<AutoRobot2018>,
	pub auto_robot2: Option<AutoRobot2018>,
	pub auto_robot3: Option<AutoRobot2018>,
	pub auto_run_points: i64,
	pub auto_scale_ownership_sec: i64,
	pub auto_switch_at_zero: Option<bool>,
	pub auto_switch_ownership_sec: i64,
	pub endgame_points: i64,
	pub endgame_robot1: Option<EndgameRobot2018>,
	pub endgame_robot2: Option<EndgameRobot2018>,
	pub endgame_robot3: Option<EndgameRobot2018>,
	pub face_the_boss_ranking_point: bool,
	pub foul_count: Option<i64>,
	pub foul_points: i64,
	pub rp: f64,
	pub tech_foul_count: Option<i64>,
	pub teleop_ownership_points: i64,
	pub teleop_points: i64,
	pub teleop_scale_boost_sec: i64,
	pub teleop_scale_force_sec: Option<i64>,
	pub teleop_scale_ownership_sec: i64,
	pub teleop_switch_boost_sec: i64,
	pub teleop_switch_force_sec: Option<i64>,
	pub teleop_switch_ownership_sec: i64,
	pub total_points: i64,
	pub vault_boost_played: i64,
	pub vault_boost_total: i64,
	pub vault_force_played: i64,
	pub vault_force_total: i64,
	pub vault_levitate_played: i64,
	pub vault_levitate_total: i64,
	pub vault_points: i64,
	#[serde(rename = "tba_gameData")]
	pub tba_game_data: Option<MatchScoreBreakdown2018AllianceTbaGameData>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MatchScoreBreakdown2018 {
	pub blue: MatchScoreBreakdown2018Alliance,
	pub red: MatchScoreBreakdown2018Alliance,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub enum PreMatchBay2019 {
	Cargo,
	Panel,
	Unknown,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MatchScoreBreakdown2019Alliance {
	pub adjust_points: Option<i64>,
	pub auto_points: Option<i64>,
	pub bay1: Bay2019,
	pub bay2: Bay2019,
	pub bay3: Bay2019,
	pub bay4: Bay2019,
	pub bay5: Bay2019,
	pub bay6: Bay2019,
	pub bay7: Bay2019,
	pub bay8: Bay2019,
	pub cargo_points: i64,
	pub complete_rocket_ranking_point: bool,
	pub completed_rocket_far: Option<bool>,
	pub completed_rocket_near: Option<bool>,
	pub endgame_robot1: EndgameRobot2019,
	pub endgame_robot2: EndgameRobot2019,
	pub endgame_robot3: EndgameRobot2019,
	pub foul_count: Option<i64>,
	pub foul_points: i64,
	pub hab_climb_points: i64,
	pub hab_docking_ranking_point: bool,
	pub hab_line_robot1: HabLine2019,
	pub hab_line_robot2: HabLine2019,
	pub hab_line_robot3: HabLine2019,
	pub hatch_panel_points: i64,
	pub low_left_rocket_far: Bay2019,
	pub low_left_rocket_near: Bay2019,
	pub low_right_rocket_far: Bay2019,
	pub low_right_rocket_near: Bay2019,
	pub mid_left_rocket_far: Bay2019,
	pub mid_left_rocket_near: Bay2019,
	pub mid_right_rocket_far: Bay2019,
	pub mid_right_rocket_near: Bay2019,
	pub pre_match_bay1: PreMatchBay2019,
	pub pre_match_bay2: PreMatchBay2019,
	pub pre_match_bay3: PreMatchBay2019,
	pub pre_match_bay6: PreMatchBay2019,
	pub pre_match_bay7: PreMatchBay2019,
	pub pre_match_bay8: PreMatchBay2019,
	pub pre_match_level_robot1: EndgameRobot2019,
	pub pre_match_level_robot2: EndgameRobot2019,
	pub pre_match_level_robot3: EndgameRobot2019,
	pub rp: f64,
	pub sand_storm_bonus_points: i64,
	pub tech_foul_count: Option<i64>,
	pub teleop_points: i64,
	pub top_left_rocket_far: Bay2019,
	pub top_left_rocket_near: Bay2019,
	pub top_right_rocket_far: Bay2019,
	pub top_right_rocket_near: Bay2019,
	pub total_points: i64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MatchScoreBreakdown2019 {
	pub blue: MatchScoreBreakdown2019Alliance,
	pub red: MatchScoreBreakdown2019Alliance,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub enum Stage3TargetColor2020 {
	Blue,
	Green,
	Red,
	Unknown,
	Yellow,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MatchScoreBreakdown2020Alliance {
	pub init_line_robot1: InitLineRobot2020,
	pub endgame_robot1: EndgameRobot2020,
	pub init_line_robot2: InitLineRobot2020,
	pub endgame_robot2: EndgameRobot2020,
	pub init_line_robot3: InitLineRobot2020,
	pub endgame_robot3: EndgameRobot2020,
	pub auto_cells_bottom: i64,
	pub auto_cells_outer: i64,
	pub auto_cells_inner: i64,
	pub teleop_cells_bottom: i64,
	pub teleop_cells_outer: i64,
	pub teleop_cells_inner: i64,
	pub stage1_activated: bool,
	pub stage2_activated: bool,
	pub stage3_activated: bool,
	pub stage3_target_color: Stage3TargetColor2020,
	pub endgame_rung_is_level: EndgameRungIsLevel2020,
	pub auto_init_line_points: i64,
	pub auto_cell_points: i64,
	pub auto_points: i64,
	pub teleop_cell_points: i64,
	pub control_panel_points: i64,
	pub endgame_points: i64,
	pub teleop_points: i64,
	pub shield_operational_ranking_point: bool,
	pub shield_energized_ranking_point: bool,
	#[serde(rename = "tba_shieldEnergizedRankingPointFromFoul")]
	pub tba_shield_energized_ranking_point_from_foul: Option<bool>,
	#[serde(rename = "tba_numRobotsHanging")]
	pub tba_num_robots_hanging: Option<i64>,
	pub foul_count: i64,
	pub tech_foul_count: i64,
	pub adjust_points: Option<i64>,
	pub foul_points: i64,
	pub rp: Option<f64>,
	pub total_points: i64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MatchScoreBreakdown2020 {
	pub blue: MatchScoreBreakdown2020Alliance,
	pub red: MatchScoreBreakdown2020Alliance,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub enum TaxiRobot2022 {
	No,
	Yes,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MatchScoreBreakdown2022Alliance {
	pub taxi_robot1: Option<TaxiRobot2022>,
	pub endgame_robot1: Option<EndgameRobot2022>,
	pub taxi_robot2: Option<TaxiRobot2022>,
	pub endgame_robot2: Option<EndgameRobot2022>,
	pub taxi_robot3: Option<TaxiRobot2022>,
	pub endgame_robot3: Option<EndgameRobot2022>,
	pub auto_cargo_lower_near: Option<i64>,
	pub auto_cargo_lower_far: Option<i64>,
	pub auto_cargo_lower_blue: Option<i64>,
	pub auto_cargo_lower_red: Option<i64>,
	pub auto_cargo_upper_near: Option<i64>,
	pub auto_cargo_upper_far: Option<i64>,
	pub auto_cargo_upper_blue: Option<i64>,
	pub auto_cargo_upper_red: Option<i64>,
	pub auto_cargo_total: Option<i64>,
	pub teleop_cargo_lower_near: Option<i64>,
	pub teleop_cargo_lower_far: Option<i64>,
	pub teleop_cargo_lower_blue: Option<i64>,
	pub teleop_cargo_lower_red: Option<i64>,
	pub teleop_cargo_upper_near: Option<i64>,
	pub teleop_cargo_upper_far: Option<i64>,
	pub teleop_cargo_upper_blue: Option<i64>,
	pub teleop_cargo_upper_red: Option<i64>,
	pub teleop_cargo_total: Option<i64>,
	pub match_cargo_total: Option<i64>,
	pub auto_taxi_points: Option<i64>,
	pub auto_cargo_points: Option<i64>,
	pub auto_points: Option<i64>,
	pub quintet_achieved: Option<bool>,
	pub teleop_cargo_points: Option<i64>,
	pub endgame_points: Option<i64>,
	pub teleop_points: Option<i64>,
	pub cargo_bonus_ranking_point: Option<bool>,
	pub hangar_bonus_ranking_point: Option<bool>,
	pub foul_count: Option<i64>,
	pub tech_foul_count: Option<i64>,
	pub adjust_points: Option<i64>,
	pub foul_points: Option<i64>,
	pub rp: Option<f64>,
	pub total_points: Option<i64>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MatchScoreBreakdown2022 {
	pub blue: MatchScoreBreakdown2022Alliance,
	pub red: MatchScoreBreakdown2022Alliance,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub enum MatchScoreBreakdown2023AllianceAutoCommunityBItem {
	None,
	Cone,
	Cube,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub enum MatchScoreBreakdown2023AllianceAutoCommunityMItem {
	None,
	Cone,
	Cube,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub enum MatchScoreBreakdown2023AllianceAutoCommunityTItem {
	None,
	Cone,
	Cube,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "UPPERCASE")]
pub struct MatchScoreBreakdown2023AllianceAutoCommunity {
	pub b: Vec<MatchScoreBreakdown2023AllianceAutoCommunityBItem>,
	pub m: Vec<MatchScoreBreakdown2023AllianceAutoCommunityMItem>,
	pub t: Vec<MatchScoreBreakdown2023AllianceAutoCommunityTItem>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub enum MobilityRobot2023 {
	No,
	Yes,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub enum MatchScoreBreakdown2023AllianceLinksItemNodesItem {
	None,
	Cone,
	Cube,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub enum MatchScoreBreakdown2023AllianceLinksItemRow {
	Bottom,
	Mid,
	Top,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MatchScoreBreakdown2023AllianceLinksItem {
	pub nodes: Vec<MatchScoreBreakdown2023AllianceLinksItemNodesItem>,
	pub row: MatchScoreBreakdown2023AllianceLinksItemRow,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub enum MatchScoreBreakdown2023AllianceTeleopCommunityBItem {
	None,
	Cone,
	Cube,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub enum MatchScoreBreakdown2023AllianceTeleopCommunityMItem {
	None,
	Cone,
	Cube,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub enum MatchScoreBreakdown2023AllianceTeleopCommunityTItem {
	None,
	Cone,
	Cube,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "UPPERCASE")]
pub struct MatchScoreBreakdown2023AllianceTeleopCommunity {
	pub b: Vec<MatchScoreBreakdown2023AllianceTeleopCommunityBItem>,
	pub m: Vec<MatchScoreBreakdown2023AllianceTeleopCommunityMItem>,
	pub t: Vec<MatchScoreBreakdown2023AllianceTeleopCommunityTItem>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MatchScoreBreakdown2023Alliance {
	pub activation_bonus_achieved: Option<bool>,
	pub adjust_points: Option<i64>,
	pub auto_bridge_state: Option<BridgeState2023>,
	pub auto_charge_station_points: Option<i64>,
	pub auto_charge_station_robot1: Option<AutoChargeStationRobot2023>,
	pub auto_charge_station_robot2: Option<AutoChargeStationRobot2023>,
	pub auto_charge_station_robot3: Option<AutoChargeStationRobot2023>,
	pub auto_docked: Option<bool>,
	pub auto_community: Option<MatchScoreBreakdown2023AllianceAutoCommunity>,
	pub auto_game_piece_count: Option<i64>,
	pub auto_game_piece_points: Option<i64>,
	pub auto_mobility_points: i64,
	pub mobility_robot1: MobilityRobot2023,
	pub mobility_robot2: MobilityRobot2023,
	pub mobility_robot3: MobilityRobot2023,
	pub auto_points: i64,
	pub coop_game_piece_count: Option<i64>,
	pub coopertition_criteria_met: Option<bool>,
	pub end_game_bridge_state: Option<BridgeState2023>,
	pub end_game_charge_station_points: Option<i64>,
	pub end_game_charge_station_robot1: Option<EndGameChargeStationRobot2023>,
	pub end_game_charge_station_robot2: Option<EndGameChargeStationRobot2023>,
	pub end_game_charge_station_robot3: Option<EndGameChargeStationRobot2023>,
	pub end_game_park_points: Option<i64>,
	pub extra_game_piece_count: Option<i64>,
	pub foul_count: i64,
	pub foul_points: i64,
	pub tech_foul_count: i64,
	pub link_points: Option<i64>,
	pub links: Option<Vec<MatchScoreBreakdown2023AllianceLinksItem>>,
	pub sustainability_bonus_achieved: Option<bool>,
	pub teleop_community:
		Option<MatchScoreBreakdown2023AllianceTeleopCommunity>,
	pub teleop_game_piece_count: Option<i64>,
	pub teleop_game_piece_points: Option<i64>,
	pub total_charge_station_points: Option<i64>,
	pub teleop_points: i64,
	pub rp: f64,
	pub total_points: i64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MatchScoreBreakdown2023 {
	pub blue: MatchScoreBreakdown2023Alliance,
	pub red: MatchScoreBreakdown2023Alliance,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MatchScoreBreakdown2024Alliance {
	pub adjust_points: Option<i64>,
	pub auto_amp_note_count: Option<i64>,
	pub auto_amp_note_points: Option<i64>,
	pub auto_leave_points: Option<i64>,
	pub auto_line_robot1: Option<AutoLineRobot2024>,
	pub auto_line_robot2: Option<AutoLineRobot2024>,
	pub auto_line_robot3: Option<AutoLineRobot2024>,
	pub auto_points: Option<i64>,
	pub auto_speaker_note_count: Option<i64>,
	pub auto_speaker_note_points: Option<i64>,
	pub auto_total_note_points: Option<i64>,
	pub coop_note_played: Option<bool>,
	pub coopertition_bonus_achieved: Option<bool>,
	pub coopertition_criteria_met: Option<bool>,
	pub end_game_harmony_points: Option<i64>,
	pub end_game_note_in_trap_points: Option<i64>,
	pub end_game_on_stage_points: Option<i64>,
	pub end_game_park_points: Option<i64>,
	pub end_game_robot1: Option<EndGameRobot2024>,
	pub end_game_robot2: Option<EndGameRobot2024>,
	pub end_game_robot3: Option<EndGameRobot2024>,
	pub end_game_spot_light_bonus_points: Option<i64>,
	pub end_game_total_stage_points: Option<i64>,
	pub ensemble_bonus_achieved: Option<bool>,
	pub ensemble_bonus_on_stage_robots_threshold: Option<i64>,
	pub ensemble_bonus_stage_points_threshold: Option<i64>,
	pub foul_count: Option<i64>,
	pub foul_points: Option<i64>,
	pub g206_penalty: Option<bool>,
	pub g408_penalty: Option<bool>,
	pub g424_penalty: Option<bool>,
	pub melody_bonus_achieved: Option<bool>,
	pub melody_bonus_threshold: Option<i64>,
	pub melody_bonus_threshold_coop: Option<i64>,
	pub melody_bonus_threshold_non_coop: Option<i64>,
	pub mic_center_stage: Option<bool>,
	pub mic_stage_left: Option<bool>,
	pub mic_stage_right: Option<bool>,
	pub rp: f64,
	pub tech_foul_count: Option<i64>,
	pub teleop_amp_note_count: Option<i64>,
	pub teleop_amp_note_points: Option<i64>,
	pub teleop_points: Option<i64>,
	pub teleop_speaker_note_amplified_count: Option<i64>,
	pub teleop_speaker_note_amplified_points: Option<i64>,
	pub teleop_speaker_note_count: Option<i64>,
	pub teleop_speaker_note_points: Option<i64>,
	pub teleop_total_note_points: Option<i64>,
	pub total_points: i64,
	pub trap_center_stage: Option<bool>,
	pub trap_stage_left: Option<bool>,
	pub trap_stage_right: Option<bool>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MatchScoreBreakdown2024 {
	pub blue: MatchScoreBreakdown2024Alliance,
	pub red: MatchScoreBreakdown2024Alliance,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ReefRow2025 {
	pub node_a: bool,
	pub node_b: bool,
	pub node_c: bool,
	pub node_d: bool,
	pub node_e: bool,
	pub node_f: bool,
	pub node_g: bool,
	pub node_h: bool,
	pub node_i: bool,
	pub node_j: bool,
	pub node_k: bool,
	pub node_l: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MatchScoreBreakdown2025AllianceAutoReef {
	pub top_row: ReefRow2025,
	pub mid_row: ReefRow2025,
	pub bot_row: ReefRow2025,
	pub trough: i64,
	#[serde(rename = "tba_botRowCount")]
	pub tba_bot_row_count: Option<i64>,
	#[serde(rename = "tba_midRowCount")]
	pub tba_mid_row_count: Option<i64>,
	#[serde(rename = "tba_topRowCount")]
	pub tba_top_row_count: Option<i64>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MatchScoreBreakdown2025AllianceTeleopReef {
	pub top_row: ReefRow2025,
	pub mid_row: ReefRow2025,
	pub bot_row: ReefRow2025,
	pub trough: i64,
	#[serde(rename = "tba_botRowCount")]
	pub tba_bot_row_count: Option<i64>,
	#[serde(rename = "tba_midRowCount")]
	pub tba_mid_row_count: Option<i64>,
	#[serde(rename = "tba_topRowCount")]
	pub tba_top_row_count: Option<i64>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MatchScoreBreakdown2025Alliance {
	pub adjust_points: Option<i64>,
	pub algae_points: i64,
	pub auto_bonus_achieved: Option<bool>,
	pub auto_coral_count: i64,
	pub auto_coral_points: i64,
	pub auto_line_robot1: AutoLineRobot2024,
	pub auto_line_robot2: AutoLineRobot2024,
	pub auto_line_robot3: AutoLineRobot2024,
	pub auto_mobility_points: i64,
	pub auto_points: i64,
	pub auto_reef: MatchScoreBreakdown2025AllianceAutoReef,
	pub barge_bonus_achieved: Option<bool>,
	pub coopertition_criteria_met: Option<bool>,
	pub coral_bonus_achieved: Option<bool>,
	pub end_game_barge_points: i64,
	pub end_game_robot1: EndGameRobot2025,
	pub end_game_robot2: EndGameRobot2025,
	pub end_game_robot3: EndGameRobot2025,
	pub foul_count: i64,
	pub foul_points: i64,
	pub g206_penalty: bool,
	pub g410_penalty: bool,
	pub g418_penalty: bool,
	pub g428_penalty: bool,
	pub net_algae_count: i64,
	pub rp: f64,
	pub tech_foul_count: i64,
	pub teleop_coral_count: i64,
	pub teleop_coral_points: i64,
	pub teleop_points: i64,
	pub teleop_reef: MatchScoreBreakdown2025AllianceTeleopReef,
	pub total_points: i64,
	pub wall_algae_count: i64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MatchScoreBreakdown2025 {
	pub blue: MatchScoreBreakdown2025Alliance,
	pub red: MatchScoreBreakdown2025Alliance,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MatchScoreBreakdown2026Alliance {
	pub adjust_points: i64,
	pub auto_tower_points: i64,
	pub auto_tower_robot1: TowerRobot2026,
	pub auto_tower_robot2: TowerRobot2026,
	pub auto_tower_robot3: TowerRobot2026,
	pub end_game_tower_points: i64,
	pub end_game_tower_robot1: TowerRobot2026,
	pub end_game_tower_robot2: TowerRobot2026,
	pub end_game_tower_robot3: TowerRobot2026,
	pub energized_achieved: bool,
	pub foul_points: i64,
	pub g206_penalty: bool,
	pub hub_score: HubScore2026,
	pub major_foul_count: i64,
	pub minor_foul_count: i64,
	pub rp: f64,
	pub supercharged_achieved: bool,
	pub total_auto_points: i64,
	pub total_points: i64,
	pub total_teleop_points: i64,
	pub total_tower_points: i64,
	pub traversal_achieved: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MatchScoreBreakdown2026 {
	pub blue: MatchScoreBreakdown2026Alliance,
	pub red: MatchScoreBreakdown2026Alliance,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum MatchScoreBreakdown {
	MatchScoreBreakdown2015(MatchScoreBreakdown2015),
	MatchScoreBreakdown2016(MatchScoreBreakdown2016),
	MatchScoreBreakdown2017(MatchScoreBreakdown2017),
	MatchScoreBreakdown2018(MatchScoreBreakdown2018),
	MatchScoreBreakdown2019(MatchScoreBreakdown2019),
	MatchScoreBreakdown2020(MatchScoreBreakdown2020),
	MatchScoreBreakdown2022(MatchScoreBreakdown2022),
	MatchScoreBreakdown2023(MatchScoreBreakdown2023),
	MatchScoreBreakdown2024(MatchScoreBreakdown2024),
	MatchScoreBreakdown2025(MatchScoreBreakdown2025),
	MatchScoreBreakdown2026(MatchScoreBreakdown2026),
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MatchVideosItem {
	#[serde(rename = "type")]
	pub type_: String,
	pub key: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Match {
	pub key: String,
	pub comp_level: CompLevel,
	pub set_number: i64,
	pub match_number: i64,
	pub alliances: MatchAlliances,
	pub winning_alliance: AllianceColor,
	pub event_key: String,
	pub time: Option<i64>,
	pub actual_time: Option<i64>,
	pub predicted_time: Option<i64>,
	pub post_result_time: Option<i64>,
	pub score_breakdown: Option<MatchScoreBreakdown>,
	pub videos: Vec<MatchVideosItem>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MatchSimpleAlliances {
	pub red: MatchAlliance,
	pub blue: MatchAlliance,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MatchSimple {
	pub key: String,
	pub comp_level: CompLevel,
	pub set_number: i64,
	pub match_number: i64,
	pub alliances: MatchSimpleAlliances,
	pub winning_alliance: AllianceColor,
	pub event_key: String,
	pub time: Option<i64>,
	pub predicted_time: Option<i64>,
	pub actual_time: Option<i64>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MatchTimeseries2018 {
	pub event_key: Option<String>,
	pub match_id: Option<String>,
	pub mode: Option<String>,
	pub play: Option<i64>,
	pub time_remaining: Option<i64>,
	pub blue_auto_quest: Option<i64>,
	pub blue_boost_count: Option<i64>,
	pub blue_boost_played: Option<i64>,
	pub blue_current_powerup: Option<String>,
	pub blue_face_the_boss: Option<i64>,
	pub blue_force_count: Option<i64>,
	pub blue_force_played: Option<i64>,
	pub blue_levitate_count: Option<i64>,
	pub blue_levitate_played: Option<i64>,
	pub blue_powerup_time_remaining: Option<String>,
	pub blue_scale_owned: Option<i64>,
	pub blue_score: Option<i64>,
	pub blue_switch_owned: Option<i64>,
	pub red_auto_quest: Option<i64>,
	pub red_boost_count: Option<i64>,
	pub red_boost_played: Option<i64>,
	pub red_current_powerup: Option<String>,
	pub red_face_the_boss: Option<i64>,
	pub red_force_count: Option<i64>,
	pub red_force_played: Option<i64>,
	pub red_levitate_count: Option<i64>,
	pub red_levitate_played: Option<i64>,
	pub red_powerup_time_remaining: Option<String>,
	pub red_scale_owned: Option<i64>,
	pub red_score: Option<i64>,
	pub red_switch_owned: Option<i64>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MediaAvatarDetails {
	pub base64_image: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MediaCdPhotoThreadDetails {
	pub image_partial: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MediaCdThreadDetails {
	pub thread_title: String,
	pub image_url: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MediaGrabCadDetails {
	pub model_created: String,
	pub model_description: Option<String>,
	pub model_image: String,
	pub model_name: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum MediaNoDetailsType {
	Youtube,
	Imgur,
	FacebookProfile,
	YoutubeChannel,
	TwitterProfile,
	GithubProfile,
	InstagramProfile,
	PeriscopeProfile,
	GitlabProfile,
	InstagramImage,
	ExternalLink,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MediaOnshapeDetails {
	pub model_created: String,
	pub model_description: Option<String>,
	pub model_image: String,
	pub model_name: String,
}

// The OpenAPI discriminator is authoritative here. A derived untagged union
// cannot distinguish missing details or the identical GrabCAD/Onshape shapes.
#[derive(Debug, Clone)]
pub enum Media {
	Avatar {
		foreign_key: String,
		preferred: Option<bool>,
		team_keys: Vec<String>,
		direct_url: Option<String>,
		view_url: Option<String>,
		details: Option<MediaAvatarDetails>,
	},
	CdPhotoThread {
		foreign_key: String,
		preferred: Option<bool>,
		team_keys: Vec<String>,
		direct_url: Option<String>,
		view_url: Option<String>,
		details: Option<MediaCdPhotoThreadDetails>,
	},
	CdThread {
		foreign_key: String,
		preferred: Option<bool>,
		team_keys: Vec<String>,
		direct_url: Option<String>,
		view_url: Option<String>,
		details: Option<MediaCdThreadDetails>,
	},
	GrabCad {
		foreign_key: String,
		preferred: Option<bool>,
		team_keys: Vec<String>,
		direct_url: Option<String>,
		view_url: Option<String>,
		details: Option<MediaGrabCadDetails>,
	},
	NoDetails {
		type_: MediaNoDetailsType,
		foreign_key: String,
		preferred: Option<bool>,
		team_keys: Vec<String>,
		direct_url: Option<String>,
		view_url: Option<String>,
		details: Option<UnknownJsonObject>,
	},
	Onshape {
		foreign_key: String,
		preferred: Option<bool>,
		team_keys: Vec<String>,
		direct_url: Option<String>,
		view_url: Option<String>,
		details: Option<MediaOnshapeDetails>,
	},
}

#[derive(Deserialize)]
struct MediaFields<D> {
	foreign_key: String,
	preferred: Option<bool>,
	team_keys: Vec<String>,
	direct_url: Option<String>,
	view_url: Option<String>,
	details: Option<D>,
}

#[derive(Serialize)]
struct MediaFieldsRef<'a, T, D> {
	#[serde(rename = "type")]
	type_: T,
	foreign_key: &'a str,
	preferred: &'a Option<bool>,
	team_keys: &'a [String],
	direct_url: &'a Option<String>,
	view_url: &'a Option<String>,
	details: &'a Option<D>,
}

impl<'de> Deserialize<'de> for Media {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: serde::Deserializer<'de>,
	{
		use serde::de::Error;

		let value = serde_json::Value::deserialize(deserializer)?;
		let type_ = value
			.get("type")
			.ok_or_else(|| {
				D::Error::custom("missing media discriminator `type`")
			})?
			.as_str()
			.ok_or_else(|| {
				D::Error::custom("media discriminator `type` must be a string")
			})?
			.to_owned();

		macro_rules! deserialize_variant {
			($details:ty, $variant:ident) => {{
				let fields: MediaFields<$details> =
					serde_json::from_value(value).map_err(D::Error::custom)?;
				Self::$variant {
					foreign_key: fields.foreign_key,
					preferred: fields.preferred,
					team_keys: fields.team_keys,
					direct_url: fields.direct_url,
					view_url: fields.view_url,
					details: fields.details,
				}
			}};
		}

		Ok(match type_.as_str() {
			"avatar" => deserialize_variant!(MediaAvatarDetails, Avatar),
			"cdphotothread" => {
				deserialize_variant!(MediaCdPhotoThreadDetails, CdPhotoThread)
			}
			"cd-thread" => deserialize_variant!(MediaCdThreadDetails, CdThread),
			"grabcad" => deserialize_variant!(MediaGrabCadDetails, GrabCad),
			"onshape" => deserialize_variant!(MediaOnshapeDetails, Onshape),
			"youtube" | "imgur" | "facebook-profile" | "youtube-channel"
			| "twitter-profile" | "github-profile" | "instagram-profile"
			| "periscope-profile" | "gitlab-profile" | "instagram-image"
			| "external-link" => {
				let type_ =
					serde_json::from_value(serde_json::Value::String(type_))
						.map_err(D::Error::custom)?;
				let fields: MediaFields<UnknownJsonObject> =
					serde_json::from_value(value).map_err(D::Error::custom)?;
				Self::NoDetails {
					type_,
					foreign_key: fields.foreign_key,
					preferred: fields.preferred,
					team_keys: fields.team_keys,
					direct_url: fields.direct_url,
					view_url: fields.view_url,
					details: fields.details,
				}
			}
			unknown => {
				return Err(D::Error::custom(format!(
					"unknown media discriminator `{unknown}`"
				)));
			}
		})
	}
}

impl Serialize for Media {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: serde::Serializer,
	{
		macro_rules! serialize_variant {
			($type:expr, $fields:expr) => {{
				let (
					foreign_key,
					preferred,
					team_keys,
					direct_url,
					view_url,
					details,
				) = $fields;
				MediaFieldsRef {
					type_: $type,
					foreign_key,
					preferred,
					team_keys,
					direct_url,
					view_url,
					details,
				}
				.serialize(serializer)
			}};
		}

		match self {
			Self::Avatar {
				foreign_key,
				preferred,
				team_keys,
				direct_url,
				view_url,
				details,
			} => serialize_variant!(
				"avatar",
				(
					foreign_key,
					preferred,
					team_keys,
					direct_url,
					view_url,
					details
				)
			),
			Self::CdPhotoThread {
				foreign_key,
				preferred,
				team_keys,
				direct_url,
				view_url,
				details,
			} => serialize_variant!(
				"cdphotothread",
				(
					foreign_key,
					preferred,
					team_keys,
					direct_url,
					view_url,
					details
				)
			),
			Self::CdThread {
				foreign_key,
				preferred,
				team_keys,
				direct_url,
				view_url,
				details,
			} => serialize_variant!(
				"cd-thread",
				(
					foreign_key,
					preferred,
					team_keys,
					direct_url,
					view_url,
					details
				)
			),
			Self::GrabCad {
				foreign_key,
				preferred,
				team_keys,
				direct_url,
				view_url,
				details,
			} => serialize_variant!(
				"grabcad",
				(
					foreign_key,
					preferred,
					team_keys,
					direct_url,
					view_url,
					details
				)
			),
			Self::NoDetails {
				type_,
				foreign_key,
				preferred,
				team_keys,
				direct_url,
				view_url,
				details,
			} => serialize_variant!(
				type_,
				(
					foreign_key,
					preferred,
					team_keys,
					direct_url,
					view_url,
					details
				)
			),
			Self::Onshape {
				foreign_key,
				preferred,
				team_keys,
				direct_url,
				view_url,
				details,
			} => serialize_variant!(
				"onshape",
				(
					foreign_key,
					preferred,
					team_keys,
					direct_url,
					view_url,
					details
				)
			),
		}
	}
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct NexusMatchTiming {
	pub estimated_queue_time_ms: Option<i64>,
	pub estimated_start_time_ms: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub enum NexusMatchInfoStatus {
	#[serde(rename = "Queuing soon")]
	QueuingSoon,
	#[serde(rename = "Now queuing")]
	NowQueuing,
	#[serde(rename = "On deck")]
	OnDeck,
	#[serde(rename = "On field")]
	OnField,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct NexusMatchInfo {
	pub label: String,
	pub status: NexusMatchInfoStatus,
	pub played: bool,
	pub times: NexusMatchTiming,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct NexusNowQueueing {
	pub match_key: String,
	pub match_name: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct NexusEventInfo {
	pub data_as_of_ms: i64,
	pub now_queueing: Option<NexusNowQueueing>,
	pub matches: std::collections::HashMap<String, NexusMatchInfo>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct NotablesInsightDataEntriesItem {
	pub context: Vec<String>,
	pub team_key: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct NotablesInsightData {
	pub entries: Vec<NotablesInsightDataEntriesItem>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct NotablesInsight {
	pub data: NotablesInsightData,
	pub name: String,
	pub year: i64,
}

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

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub enum RegionalAdvancementCMPStatus {
	NotInvited,
	PreQualified,
	EventQualified,
	PoolQualified,
	Declined,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RegionalAdvancement {
	pub cmp: bool,
	pub cmp_status: RegionalAdvancementCMPStatus,
	pub qualifying_event: Option<String>,
	pub qualifying_award_name: Option<String>,
	pub qualifying_pool_week: Option<i64>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RegionalRankingEventPointsItem {
	pub total: i64,
	pub alliance_points: i64,
	pub elim_points: i64,
	pub award_points: i64,
	pub event_key: String,
	pub qual_points: i64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RegionalRanking {
	pub team_key: String,
	pub rank: i64,
	pub rookie_bonus: Option<i64>,
	pub single_event_bonus: Option<i64>,
	pub point_total: i64,
	pub event_points: Option<Vec<RegionalRankingEventPointsItem>>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SearchIndexTeamsItem {
	pub key: String,
	pub nickname: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SearchIndexEventsItem {
	pub key: String,
	pub name: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SearchIndex {
	pub teams: Vec<SearchIndexTeamsItem>,
	pub events: Vec<SearchIndexEventsItem>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Team {
	pub key: String,
	pub team_number: i64,
	pub nickname: String,
	pub name: String,
	pub school_name: Option<String>,
	pub city: Option<String>,
	pub state_prov: Option<String>,
	pub country: Option<String>,
	pub address: Option<String>,
	pub postal_code: Option<String>,
	pub gmaps_place_id: Option<String>,
	pub gmaps_url: Option<String>,
	pub lat: Option<f64>,
	pub lng: Option<f64>,
	pub location_name: Option<String>,
	pub website: Option<String>,
	pub rookie_year: Option<i64>,
	pub motto: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TeamEventStatusRankRanking {
	pub matches_played: Option<i64>,
	pub qual_average: Option<f64>,
	pub sort_orders: Option<Vec<f64>>,
	pub record: Option<WLTRecord>,
	pub rank: Option<i64>,
	pub dq: Option<i64>,
	pub team_key: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TeamEventStatusRankSortOrderInfoItem {
	pub precision: Option<i64>,
	pub name: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TeamEventStatusRank {
	pub num_teams: Option<i64>,
	pub ranking: Option<TeamEventStatusRankRanking>,
	pub sort_order_info: Option<Vec<TeamEventStatusRankSortOrderInfoItem>>,
	pub status: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TeamEventStatusAllianceBackup {
	pub out: Option<String>,
	#[serde(rename = "in")]
	pub in_: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TeamEventStatusAlliance {
	pub name: Option<String>,
	pub number: i64,
	pub backup: Option<TeamEventStatusAllianceBackup>,
	pub pick: i64,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum TeamEventStatusPlayoffStatus {
	Won,
	Eliminated,
	Playing,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TeamEventStatusPlayoff {
	pub level: Option<CompLevel>,
	pub current_level_record: Option<WLTRecord>,
	pub record: Option<WLTRecord>,
	pub status: Option<TeamEventStatusPlayoffStatus>,
	pub playoff_average: Option<f64>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TeamEventStatus {
	pub qual: Option<TeamEventStatusRank>,
	pub alliance: Option<TeamEventStatusAlliance>,
	pub playoff: Option<TeamEventStatusPlayoff>,
	pub alliance_status_str: Option<String>,
	pub playoff_status_str: Option<String>,
	pub overall_status_str: Option<String>,
	pub next_match_key: Option<String>,
	pub last_match_key: Option<String>,
	pub pit_location: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TeamRobot {
	pub year: i64,
	pub robot_name: String,
	pub key: String,
	pub team_key: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TeamSimple {
	pub key: String,
	pub team_number: i64,
	pub nickname: String,
	pub name: String,
	pub city: Option<String>,
	pub state_prov: Option<String>,
	pub country: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ZebraTeam {
	pub team_key: String,
	pub xs: Vec<f64>,
	pub ys: Vec<f64>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ZebraAlliances {
	pub red: Option<Vec<ZebraTeam>>,
	pub blue: Option<Vec<ZebraTeam>>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Zebra {
	pub key: String,
	pub times: Vec<f64>,
	pub alliances: ZebraAlliances,
}

pub type DistrictAdvancementByTeam =
	std::collections::HashMap<String, DistrictAdvancement>;
pub type RegionalAdvancementByTeam =
	std::collections::HashMap<String, RegionalAdvancement>;
