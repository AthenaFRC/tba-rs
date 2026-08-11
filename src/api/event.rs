use crate::{
	api_client::APIClient,
	api_result::APIResult,
	models::*,
};

pub async fn event(
	api: &APIClient,
	event_key: impl AsRef<str>,
	e_tag: Option<String>,
) -> APIResult<Event> {
	api.get(format!("/event/{}", event_key.as_ref()).as_str(), e_tag)
		.await
}

pub async fn event_advancement_points(
	api: &APIClient,
	event_key: impl AsRef<str>,
	e_tag: Option<String>,
) -> APIResult<Option<EventDistrictPoints>> {
	api.get(
		format!("/event/{}/advancement_points", event_key.as_ref()).as_str(),
		e_tag,
	)
	.await
}

pub async fn event_alliances(
	api: &APIClient,
	event_key: impl AsRef<str>,
	e_tag: Option<String>,
) -> APIResult<Option<Vec<EliminationAlliance>>> {
	api.get(
		format!("/event/{}/alliances", event_key.as_ref()).as_str(),
		e_tag,
	)
	.await
}

pub async fn event_awards(
	api: &APIClient,
	event_key: impl AsRef<str>,
	e_tag: Option<String>,
) -> APIResult<Vec<Award>> {
	api.get(
		format!("/event/{}/awards", event_key.as_ref()).as_str(),
		e_tag,
	)
	.await
}

pub async fn event_coprs(
	api: &APIClient,
	event_key: impl AsRef<str>,
	e_tag: Option<String>,
) -> APIResult<Option<EventCOPRs>> {
	api.get(
		format!("/event/{}/coprs", event_key.as_ref()).as_str(),
		e_tag,
	)
	.await
}

pub async fn event_district_points(
	api: &APIClient,
	event_key: impl AsRef<str>,
	e_tag: Option<String>,
) -> APIResult<Option<EventDistrictPoints>> {
	api.get(
		format!("/event/{}/district_points", event_key.as_ref()).as_str(),
		e_tag,
	)
	.await
}

pub async fn event_insights(
	api: &APIClient,
	event_key: impl AsRef<str>,
	e_tag: Option<String>,
) -> APIResult<Option<EventInsights>> {
	api.get(
		format!("/event/{}/insights", event_key.as_ref()).as_str(),
		e_tag,
	)
	.await
}

pub async fn event_matches(
	api: &APIClient,
	event_key: impl AsRef<str>,
	e_tag: Option<String>,
) -> APIResult<Vec<Match>> {
	api.get(
		format!("/event/{}/matches", event_key.as_ref()).as_str(),
		e_tag,
	)
	.await
}

pub async fn event_matches_keys(
	api: &APIClient,
	event_key: impl AsRef<str>,
	e_tag: Option<String>,
) -> APIResult<Vec<String>> {
	api.get(
		format!("/event/{}/matches/keys", event_key.as_ref()).as_str(),
		e_tag,
	)
	.await
}

pub async fn event_matches_simple(
	api: &APIClient,
	event_key: impl AsRef<str>,
	e_tag: Option<String>,
) -> APIResult<Vec<MatchSimple>> {
	api.get(
		format!("/event/{}/matches/simple", event_key.as_ref()).as_str(),
		e_tag,
	)
	.await
}

pub async fn event_match_timeseries(
	api: &APIClient,
	event_key: impl AsRef<str>,
	e_tag: Option<String>,
) -> APIResult<Vec<String>> {
	api.get(
		format!("/event/{}/matches/timeseries", event_key.as_ref()).as_str(),
		e_tag,
	)
	.await
}

pub async fn event_nexus_info(
	api: &APIClient,
	event_key: impl AsRef<str>,
	e_tag: Option<String>,
) -> APIResult<Option<NexusEventInfo>> {
	api.get(
		format!("/event/{}/nexus_info", event_key.as_ref()).as_str(),
		e_tag,
	)
	.await
}

