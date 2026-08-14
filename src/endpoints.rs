#[rustfmt::skip]
endpoints!(

	/// Endpoints responsible for information about individual competition
	/// districts.
	District {

		/// Gets a list of DCMP events and awards for the given district
		/// abbreviation.
		DCMPHistory {
			snake_case: dcmp_history,
			path: "/district/{district_abbreviation}/dcmp_history",
			input: {
				/// The abbreviated district name (e.g. `ne` or `fim`).
				district_abbreviation: String,
			},
			output: Vec<crate::models::DistrictDCMPHistoryEntry>,
		}

		/// Gets a list of District objects with the given district
		/// abbreviation. This accounts for district abbreviation
		/// changes, such as MAR to FMA.
		History {
			path: "/district/{district_abbreviation}/history",
			input: {
				/// The abbreviated district name (e.g. `ne` or `fim`).
				district_abbreviation: String,
			},
			output: Vec<crate::models::District>,
		}

		/// Gets insights for a given district.
		Insights {
			path: "/district/{district_abbreviation}/insights",
			input: {
				/// The abbreviated district name (e.g. `ne` or `fim`).
				district_abbreviation: String,
			},
			output: crate::models::DistrictInsight,
		}

		/// Gets a list of advancement information per team in a district.
		Advancement {
			path: "/district/{district_key}/advancement",
			input: {
				/// The TBA district key (e.g. `2016fim`).
				district_key: String,
			},
			output: Option<crate::models::DistrictAdvancementByTeam>,
		}

		/// Gets a list of awards in the given district.
		Awards {
			path: "/district/{district_key}/awards",
			input: {
				/// The TBA district key (e.g. `2016fim`).
				district_key: String,
			},
			output: Vec<crate::models::Award>,
		}

		/// Gets a list of events in the given district.
		Events {
			path: "/district/{district_key}/events",
			input: {
				/// The TBA district key (e.g. `2016fim`).
				district_key: String,
			},
			output: Vec<crate::models::Event>,
		}

		/// Gets a list of event keys in the given district.
		EventsKeys {
			path: "/district/{district_key}/events/keys",
			input: {
				/// The TBA district key (e.g. `2016fim`).
				district_key: String,
			},
			output: Vec<String>,
		}

		/// Gets a list of simple events in the given district.
		EventsSimple {
			path: "/district/{district_key}/events/simple",
			input: {
				/// The TBA district key (e.g. `2016fim`).
				district_key: String,
			},
			output: Vec<crate::models::EventSimple>,
		}

		/// Gets rankings for teams in the given district.
		Rankings {
			path: "/district/{district_key}/rankings",
			input: {
				/// The TBA district key (e.g. `2016fim`).
				district_key: String,
			},
			output: Option<Vec<crate::models::DistrictRanking>>,
		}

		/// Gets a list of teams in the given district.
		Teams {
			path: "/district/{district_key}/teams",
			input: {
				/// The TBA district key (e.g. `2016fim`).
				district_key: String,
			},
			output: Vec<crate::models::Team>,
		}

		/// Gets a list of team keys in the given district.
		TeamsKeys {
			path: "/district/{district_key}/teams/keys",
			input: {
				/// The TBA district key (e.g. `2016fim`).
				district_key: String,
			},
			output: Vec<String>,
		}

		/// Gets a list of simple teams in the given district.
		TeamsSimple {
			path: "/district/{district_key}/teams/simple",
			input: {
				/// The TBA district key (e.g. `2016fim`).
				district_key: String,
			},
			output: Vec<crate::models::TeamSimple>,
		}

		/// Gets a list of districts for the given year.
		DistrictsByYear {
			path: "/districts/{year}",
			input: {
				/// Competition year.
				year: i64,
			},
			output: Vec<crate::models::District>,
		}

	}

	/// Endpoints responsible for information about individual events.
	Event {

		/// Gets an event.
		Event {
			path: "/event/{event_key}",
			input: {
				/// The TBA event key (e.g. `2016nytr`).
				event_key: String,
			},
			output: crate::models::Event,
		}

		/// Gets advancement points for an event.
		AdvancementPoints {
			path: "/event/{event_key}/advancement_points",
			input: {
				/// The TBA event key (e.g. `2016nytr`).
				event_key: String,
			},
			output: Option<crate::models::EventDistrictPoints>,
		}

		/// Gets alliances for an event.
		Alliances {
			path: "/event/{event_key}/alliances",
			input: {
				/// The TBA event key (e.g. `2016nytr`).
				event_key: String,
			},
			output: Option<Vec<crate::models::EliminationAlliance>>,
		}

		/// Gets awards for an event.
		Awards {
			path: "/event/{event_key}/awards",
			input: {
				/// The TBA event key (e.g. `2016nytr`).
				event_key: String,
			},
			output: Vec<crate::models::Award>,
		}

		/// Gets component OPRs for an event.
		COPRs {
			snake_case: coprs,
			path: "/event/{event_key}/coprs",
			input: {
				/// The TBA event key (e.g. `2016nytr`).
				event_key: String,
			},
			output: Option<crate::models::EventCOPRs>,
		}

		/// Gets district points for an event.
		DistrictPoints {
			path: "/event/{event_key}/district_points",
			input: {
				/// The TBA event key (e.g. `2016nytr`).
				event_key: String,
			},
			output: Option<crate::models::EventDistrictPoints>,
		}

		/// Gets insights for an event.
		Insights {
			path: "/event/{event_key}/insights",
			input: {
				/// The TBA event key (e.g. `2016nytr`).
				event_key: String,
			},
			output: Option<crate::models::EventInsights>,
		}

		/// Gets matches for an event.
		Matches {
			path: "/event/{event_key}/matches",
			input: {
				/// The TBA event key (e.g. `2016nytr`).
				event_key: String,
			},
			output: Vec<crate::models::Match>,
		}

		/// Gets match keys for an event.
		MatchesKeys {
			path: "/event/{event_key}/matches/keys",
			input: {
				/// The TBA event key (e.g. `2016nytr`).
				event_key: String,
			},
			output: Vec<String>,
		}

		/// Gets simple matches for an event.
		MatchesSimple {
			path: "/event/{event_key}/matches/simple",
			input: {
				/// The TBA event key (e.g. `2016nytr`).
				event_key: String,
			},
			output: Vec<crate::models::MatchSimple>,
		}

		/// Gets match timeseries data for an event.
		MatchTimeseries {
			path: "/event/{event_key}/matches/timeseries",
			input: {
				/// The TBA event key (e.g. `2016nytr`).
				event_key: String,
			},
			output: Vec<String>,
		}

		/// Gets Nexus information for an event.
		NexusInfo {
			path: "/event/{event_key}/nexus_info",
			input: {
				/// The TBA event key (e.g. `2016nytr`).
				event_key: String,
			},
			output: Option<crate::models::NexusEventInfo>,
		}

		/// Gets OPRs for an event.
		OPRs {
			snake_case: oprs,
			path: "/event/{event_key}/oprs",
			input: {
				/// The TBA event key (e.g. `2016nytr`).
				event_key: String,
			},
			output: Option<crate::models::EventOPRs>,
		}

		/// Gets predictions for an event.
		Predictions {
			path: "/event/{event_key}/predictions",
			input: {
				/// The TBA event key (e.g. `2016nytr`).
				event_key: String,
			},
			output: Option<crate::models::EventPredictions>,
		}

		/// Gets rankings for an event.
		Rankings {
			path: "/event/{event_key}/rankings",
			input: {
				/// The TBA event key (e.g. `2016nytr`).
				event_key: String,
			},
			output: Option<crate::models::EventRanking>,
		}

		/// Gets regional champs pool points for an event.
		RegionalChampsPoolPoints {
			path: "/event/{event_key}/regional_champs_pool_points",
			input: {
				/// The TBA event key (e.g. `2016nytr`).
				event_key: String,
			},
			output: Option<crate::models::EventDistrictPoints>,
		}

		/// Gets a simple event.
		Simple {
			path: "/event/{event_key}/simple",
			input: {
				/// The TBA event key (e.g. `2016nytr`).
				event_key: String,
			},
			output: crate::models::EventSimple,
		}

		/// Gets team media for an event.
		TeamMedia {
			path: "/event/{event_key}/team_media",
			input: {
				/// The TBA event key (e.g. `2016nytr`).
				event_key: String,
			},
			output: Vec<crate::models::Media>,
		}

		/// Gets teams for an event.
		Teams {
			path: "/event/{event_key}/teams",
			input: {
				/// The TBA event key (e.g. `2016nytr`).
				event_key: String,
			},
			output: Vec<crate::models::Team>,
		}

		/// Gets team keys for an event.
		TeamsKeys {
			path: "/event/{event_key}/teams/keys",
			input: {
				/// The TBA event key (e.g. `2016nytr`).
				event_key: String,
			},
			output: Vec<String>,
		}

		/// Gets simple teams for an event.
		TeamsSimple {
			path: "/event/{event_key}/teams/simple",
			input: {
				/// The TBA event key (e.g. `2016nytr`).
				event_key: String,
			},
			output: Vec<crate::models::TeamSimple>,
		}

		/// Gets team statuses for an event.
		TeamsStatuses {
			path: "/event/{event_key}/teams/statuses",
			input: {
				/// The TBA event key (e.g. `2016nytr`).
				event_key: String,
			},
			output: std::collections::HashMap<String, Option<crate::models::TeamEventStatus>>,
		}

		/// Gets events for a year.
		EventsByYear {
			path: "/events/{year}",
			input: {
				/// Competition year.
				year: i64,
			},
			output: Vec<crate::models::Event>,
		}

		/// Gets event keys for a year.
		EventsByYearKeys {
			path: "/events/{year}/keys",
			input: {
				/// Competition year.
				year: i64,
			},
			output: Vec<String>,
		}

		/// Gets simple events for a year.
		EventsByYearSimple {
			path: "/events/{year}/simple",
			input: {
				/// Competition year.
				year: i64,
			},
			output: Vec<crate::models::EventSimple>,
		}

	}

	/// Endpoints responsible for insights.
	Insight {

		/// Gets leaderboard insights for a year.
		LeaderboardsYear {
			path: "/insights/leaderboards/{year}",
			input: {
				/// Competition year.
				year: i64,
			},
			output: Vec<crate::models::LeaderboardInsight>,
		}

		/// Gets notable insights for a year.
		NotablesYear {
			path: "/insights/notables/{year}",
			input: {
				/// Competition year.
				year: i64,
			},
			output: Vec<crate::models::NotablesInsight>,
		}

		/// Gets V2 insights for a year.
		V2Year {
			path: "/insights/{year}",
			input: {
				/// Competition year.
				year: i64,
			},
			output: Vec<crate::models::InsightV2>,
		}

		/// Gets V2 insights for a year and category.
		V2YearCategory {
			path: "/insights/{year}/{category}",
			input: {
				/// Competition year.
				year: i64,
				/// Insight category.
				category: String,
			},
			output: Vec<crate::models::InsightV2>,
		}

		/// Gets V2 insights for a year and district.
		V2YearDistrict {
			path: "/insights/{year}/district/{district_abbreviation}",
			input: {
				/// Competition year.
				year: i64,
				/// The abbreviated district name (e.g. `ne` or `fim`).
				district_abbreviation: String,
			},
			output: Vec<crate::models::InsightV2>,
		}

		/// Gets V2 insights for a year, category, and district.
		V2YearCategoryDistrict {
			path: "/insights/{year}/{category}/district/{district_abbreviation}",
			input: {
				/// Competition year.
				year: i64,
				/// Insight category.
				category: String,
				/// The abbreviated district name (e.g. `ne` or `fim`).
				district_abbreviation: String,
			},
			output: Vec<crate::models::InsightV2>,
		}

	}

	/// Endpoints responsible for information about individual matches.
	MatchAPI {

		/// Gets a match.
		Match {
			snake_case: match_,
			path: "/match/{match_key}",
			input: {
				/// The TBA match key.
				match_key: String,
			},
			output: crate::models::Match,
		}

		/// Gets a simple match.
		Simple {
			path: "/match/{match_key}/simple",
			input: {
				/// The TBA match key.
				match_key: String,
			},
			output: crate::models::MatchSimple,
		}

		/// Gets match timeseries data.
		Timeseries {
			path: "/match/{match_key}/timeseries",
			input: {
				/// The TBA match key.
				match_key: String,
			},
			output: Vec<crate::models::UnknownJsonObject>,
		}

		/// Gets Zebra MotionWorks data for a match.
		Zebra {
			path: "/match/{match_key}/zebra_motionworks",
			input: {
				/// The TBA match key.
				match_key: String,
			},
			output: crate::models::Zebra,
		}

	}

	/// Endpoints responsible for regional advancement information.
	RegionalAdvancement {

		/// Gets regional advancement information for a year.
		Advancement {
			path: "/regional_advancement/{year}",
			input: {
				/// Competition year.
				year: i64,
			},
			output: Option<crate::models::RegionalAdvancementByTeam>,
		}

		/// Gets regional rankings for a year.
		Rankings {
			path: "/regional_advancement/{year}/rankings",
			input: {
				/// Competition year.
				year: i64,
			},
			output: Option<Vec<crate::models::RegionalRanking>>,
		}

	}

	/// Endpoints responsible for search metadata.
	Search {

		/// Gets the search index.
		Index {
			path: "/search_index",
			input: {},
			output: crate::models::SearchIndex,
		}

	}

	/// Endpoints responsible for TBA API metadata.
	TBA {

		/// Gets API status information.
		Status {
			path: "/status",
			input: {},
			output: crate::models::APIStatus,
		}

	}

	/// Endpoints responsible for information about individual teams.
	Team {

		/// Gets a team.
		Team {
			path: "/team/{team_key}",
			input: {
				/// The TBA team key (e.g. `frc254`).
				team_key: String,
			},
			output: crate::models::Team,
		}

		/// Gets awards for a team.
		Awards {
			path: "/team/{team_key}/awards",
			input: {
				/// The TBA team key (e.g. `frc254`).
				team_key: String,
			},
			output: Vec<crate::models::Award>,
		}

		/// Gets awards for a team in a year.
		AwardsByYear {
			path: "/team/{team_key}/awards/{year}",
			input: {
				/// The TBA team key (e.g. `frc254`).
				team_key: String,
				/// Competition year.
				year: i64,
			},
			output: Vec<crate::models::Award>,
		}

		/// Gets districts for a team.
		Districts {
			path: "/team/{team_key}/districts",
			input: {
				/// The TBA team key (e.g. `frc254`).
				team_key: String,
			},
			output: Vec<crate::models::District>,
		}

		/// Gets awards for a team at an event.
		EventAwards {
			path: "/team/{team_key}/event/{event_key}/awards",
			input: {
				/// The TBA team key (e.g. `frc254`).
				team_key: String,
				/// The TBA event key (e.g. `2016nytr`).
				event_key: String,
			},
			output: Vec<crate::models::Award>,
		}

		/// Gets matches for a team at an event.
		EventMatches {
			path: "/team/{team_key}/event/{event_key}/matches",
			input: {
				/// The TBA team key (e.g. `frc254`).
				team_key: String,
				/// The TBA event key (e.g. `2016nytr`).
				event_key: String,
			},
			output: Vec<crate::models::Match>,
		}

		/// Gets match keys for a team at an event.
		EventMatchesKeys {
			path: "/team/{team_key}/event/{event_key}/matches/keys",
			input: {
				/// The TBA team key (e.g. `frc254`).
				team_key: String,
				/// The TBA event key (e.g. `2016nytr`).
				event_key: String,
			},
			output: Vec<String>,
		}

		/// Gets simple matches for a team at an event.
		EventMatchesSimple {
			path: "/team/{team_key}/event/{event_key}/matches/simple",
			input: {
				/// The TBA team key (e.g. `frc254`).
				team_key: String,
				/// The TBA event key (e.g. `2016nytr`).
				event_key: String,
			},
			output: Vec<crate::models::Match>,
		}

		/// Gets team status at an event.
		EventStatus {
			path: "/team/{team_key}/event/{event_key}/status",
			input: {
				/// The TBA team key (e.g. `frc254`).
				team_key: String,
				/// The TBA event key (e.g. `2016nytr`).
				event_key: String,
			},
			output: Option<crate::models::TeamEventStatus>,
		}

		/// Gets events for a team.
		Events {
			path: "/team/{team_key}/events",
			input: {
				/// The TBA team key (e.g. `frc254`).
				team_key: String,
			},
			output: Vec<crate::models::Event>,
		}

		/// Gets event keys for a team.
		EventsKeys {
			path: "/team/{team_key}/events/keys",
			input: {
				/// The TBA team key (e.g. `frc254`).
				team_key: String,
			},
			output: Vec<String>,
		}

		/// Gets simple events for a team.
		EventsSimple {
			path: "/team/{team_key}/events/simple",
			input: {
				/// The TBA team key (e.g. `frc254`).
				team_key: String,
			},
			output: Vec<crate::models::EventSimple>,
		}

		/// Gets events for a team in a year.
		EventsByYear {
			path: "/team/{team_key}/events/{year}",
			input: {
				/// The TBA team key (e.g. `frc254`).
				team_key: String,
				/// Competition year.
				year: i64,
			},
			output: Vec<crate::models::Event>,
		}

		/// Gets event keys for a team in a year.
		EventsByYearKeys {
			path: "/team/{team_key}/events/{year}/keys",
			input: {
				/// The TBA team key (e.g. `frc254`).
				team_key: String,
				/// Competition year.
				year: i64,
			},
			output: Vec<String>,
		}

		/// Gets simple events for a team in a year.
		EventsByYearSimple {
			path: "/team/{team_key}/events/{year}/simple",
			input: {
				/// The TBA team key (e.g. `frc254`).
				team_key: String,
				/// Competition year.
				year: i64,
			},
			output: Vec<crate::models::EventSimple>,
		}

		/// Gets event statuses for a team in a year.
		EventsStatusesByYear {
			path: "/team/{team_key}/events/{year}/statuses",
			input: {
				/// The TBA team key (e.g. `frc254`).
				team_key: String,
				/// Competition year.
				year: i64,
			},
			output: std::collections::HashMap<String, Option<crate::models::TeamEventStatus>>,
		}

		/// Gets history for a team.
		History {
			path: "/team/{team_key}/history",
			input: {
				/// The TBA team key (e.g. `frc254`).
				team_key: String,
			},
			output: crate::models::History,
		}

		/// Gets matches for a team in a year.
		MatchesByYear {
			path: "/team/{team_key}/matches/{year}",
			input: {
				/// The TBA team key (e.g. `frc254`).
				team_key: String,
				/// Competition year.
				year: i64,
			},
			output: Vec<crate::models::Match>,
		}

		/// Gets match keys for a team in a year.
		MatchesByYearKeys {
			path: "/team/{team_key}/matches/{year}/keys",
			input: {
				/// The TBA team key (e.g. `frc254`).
				team_key: String,
				/// Competition year.
				year: i64,
			},
			output: Vec<String>,
		}

		/// Gets simple matches for a team in a year.
		MatchesByYearSimple {
			path: "/team/{team_key}/matches/{year}/simple",
			input: {
				/// The TBA team key (e.g. `frc254`).
				team_key: String,
				/// Competition year.
				year: i64,
			},
			output: Vec<crate::models::MatchSimple>,
		}

		/// Gets media for a team with the given tag.
		MediaByTag {
			path: "/team/{team_key}/media/tag/{media_tag}",
			input: {
				/// The TBA team key (e.g. `frc254`).
				team_key: String,
				/// Media tag.
				media_tag: String,
			},
			output: Vec<crate::models::Media>,
		}

		/// Gets media for a team with the given tag and year.
		MediaByTagYear {
			path: "/team/{team_key}/media/tag/{media_tag}/{year}",
			input: {
				/// The TBA team key (e.g. `frc254`).
				team_key: String,
				/// Media tag.
				media_tag: String,
				/// Competition year.
				year: i64,
			},
			output: Vec<crate::models::Media>,
		}

		/// Gets media for a team in a year.
		MediaByYear {
			path: "/team/{team_key}/media/{year}",
			input: {
				/// The TBA team key (e.g. `frc254`).
				team_key: String,
				/// Competition year.
				year: i64,
			},
			output: Vec<crate::models::Media>,
		}

		/// Gets robots for a team.
		Robots {
			path: "/team/{team_key}/robots",
			input: {
				/// The TBA team key (e.g. `frc254`).
				team_key: String,
			},
			output: Vec<crate::models::TeamRobot>,
		}

		/// Gets a simple team.
		Simple {
			path: "/team/{team_key}/simple",
			input: {
				/// The TBA team key (e.g. `frc254`).
				team_key: String,
			},
			output: crate::models::TeamSimple,
		}

		/// Gets social media for a team.
		SocialMedia {
			path: "/team/{team_key}/social_media",
			input: {
				/// The TBA team key (e.g. `frc254`).
				team_key: String,
			},
			output: Vec<crate::models::Media>,
		}

		/// Gets years participated for a team.
		YearsParticipated {
			path: "/team/{team_key}/years_participated",
			input: {
				/// The TBA team key (e.g. `frc254`).
				team_key: String,
			},
			output: Vec<i64>,
		}

		/// Gets teams for a page.
		Teams {
			path: "/teams/{page_num}",
			input: {
				/// Page number.
				page_num: i64,
			},
			output: Vec<crate::models::Team>,
		}

		/// Gets team keys for a page.
		TeamsKeys {
			path: "/teams/{page_num}/keys",
			input: {
				/// Page number.
				page_num: i64,
			},
			output: Vec<String>,
		}

		/// Gets simple teams for a page.
		TeamsSimple {
			path: "/teams/{page_num}/simple",
			input: {
				/// Page number.
				page_num: i64,
			},
			output: Vec<crate::models::TeamSimple>,
		}

		/// Gets teams for a year and page.
		TeamsByYear {
			path: "/teams/{year}/{page_num}",
			input: {
				/// Competition year.
				year: i64,
				/// Page number.
				page_num: i64,
			},
			output: Vec<crate::models::Team>,
		}

		/// Gets team keys for a year and page.
		TeamsByYearKeys {
			path: "/teams/{year}/{page_num}/keys",
			input: {
				/// Competition year.
				year: i64,
				/// Page number.
				page_num: i64,
			},
			output: Vec<String>,
		}

		/// Gets simple teams for a year and page.
		TeamsByYearSimple {
			path: "/teams/{year}/{page_num}/simple",
			input: {
				/// Competition year.
				year: i64,
				/// Page number.
				page_num: i64,
			},
			output: Vec<crate::models::TeamSimple>,
		}

	}

);
