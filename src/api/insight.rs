use crate::{
	api_client::APIClient,
	api_result::APIResult,
	models::*,
};

pub async fn insights_leaderboards_year(
	api: &APIClient,
	year: i64,
	e_tag: Option<String>,
) -> APIResult<Vec<LeaderboardInsight>> {
	api.get(format!("/insights/leaderboards/{}", year).as_str(), e_tag)
		.await
}

pub async fn insights_notables_year(
	api: &APIClient,
	year: i64,
	e_tag: Option<String>,
) -> APIResult<Vec<NotablesInsight>> {
	api.get(format!("/insights/notables/{}", year).as_str(), e_tag)
		.await
}

pub async fn insights_v2_year(
	api: &APIClient,
	year: i64,
	e_tag: Option<String>,
) -> APIResult<Vec<InsightV2>> {
	api.get(format!("/insights/{}", year).as_str(), e_tag).await
}

pub async fn insights_v2_year_category(
	api: &APIClient,
	year: i64,
	category: impl AsRef<str>,
	e_tag: Option<String>,
) -> APIResult<Vec<InsightV2>> {
	api.get(
		format!("/insights/{}/{}", year, category.as_ref()).as_str(),
		e_tag,
	)
	.await
}

pub async fn insights_v2_year_district(
	api: &APIClient,
	year: i64,
	district_abbreviation: impl AsRef<str>,
	e_tag: Option<String>,
) -> APIResult<Vec<InsightV2>> {
	api.get(
		format!(
			"/insights/{}/district/{}",
			year,
			district_abbreviation.as_ref()
		)
		.as_str(),
		e_tag,
	)
	.await
}

pub async fn insights_v2_year_category_district(
	api: &APIClient,
	year: i64,
	category: impl AsRef<str>,
	district_abbreviation: impl AsRef<str>,
	e_tag: Option<String>,
) -> APIResult<Vec<InsightV2>> {
	api.get(
		format!(
			"/insights/{}/{}/district/{}",
			year,
			category.as_ref(),
			district_abbreviation.as_ref()
		)
		.as_str(),
		e_tag,
	)
	.await
}
