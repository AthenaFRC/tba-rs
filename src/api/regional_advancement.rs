use crate::{
	api_client::APIClient,
	api_result::APIResult,
	models::*,
};

pub async fn regional_advancement(
	api: &APIClient,
	year: i64,
	e_tag: Option<String>,
) -> APIResult<Option<RegionalAdvancementByTeam>> {
	api.get(format!("/regional_advancement/{}", year).as_str(), e_tag)
		.await
}

pub async fn regional_rankings(
	api: &APIClient,
	year: i64,
	e_tag: Option<String>,
) -> APIResult<Option<Vec<RegionalRanking>>> {
	api.get(
		format!("/regional_advancement/{}/rankings", year).as_str(),
		e_tag,
	)
	.await
}
