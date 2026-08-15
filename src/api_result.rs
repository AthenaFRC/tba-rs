use crate::{
	models::APIError,
	util::response_body_excerpt::response_body_excerpt,
};

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
			"Failed to decode {} response from {}: {}; response body \
			 excerpt:\n{}",
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

#[cfg(test)]
mod tests {
	use std::{
		io::{
			Read,
			Write,
		},
		net::TcpListener,
		thread,
	};

	use serde::Deserialize;

	use super::{
		APIResponseDecodeError,
		APIResult,
	};
	use crate::util::response_body_excerpt::RESPONSE_BODY_EXCERPT_MAX_BYTES;

	#[derive(Debug, Deserialize)]
	struct ExpectedObject {
		#[serde(rename = "value")]
		_value: u64,
	}

	#[derive(Debug, Deserialize)]
	#[serde(rename_all = "snake_case")]
	enum ExpectedEnum {
		Known,
	}

	async fn successful_response(body: &str) -> reqwest::Response {
		let listener = TcpListener::bind("127.0.0.1:0").unwrap();
		let address = listener.local_addr().unwrap();
		let body = body.to_owned();
		let server = thread::spawn(move || {
			let (mut stream, _) = listener.accept().unwrap();
			let mut request = Vec::new();
			let mut buffer = [0; 1024];
			loop {
				let bytes_read = stream.read(&mut buffer).unwrap();
				request.extend_from_slice(&buffer[..bytes_read]);
				if bytes_read == 0
					|| request.windows(4).any(|window| window == b"\r\n\r\n")
				{
					break;
				}
			}
			write!(
				stream,
				"HTTP/1.1 200 OK\r\nContent-Type: \
				 application/json\r\nContent-Length: {}\r\nConnection: \
				 close\r\n\r\n{}",
				body.len(),
				body,
			)
			.unwrap();
		});

		let response = reqwest::get(format!(
			"http://{address}/response?api_key=super-secret"
		))
		.await
		.unwrap();
		server.join().unwrap();
		response
	}

	fn decode_error<T: std::fmt::Debug>(
		result: APIResult<T>,
	) -> APIResponseDecodeError {
		match result {
			APIResult::DecodeError(error) => error,
			other => panic!("expected decode error, got {other:?}"),
		}
	}

	#[tokio::test]
	async fn malformed_json_returns_decode_error() {
		let response = successful_response("{not-json").await;
		let error = decode_error(
			APIResult::<ExpectedObject>::from_response(response).await,
		);

		assert_eq!(error.status, reqwest::StatusCode::OK);
		assert_eq!(error.response_body_excerpt, "{\n  not-json");
		assert!(error.source.is_syntax());
	}

	#[tokio::test]
	async fn empty_body_returns_decode_error() {
		let response = successful_response("").await;
		let error = decode_error(
			APIResult::<ExpectedObject>::from_response(response).await,
		);

		assert!(error.response_body_excerpt.is_empty());
		assert!(error.source.is_eof());
	}

	#[tokio::test]
	async fn schema_mismatch_returns_decode_error() {
		let body = r#"{"value":"not-a-number"}"#;
		let response = successful_response(body).await;
		let error = decode_error(
			APIResult::<ExpectedObject>::from_response(response).await,
		);

		assert_eq!(
			error.response_body_excerpt,
			"{\n  \"value\": \"not-a-number\"\n}"
		);
		assert!(error.to_string().contains(
			"response body excerpt:\n{\n  \"value\": \"not-a-number\"\n}"
		));
		assert!(error.source.is_data());
	}

	#[tokio::test]
	async fn unknown_enum_value_returns_decode_error() {
		let response = successful_response(r#""future_value""#).await;
		let error = decode_error(
			APIResult::<ExpectedEnum>::from_response(response).await,
		);

		assert!(error.source.is_data());
		assert!(error.source.to_string().contains("unknown variant"));
	}

	#[tokio::test]
	async fn decode_error_context_is_bounded_and_sanitized() {
		let body = "x".repeat(RESPONSE_BODY_EXCERPT_MAX_BYTES + 100);
		let response = successful_response(&body).await;
		let error = decode_error(
			APIResult::<ExpectedObject>::from_response(response).await,
		);

		assert_eq!(error.request_url.path(), "/response");
		assert!(error.request_url.query().is_none());
		assert!(!error.to_string().contains("super-secret"));
		assert_eq!(
			error.response_body_excerpt.chars().count(),
			RESPONSE_BODY_EXCERPT_MAX_BYTES + 1
		);
		assert!(error.response_body_excerpt.ends_with('…'));
	}
}
