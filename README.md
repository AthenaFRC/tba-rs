# tba

`tba` is an async Rust client for
[The Blue Alliance API v3](https://www.thebluealliance.com/apidocs/v3).

The crate provides generated endpoint accessors, serde response models, ETag
support, and a small `tba` binary for local smoke testing. It is intended for
FRC tools that want typed access to TBA data without hand-building request URLs.

## Table of Contents
- [Installation](#installation)
- [Quick Start](#quick-start)
- [Authentication](#authentication)
- [Endpoint Modules](#endpoint-modules)
- [ETags](#etags)
- [Models](#models)
- [Development](#development)
- [License](#license)

## Installation

```sh
cargo add tba
```

or add it to your `Cargo.toml`:

```toml
[dependencies]
tba = "0.1"
```

## Quick Start

```rust
use tba::{
    api,
    APIClient,
    APIResult,
};

// This library relies on `tokio` for async execution, which you will need if
// you do not already depend on it (cargo add tokio --features=full).
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	
	if let Err(error) = dotenvy::dotenv() {
		println!("Did you configure your API key in a .env file? ({})", error);
	}
	
	// Create an API client. This will read your TBA API key from the
	// `X_TBA_AUTH_KEY` environment variable.
	let client = APIClient::new().await?;
	
	// If you have a cached response from a previous request, you can pass the
	// ETag to avoid unnecessary data transfer. If you don't have a cached
	// response, pass `None`.
	let e_tag: Option<String> = Some("etag-from-previous-request".to_string());
	
	// Call the `team_simple` endpoint for team "frc1711".
	// The data from the endpoint is deserialized into a `TeamSimple` struct,
	// which is wrapped in an `APIResult<T>` struct.
	let response: APIResult<TeamSimple> =
		api::team::team_simple(&client, "frc1711", e_tag).await;
	
	match response {
		
		// If the request was successful, we can destructure the `APIResult`
		// into its Ok variant, which contains the deserialized `TeamSimple`
		// struct and the ETag for the response.
		APIResult::Ok { result, e_tag } => {
		println!("{}: {}", result.key, result.nickname);
		println!("etag: {e_tag}");
		},
		
		// If we provided an ETag that the server determined was still valid,
		// we'll get a `NotModified` response, which means that the data we
		// received alongside the aforementioned ETag is still fresh and can
		// be used instead of ingesting a new response.
		APIResult::NotModified => println!("cached response is still fresh"),
		
		// If our API key was invalid, we'll get an `Unauthorized` response.
		// This is a good time to check that your `X_TBA_AUTH_KEY` environment
		// variable is set correctly.
		APIResult::Unauthorized => eprintln!("invalid TBA API key"),
		
		// If the request failed for any other reason, we'll get an `Err`
		// response, which contains the error message.
		APIResult::Err(error) => eprintln!("request failed: {error}"),
		
	}
	
	Ok(())

}
```

## Authentication

Access to the TBA API requires an authorization key. Create an API key from your
The Blue Alliance [account page](https://www.thebluealliance.com/account).

Once you have your key, set the `X_TBA_AUTH_KEY` environment variable (you can
use a library such as `dotenvy` to load it from a `.env` file) and simply call:

```rust
let client = APIClient::new().await?;
```

This is the preferred way to create an `APIClient` instance, as it reduces the
risk of accidentally committing your API key to source control (assuming you
remember to add `.env` to your `.gitignore`).

If you prefer to pass your API key directly, you can do so as follows:

```rust
let client = APIClient::new_with(
	Some("your-tba-api-key".to_string()),
	None,
).await?;
```

Creating instances of `tba::APIClient` without specifying an API key will
read `X_TBA_AUTH_KEY` from the environment, and will cause a `panic` if the key
is not set.

If you (for some reason) want to override the base URL used for requests to the
TBA API, you can do so by passing a `base_api_url` to `APIClient::new_with` or
by setting the `BASE_API_URL` environment variable.

## Endpoint Modules

Generated accessors are grouped by the OpenAPI tag used by TBA:

- `api::district`
- `api::event`
- `api::insight`
- `api::match_api`
- `api::regional_advancement`
- `api::search`
- `api::tba`
- `api::team`

Function names follow the TBA operation names in snake case with the leading
`get` removed. Examples:

- `api::tba::status(&client, None)`
- `api::team::team(&client, "frc254", None)`
- `api::team::team_events_by_year(&client, "frc254", 2024, None)`
- `api::event::event_matches_simple(&client, "2024casj", None)`
- `api::match_api::match_zebra(&client, "2024casj_qm1", None)`

Each accessor takes `&APIClient`, path parameters, and an optional ETag as the
final argument. Each accessor returns `APIResult<T>`.

## ETags

TBA supports HTTP ETags for caching. Successful responses include the response
ETag:

```rust
let first = api::event::event(&client, "2024casj", None).await;

let APIResult::Ok { e_tag, .. } = first else {
	return Ok(());
};

let second = api::event::event(&client, "2024casj", Some(e_tag)).await;

if matches!(second, APIResult::NotModified) {
	println!("use your cached event");
}
```

## Models

Response models are exported from `tba::models`.

```rust
use tba::models::{
	Event,
	Match,
	Team,
};
```

Most models are strongly typed from the OpenAPI schema. A few intentionally
dynamic payloads use `UnknownJsonObject`, including event predictions, some
year-specific event insights, media entries with no details schema, and match
timeseries objects. Those parts of the TBA API are either explicitly
year-specific or declared as empty objects in the upstream schema.

## License

Licensed under the GNU Lesser General Public License, version 3 or later. See
[LICENSE](LICENSE).
