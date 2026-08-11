use crate::{
	api_client::APIClient,
	api_result::APIResult,
	models::*,
};

pub async fn status(
	api: &APIClient,
	e_tag: Option<String>,
) -> APIResult<APIStatus> {
	api.get("/status", e_tag).await
}
