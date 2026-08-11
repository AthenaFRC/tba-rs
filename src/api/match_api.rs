use crate::{
	api_client::APIClient,
	api_result::APIResult,
	models::*,
};

pub async fn match_(
	api: &APIClient,
	match_key: impl AsRef<str>,
	e_tag: Option<String>,
) -> APIResult<Match> {
	api.get(format!("/match/{}", match_key.as_ref()).as_str(), e_tag)
		.await
}

pub async fn match_simple(
	api: &APIClient,
	match_key: impl AsRef<str>,
	e_tag: Option<String>,
) -> APIResult<MatchSimple> {
	api.get(
		format!("/match/{}/simple", match_key.as_ref()).as_str(),
		e_tag,
	)
	.await
}

pub async fn match_timeseries(
	api: &APIClient,
	match_key: impl AsRef<str>,
	e_tag: Option<String>,
) -> APIResult<Vec<UnknownJsonObject>> {
	api.get(
		format!("/match/{}/timeseries", match_key.as_ref()).as_str(),
		e_tag,
	)
	.await
}

pub async fn match_zebra(
	api: &APIClient,
	match_key: impl AsRef<str>,
	e_tag: Option<String>,
) -> APIResult<Zebra> {
	api.get(
		format!("/match/{}/zebra_motionworks", match_key.as_ref()).as_str(),
		e_tag,
	)
	.await
}
