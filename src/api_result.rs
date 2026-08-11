use crate::models::APIError;

#[derive(Debug)]
pub enum APIResult<T> {
	Ok { result: T, e_tag: String },
	NotModified,
	Unauthorized,
	Err(String),
}

impl<T> APIResult<T>
where
	T: serde::de::DeserializeOwned,
{
	pub async fn from_response(response: reqwest::Response) -> APIResult<T> {
		match response.status() {
			reqwest::StatusCode::NOT_MODIFIED => APIResult::NotModified,
			reqwest::StatusCode::UNAUTHORIZED => APIResult::Unauthorized,
			reqwest::StatusCode::OK => {
				let e_tag = response
					.headers()
					.get("ETag")
					.map(|v| v.to_str().unwrap_or("").to_string())
					.unwrap_or_default();
				let result = response.json::<T>().await.unwrap();
				APIResult::Ok { result, e_tag }
			}
			_ => APIResult::Err(
				response
					.json::<APIError>()
					.await
					.unwrap_or_else(|_| APIError {
						error: "Failed to read response body".to_string(),
					})
					.error,
			),
		}
	}
}
