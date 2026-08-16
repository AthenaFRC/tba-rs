# tba

`tba` is an async Rust client for
[The Blue Alliance API v3](https://www.thebluealliance.com/apidocs/v3).

The workspace provides generated endpoint accessors, serde response models,
ETag support, optional client-side rate limiting, and a `tba` CLI. It is
intended for FRC tools that want typed access to TBA data without hand-building
request URLs.

## Table of Contents
- [Installation](#installation)
- [Optional Features](#optional-features)
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

## Optional Features

- `rate-limit` enables `APIClient::with_rate_limiter`.
- The `cli` workspace package builds the `tba` command-line client. Run it
  from a checkout with:

  ```sh
  cargo run -p tba-cli -- --help
  ```

  With `X_TBA_AUTH_KEY` set, an endpoint can then be queried with commands such
  as `tba get team simple frc1711`.

## Quick Start

```rust
use tba::{
    endpoints,
    models::TeamSimple,
    APIClient,
    APIResult,
};

// This library relies on `tokio` for async execution, which you will need if
// you do not already depend on it
// (cargo add tokio --features macros,rt-multi-thread).
#[tokio::main]
async fn main() {
	// Create an API client. This will read your TBA API key from the
	// `X_TBA_AUTH_KEY` environment variable.
	let client = match APIClient::new().await {
		Ok(client) => client,
		Err(_) => {
			eprintln!("failed to initialize the TBA API client");
			return;
		},
	};

	// If you have a cached response from a previous request, you can pass the
	// ETag to avoid unnecessary data transfer. If you don't have a cached
	// response, pass `None`.
	let e_tag: Option<String> = None;

	// Call the simple team endpoint for team "frc1711".
	// The data from the endpoint is deserialized into a `TeamSimple` struct,
	// which is wrapped in an `APIResult<T>` struct.
	let response: APIResult<TeamSimple> =
		endpoints::team::simple(&client, "frc1711".to_string(), e_tag).await;

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

The constructor returns a `Result`; on success, it contains the `APIClient`.
Using the environment is preferred because it reduces the risk of accidentally
committing your API key to source control (assuming you remember to add `.env`
to your `.gitignore`).

If you prefer to pass your API key directly, you can do so as follows:

```rust
let client_result = APIClient::new_with(
	Some("your-tba-api-key".to_string()),
	None,
).await;
```

Creating an `APIClient` without specifying an API key reads
`X_TBA_AUTH_KEY` from the environment. If the key is not set, construction
returns `APIClientInitError::APIKeyError`.

If you (for some reason) want to override the base URL used for requests to the
TBA API, you can do so by passing a `base_api_url` to `APIClient::new_with` or
by setting the `BASE_API_URL` environment variable.

## Endpoint Modules

Generated accessors are grouped by the OpenAPI tag used by TBA:

- `endpoints::district`
- `endpoints::event`
- `endpoints::insight`
- `endpoints::match_a_p_i`
- `endpoints::regional_advancement`
- `endpoints::search`
- `endpoints::t_b_a`
- `endpoints::team`

Accessor names are snake case. Examples:

- `endpoints::t_b_a::status(&client, None)`
- `endpoints::team::team(&client, "frc254".to_string(), None)`
- `endpoints::team::events_by_year(&client, "frc254".to_string(), 2024, None)`
- `endpoints::event::matches_simple(&client, "2024casj".to_string(), None)`
- `endpoints::match_a_p_i::zebra(&client, "2024casj_qm1".to_string(), None)`

Each accessor takes `&APIClient`, path parameters, and an optional ETag as the
final argument. Each accessor returns `APIResult<T>`.

## ETags

TBA supports HTTP ETags for caching. Successful responses include the response
ETag:

```rust
let first = endpoints::event::event(
	&client,
	"2024casj".to_string(),
	None,
).await;

if let APIResult::Ok { e_tag, .. } = first {
	let second = endpoints::event::event(
		&client,
		"2024casj".to_string(),
		Some(e_tag),
	).await;

	if matches!(second, APIResult::NotModified) {
		println!("use your cached event");
	}
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

Top-level string and integer enum models are generated from the pinned OpenAPI
document. Regenerate them with
`cargo run -p tba-codegen -- generate`, or verify the checked-in output with
`cargo run -p tba-codegen -- check`. Generator inputs and expected schema
versions, along with Rust compatibility overrides, are pinned in the
workspace-root `codegen.toml`.

## Development

Run the formatter and all checks before submitting changes:

```sh
cargo +nightly fmt
cargo check
cargo test
cargo run -p tba-codegen -- check
cargo clippy --all-targets -- -D warnings
```

## License

Licensed under the GNU Lesser General Public License, version 3 or later. See
[LICENSE](LICENSE).