pub async fn event_oprs(
	api: &APIClient,
	event_key: impl AsRef<str>,
	e_tag: Option<String>,
) -> APIResult<Option<EventOPRs>> {
	api.get(
		format!("/event/{}/oprs", event_key.as_ref()).as_str(),
		e_tag,
	)
	.await
}

pub async fn event_predictions(
	api: &APIClient,
	event_key: impl AsRef<str>,
	e_tag: Option<String>,
) -> APIResult<Option<EventPredictions>> {
	api.get(
		format!("/event/{}/predictions", event_key.as_ref()).as_str(),
		e_tag,
	)
	.await
}

pub async fn event_rankings(
	api: &APIClient,
	event_key: impl AsRef<str>,
	e_tag: Option<String>,
) -> APIResult<Option<EventRanking>> {
	api.get(
		format!("/event/{}/rankings", event_key.as_ref()).as_str(),
		e_tag,
	)
	.await
}

pub async fn regional_champs_pool_points(
	api: &APIClient,
	event_key: impl AsRef<str>,
	e_tag: Option<String>,
) -> APIResult<Option<EventDistrictPoints>> {
	api.get(
		format!("/event/{}/regional_champs_pool_points", event_key.as_ref())
			.as_str(),
		e_tag,
	)
	.await
}

pub async fn event_simple(
	api: &APIClient,
	event_key: impl AsRef<str>,
	e_tag: Option<String>,
) -> APIResult<EventSimple> {
	api.get(
		format!("/event/{}/simple", event_key.as_ref()).as_str(),
		e_tag,
	)
	.await
}

pub async fn event_team_media(
	api: &APIClient,
	event_key: impl AsRef<str>,
	e_tag: Option<String>,
) -> APIResult<Vec<Media>> {
	api.get(
		format!("/event/{}/team_media", event_key.as_ref()).as_str(),
		e_tag,
	)
	.await
}

pub async fn event_teams(
	api: &APIClient,
	event_key: impl AsRef<str>,
	e_tag: Option<String>,
) -> APIResult<Vec<Team>> {
	api.get(
		format!("/event/{}/teams", event_key.as_ref()).as_str(),
		e_tag,
	)
	.await
}

pub async fn event_teams_keys(
	api: &APIClient,
	event_key: impl AsRef<str>,
	e_tag: Option<String>,
) -> APIResult<Vec<String>> {
	api.get(
		format!("/event/{}/teams/keys", event_key.as_ref()).as_str(),
		e_tag,
	)
	.await
}

pub async fn event_teams_simple(
	api: &APIClient,
	event_key: impl AsRef<str>,
	e_tag: Option<String>,
) -> APIResult<Vec<TeamSimple>> {
	api.get(
		format!("/event/{}/teams/simple", event_key.as_ref()).as_str(),
		e_tag,
	)
	.await
}

pub async fn event_teams_statuses(
	api: &APIClient,
	event_key: impl AsRef<str>,
	e_tag: Option<String>,
) -> APIResult<std::collections::HashMap<String, Option<TeamEventStatus>>> {
	api.get(
		format!("/event/{}/teams/statuses", event_key.as_ref()).as_str(),
		e_tag,
	)
	.await
}

pub async fn events_by_year(
	api: &APIClient,
	year: i64,
	e_tag: Option<String>,
) -> APIResult<Vec<Event>> {
	api.get(format!("/events/{}", year).as_str(), e_tag).await
}

pub async fn events_by_year_keys(
	api: &APIClient,
	year: i64,
	e_tag: Option<String>,
) -> APIResult<Vec<String>> {
	api.get(format!("/events/{}/keys", year).as_str(), e_tag)
		.await
}

pub async fn events_by_year_simple(
	api: &APIClient,
	year: i64,
	e_tag: Option<String>,
) -> APIResult<Vec<EventSimple>> {
	api.get(format!("/events/{}/simple", year).as_str(), e_tag)
		.await
}
