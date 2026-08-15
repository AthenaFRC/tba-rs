use crate::models::APIError;

const RESPONSE_BODY_EXCERPT_MAX_BYTES: usize = 1024;

#[derive(Debug)]
pub struct APIResponseDecodeError {
	pub status: reqwest::StatusCode,
	pub request_url: reqwest::Url,
	pub source: serde_json::Error,
	pub response_body_excerpt: String,
}

impl std::fmt::Display for APIResponseDecodeError {
	fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(
			formatter,
			"Failed to decode {} response from {}: {}; response body excerpt: \
			 {:?}",
			self.status,
			self.request_url,
			self.source,
			self.response_body_excerpt,
		)
	}
}

impl std::error::Error for APIResponseDecodeError {
	fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
		Some(&self.source)
	}
}

#[derive(Debug)]
pub enum APIResult<T> {
	Ok { result: T, e_tag: String },
	NotModified,
	Unauthorized,
	DecodeError(APIResponseDecodeError),
	Err(String),
}

fn sanitized_request_url(response: &reqwest::Response) -> reqwest::Url {
	let mut request_url = response.url().clone();
	let _ = request_url.set_username("");
	let _ = request_url.set_password(None);
	request_url.set_query(None);
	request_url.set_fragment(None);
	request_url
}

fn response_body_excerpt(body: &[u8]) -> String {
	let excerpt_end = body.len().min(RESPONSE_BODY_EXCERPT_MAX_BYTES);
	let mut excerpt =
		String::from_utf8_lossy(&body[..excerpt_end]).into_owned();
	if body.len() > RESPONSE_BODY_EXCERPT_MAX_BYTES {
		excerpt.push('…');
	}
	excerpt
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
				let status = response.status();
				let request_url = sanitized_request_url(&response);
				let body = match response.bytes().await {
					Ok(body) => body,
					Err(error) => {
						return APIResult::Err(format!(
							"Failed to read {status} response body from \
							 {request_url}: {error}"
						));
					}
				};
				match serde_json::from_slice::<T>(&body) {
					Ok(result) => APIResult::Ok { result, e_tag },
					Err(source) => {
						APIResult::DecodeError(APIResponseDecodeError {
							status,
							request_url,
							source,
							response_body_excerpt: response_body_excerpt(&body),
						})
					}
				}
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
