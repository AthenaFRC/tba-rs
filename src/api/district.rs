use crate::{
	api_client::APIClient,
	api_result::APIResult,
	models::*,
};

pub async fn district_dcmp_history(
	api: &APIClient,
	district_abbreviation: impl AsRef<str>,
	e_tag: Option<String>,
) -> APIResult<Vec<DistrictDCMPHistoryEntry>> {
	api.get(
		format!("/district/{}/dcmp_history", district_abbreviation.as_ref())
			.as_str(),
		e_tag,
	)
	.await
}

pub async fn district_history(
	api: &APIClient,
	district_abbreviation: impl AsRef<str>,
	e_tag: Option<String>,
) -> APIResult<Vec<District>> {
	api.get(
		format!("/district/{}/history", district_abbreviation.as_ref())
			.as_str(),
		e_tag,
	)
	.await
}

pub async fn district_insights(
	api: &APIClient,
	district_abbreviation: impl AsRef<str>,
	e_tag: Option<String>,
) -> APIResult<DistrictInsight> {
	api.get(
		format!("/district/{}/insights", district_abbreviation.as_ref())
			.as_str(),
		e_tag,
	)
	.await
}

pub async fn district_advancement(
	api: &APIClient,
	district_key: impl AsRef<str>,
	e_tag: Option<String>,
) -> APIResult<Option<DistrictAdvancementByTeam>> {
	api.get(
		format!("/district/{}/advancement", district_key.as_ref()).as_str(),
		e_tag,
	)
	.await
}

pub async fn district_awards(
	api: &APIClient,
	district_key: impl AsRef<str>,
	e_tag: Option<String>,
) -> APIResult<Vec<Award>> {
	api.get(
		format!("/district/{}/awards", district_key.as_ref()).as_str(),
		e_tag,
	)
	.await
}

pub async fn district_events(
	api: &APIClient,
	district_key: impl AsRef<str>,
	e_tag: Option<String>,
) -> APIResult<Vec<Event>> {
	api.get(
		format!("/district/{}/events", district_key.as_ref()).as_str(),
		e_tag,
	)
	.await
}

pub async fn district_events_keys(
	api: &APIClient,
	district_key: impl AsRef<str>,
	e_tag: Option<String>,
) -> APIResult<Vec<String>> {
	api.get(
		format!("/district/{}/events/keys", district_key.as_ref()).as_str(),
		e_tag,
	)
	.await
}

pub async fn district_events_simple(
	api: &APIClient,
	district_key: impl AsRef<str>,
	e_tag: Option<String>,
) -> APIResult<Vec<EventSimple>> {
	api.get(
		format!("/district/{}/events/simple", district_key.as_ref()).as_str(),
		e_tag,
	)
	.await
}

pub async fn district_rankings(
	api: &APIClient,
	district_key: impl AsRef<str>,
	e_tag: Option<String>,
) -> APIResult<Option<Vec<DistrictRanking>>> {
	api.get(
		format!("/district/{}/rankings", district_key.as_ref()).as_str(),
		e_tag,
	)
	.await
}

pub async fn district_teams(
	api: &APIClient,
	district_key: impl AsRef<str>,
	e_tag: Option<String>,
) -> APIResult<Vec<Team>> {
	api.get(
		format!("/district/{}/teams", district_key.as_ref()).as_str(),
		e_tag,
	)
	.await
}

pub async fn district_teams_keys(
	api: &APIClient,
	district_key: impl AsRef<str>,
	e_tag: Option<String>,
) -> APIResult<Vec<String>> {
	api.get(
		format!("/district/{}/teams/keys", district_key.as_ref()).as_str(),
		e_tag,
	)
	.await
}

pub async fn district_teams_simple(
	api: &APIClient,
	district_key: impl AsRef<str>,
	e_tag: Option<String>,
) -> APIResult<Vec<TeamSimple>> {
	api.get(
		format!("/district/{}/teams/simple", district_key.as_ref()).as_str(),
		e_tag,
	)
	.await
}

pub async fn districts_by_year(
	api: &APIClient,
	year: i64,
	e_tag: Option<String>,
) -> APIResult<Vec<District>> {
	api.get(format!("/districts/{}", year).as_str(), e_tag)
		.await
}
