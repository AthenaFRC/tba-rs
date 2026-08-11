use crate::{
	api_client::APIClient,
	api_result::APIResult,
	models::*,
};

pub async fn team(
	api: &APIClient,
	team_key: impl AsRef<str>,
	e_tag: Option<String>,
) -> APIResult<Team> {
	api.get(format!("/team/{}", team_key.as_ref()).as_str(), e_tag)
		.await
}

pub async fn team_awards(
	api: &APIClient,
	team_key: impl AsRef<str>,
	e_tag: Option<String>,
) -> APIResult<Vec<Award>> {
	api.get(
		format!("/team/{}/awards", team_key.as_ref()).as_str(),
		e_tag,
	)
	.await
}

pub async fn team_awards_by_year(
	api: &APIClient,
	team_key: impl AsRef<str>,
	year: i64,
	e_tag: Option<String>,
) -> APIResult<Vec<Award>> {
	api.get(
		format!("/team/{}/awards/{}", team_key.as_ref(), year).as_str(),
		e_tag,
	)
	.await
}

pub async fn team_districts(
	api: &APIClient,
	team_key: impl AsRef<str>,
	e_tag: Option<String>,
) -> APIResult<Vec<District>> {
	api.get(
		format!("/team/{}/districts", team_key.as_ref()).as_str(),
		e_tag,
	)
	.await
}

pub async fn team_event_awards(
	api: &APIClient,
	team_key: impl AsRef<str>,
	event_key: impl AsRef<str>,
	e_tag: Option<String>,
) -> APIResult<Vec<Award>> {
	api.get(
		format!(
			"/team/{}/event/{}/awards",
			team_key.as_ref(),
			event_key.as_ref()
		)
		.as_str(),
		e_tag,
	)
	.await
}

pub async fn team_event_matches(
	api: &APIClient,
	team_key: impl AsRef<str>,
	event_key: impl AsRef<str>,
	e_tag: Option<String>,
) -> APIResult<Vec<Match>> {
	api.get(
		format!(
			"/team/{}/event/{}/matches",
			team_key.as_ref(),
			event_key.as_ref()
		)
		.as_str(),
		e_tag,
	)
	.await
}

pub async fn team_event_matches_keys(
	api: &APIClient,
	team_key: impl AsRef<str>,
	event_key: impl AsRef<str>,
	e_tag: Option<String>,
) -> APIResult<Vec<String>> {
	api.get(
		format!(
			"/team/{}/event/{}/matches/keys",
			team_key.as_ref(),
			event_key.as_ref()
		)
		.as_str(),
		e_tag,
	)
	.await
}

pub async fn team_event_matches_simple(
	api: &APIClient,
	team_key: impl AsRef<str>,
	event_key: impl AsRef<str>,
	e_tag: Option<String>,
) -> APIResult<Vec<Match>> {
	api.get(
		format!(
			"/team/{}/event/{}/matches/simple",
			team_key.as_ref(),
			event_key.as_ref()
		)
		.as_str(),
		e_tag,
	)
	.await
}

pub async fn team_event_status(
	api: &APIClient,
	team_key: impl AsRef<str>,
	event_key: impl AsRef<str>,
	e_tag: Option<String>,
) -> APIResult<Option<TeamEventStatus>> {
	api.get(
		format!(
			"/team/{}/event/{}/status",
			team_key.as_ref(),
			event_key.as_ref()
		)
		.as_str(),
		e_tag,
	)
	.await
}

pub async fn team_events(
	api: &APIClient,
	team_key: impl AsRef<str>,
	e_tag: Option<String>,
) -> APIResult<Vec<Event>> {
	api.get(
		format!("/team/{}/events", team_key.as_ref()).as_str(),
		e_tag,
	)
	.await
}

pub async fn team_events_keys(
	api: &APIClient,
	team_key: impl AsRef<str>,
	e_tag: Option<String>,
) -> APIResult<Vec<String>> {
	api.get(
		format!("/team/{}/events/keys", team_key.as_ref()).as_str(),
		e_tag,
	)
	.await
}

pub async fn team_events_simple(
	api: &APIClient,
	team_key: impl AsRef<str>,
	e_tag: Option<String>,
) -> APIResult<Vec<EventSimple>> {
	api.get(
		format!("/team/{}/events/simple", team_key.as_ref()).as_str(),
		e_tag,
	)
	.await
}

pub async fn team_events_by_year(
	api: &APIClient,
	team_key: impl AsRef<str>,
	year: i64,
	e_tag: Option<String>,
) -> APIResult<Vec<Event>> {
	api.get(
		format!("/team/{}/events/{}", team_key.as_ref(), year).as_str(),
		e_tag,
	)
	.await
}

pub async fn team_events_by_year_keys(
	api: &APIClient,
	team_key: impl AsRef<str>,
	year: i64,
	e_tag: Option<String>,
) -> APIResult<Vec<String>> {
	api.get(
		format!("/team/{}/events/{}/keys", team_key.as_ref(), year).as_str(),
		e_tag,
	)
	.await
}

pub async fn team_events_by_year_simple(
	api: &APIClient,
	team_key: impl AsRef<str>,
	year: i64,
	e_tag: Option<String>,
) -> APIResult<Vec<EventSimple>> {
	api.get(
		format!("/team/{}/events/{}/simple", team_key.as_ref(), year).as_str(),
		e_tag,
	)
	.await
}

pub async fn team_events_statuses_by_year(
	api: &APIClient,
	team_key: impl AsRef<str>,
	year: i64,
	e_tag: Option<String>,
) -> APIResult<std::collections::HashMap<String, Option<TeamEventStatus>>> {
	api.get(
		format!("/team/{}/events/{}/statuses", team_key.as_ref(), year)
			.as_str(),
		e_tag,
	)
	.await
}

