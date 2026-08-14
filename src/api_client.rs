#[cfg(feature = "rate-limit")]
use governor::{
	DefaultDirectRateLimiter,
	Quota,
	RateLimiter,
};

use crate::api_result::APIResult;

pub const API_KEY_ENV_VAR: &str = "X_TBA_AUTH_KEY";
pub const BASE_API_URL_ENV_VAR: &str = "BASE_API_URL";
pub const BASE_API_URL_FALLBACK: &str =
	"https://www.thebluealliance.com/api/v3";

pub enum APIClientInitializationError {
	ReqwestClientInitializationError(reqwest::Error),
	APIKeyError(String),
}

pub struct APIClient {
	/// The Reqwest client used to facilitate API interaction.
	client: reqwest::Client,

	/// The TBA API key that will be provided for authentication alongside each
	/// request from this API client.
	api_key: String,

	/// The base API URL that will be used for all requests from this API
	/// client.
	base_api_url: String,

	#[cfg(feature = "rate-limit")]
	rate_limiter: Option<DefaultDirectRateLimiter>,
}

impl APIClient {
	pub async fn new() -> Result<APIClient, APIClientInitializationError> {
		Self::new_with(None, None).await
	}

	pub async fn new_with(
		api_key: Option<String>,
		base_api_url: Option<String>,
	) -> Result<APIClient, APIClientInitializationError> {
		Ok(APIClient {
			client: reqwest::Client::builder().build().map_err(
				APIClientInitializationError::ReqwestClientInitializationError,
			)?,
			api_key: api_key
				.or_else(|| std::env::var(API_KEY_ENV_VAR).ok())
				.ok_or_else(|| {
					APIClientInitializationError::APIKeyError(format!(
						"API key must be provided either as an argument or in \
						 the environment variable '{}'.",
						API_KEY_ENV_VAR
					))
				})?,
			base_api_url: base_api_url.unwrap_or_else(|| {
				std::env::var(BASE_API_URL_ENV_VAR)
					.unwrap_or_else(|_| BASE_API_URL_FALLBACK.to_string())
			}),
			#[cfg(feature = "rate-limit")]
			rate_limiter: None,
		})
	}

	#[cfg(feature = "rate-limit")]
	pub fn with_rate_limiter(self, quota: Quota) -> APIClient {
		APIClient {
			client: self.client,
			api_key: self.api_key,
			base_api_url: self.base_api_url,
			rate_limiter: Some(RateLimiter::direct(quota)),
		}
	}

	pub async fn get<T: serde::de::DeserializeOwned>(
		&self,
		url: &str,
		e_tag: Option<String>,
	) -> APIResult<T> {
		let mut headers = reqwest::header::HeaderMap::new();

		headers.insert("X-TBA-Auth-Key", self.api_key.parse().unwrap());

		if let Some(etag) = e_tag {
			headers.insert("If-None-Match", etag.parse().unwrap());
		}

		#[cfg(feature = "rate-limit")]
		if let Some(limiter) = &self.rate_limiter {
			limiter.until_ready().await;
		}

		let response = self
			.client
			.get(self.base_api_url.to_string() + url)
			.headers(headers)
			.send()
			.await;

		match response {
			Ok(response) => APIResult::from_response(response).await,
			Err(err) => APIResult::Err(format!("Request failed: {}", err)),
		}
	}
}
