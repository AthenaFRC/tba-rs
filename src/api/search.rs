use crate::{
	api_client::APIClient,
	api_result::APIResult,
	models::*,
};

pub async fn search_index(
	api: &APIClient,
	e_tag: Option<String>,
) -> APIResult<SearchIndex> {
	api.get("/search_index", e_tag).await
}