pub async fn team_history(
	api: &APIClient,
	team_key: impl AsRef<str>,
	e_tag: Option<String>,
) -> APIResult<History> {
	api.get(
		format!("/team/{}/history", team_key.as_ref()).as_str(),
		e_tag,
	)
	.await
}

pub async fn team_matches_by_year(
	api: &APIClient,
	team_key: impl AsRef<str>,
	year: i64,
	e_tag: Option<String>,
) -> APIResult<Vec<Match>> {
	api.get(
		format!("/team/{}/matches/{}", team_key.as_ref(), year).as_str(),
		e_tag,
	)
	.await
}

pub async fn team_matches_by_year_keys(
	api: &APIClient,
	team_key: impl AsRef<str>,
	year: i64,
	e_tag: Option<String>,
) -> APIResult<Vec<String>> {
	api.get(
		format!("/team/{}/matches/{}/keys", team_key.as_ref(), year).as_str(),
		e_tag,
	)
	.await
}

pub async fn team_matches_by_year_simple(
	api: &APIClient,
	team_key: impl AsRef<str>,
	year: i64,
	e_tag: Option<String>,
) -> APIResult<Vec<MatchSimple>> {
	api.get(
		format!("/team/{}/matches/{}/simple", team_key.as_ref(), year).as_str(),
		e_tag,
	)
	.await
}

pub async fn team_media_by_tag(
	api: &APIClient,
	team_key: impl AsRef<str>,
	media_tag: impl AsRef<str>,
	e_tag: Option<String>,
) -> APIResult<Vec<Media>> {
	api.get(
		format!(
			"/team/{}/media/tag/{}",
			team_key.as_ref(),
			media_tag.as_ref()
		)
		.as_str(),
		e_tag,
	)
	.await
}

pub async fn team_media_by_tag_year(
	api: &APIClient,
	team_key: impl AsRef<str>,
	media_tag: impl AsRef<str>,
	year: i64,
	e_tag: Option<String>,
) -> APIResult<Vec<Media>> {
	api.get(
		format!(
			"/team/{}/media/tag/{}/{}",
			team_key.as_ref(),
			media_tag.as_ref(),
			year
		)
		.as_str(),
		e_tag,
	)
	.await
}

pub async fn team_media_by_year(
	api: &APIClient,
	team_key: impl AsRef<str>,
	year: i64,
	e_tag: Option<String>,
) -> APIResult<Vec<Media>> {
	api.get(
		format!("/team/{}/media/{}", team_key.as_ref(), year).as_str(),
		e_tag,
	)
	.await
}

pub async fn team_robots(
	api: &APIClient,
	team_key: impl AsRef<str>,
	e_tag: Option<String>,
) -> APIResult<Vec<TeamRobot>> {
	api.get(
		format!("/team/{}/robots", team_key.as_ref()).as_str(),
		e_tag,
	)
	.await
}

pub async fn team_simple(
	api: &APIClient,
	team_key: impl AsRef<str>,
	e_tag: Option<String>,
) -> APIResult<TeamSimple> {
	api.get(
		format!("/team/{}/simple", team_key.as_ref()).as_str(),
		e_tag,
	)
	.await
}

pub async fn team_social_media(
	api: &APIClient,
	team_key: impl AsRef<str>,
	e_tag: Option<String>,
) -> APIResult<Vec<Media>> {
	api.get(
		format!("/team/{}/social_media", team_key.as_ref()).as_str(),
		e_tag,
	)
	.await
}

pub async fn team_years_participated(
	api: &APIClient,
	team_key: impl AsRef<str>,
	e_tag: Option<String>,
) -> APIResult<Vec<i64>> {
	api.get(
		format!("/team/{}/years_participated", team_key.as_ref()).as_str(),
		e_tag,
	)
	.await
}

pub async fn teams(
	api: &APIClient,
	page_num: i64,
	e_tag: Option<String>,
) -> APIResult<Vec<Team>> {
	api.get(format!("/teams/{}", page_num).as_str(), e_tag)
		.await
}

pub async fn teams_keys(
	api: &APIClient,
	page_num: i64,
	e_tag: Option<String>,
) -> APIResult<Vec<String>> {
	api.get(format!("/teams/{}/keys", page_num).as_str(), e_tag)
		.await
}

pub async fn teams_simple(
	api: &APIClient,
	page_num: i64,
	e_tag: Option<String>,
) -> APIResult<Vec<TeamSimple>> {
	api.get(format!("/teams/{}/simple", page_num).as_str(), e_tag)
		.await
}

pub async fn teams_by_year(
	api: &APIClient,
	year: i64,
	page_num: i64,
	e_tag: Option<String>,
) -> APIResult<Vec<Team>> {
	api.get(format!("/teams/{}/{}", year, page_num).as_str(), e_tag)
		.await
}

pub async fn teams_by_year_keys(
	api: &APIClient,
	year: i64,
	page_num: i64,
	e_tag: Option<String>,
) -> APIResult<Vec<String>> {
	api.get(format!("/teams/{}/{}/keys", year, page_num).as_str(), e_tag)
		.await
}

pub async fn teams_by_year_simple(
	api: &APIClient,
	year: i64,
	page_num: i64,
	e_tag: Option<String>,
) -> APIResult<Vec<TeamSimple>> {
	api.get(
		format!("/teams/{}/{}/simple", year, page_num).as_str(),
		e_tag,
	)
	.await
}
