use std::{
	io::{Read, Write},
	net::TcpListener,
	thread,
};

use serde_json::Value;
use tba::{
	APIClient, APIResult, endpoints,
	models::{Match, MatchSimple},
};

const SIMPLE_MATCH_FIELDS: [&str; 10] = [
	"key",
	"event_key",
	"comp_level",
	"set_number",
	"match_number",
	"alliances",
	"winning_alliance",
	"time",
	"actual_time",
	"predicted_time",
];

// This fixture reproduces the authenticated `test_team_event_matches`
// contract from The Blue Alliance at commit
// d9724f8479116cdb7a87989e24657f5dcf0621a0. That test constructs these two
// matches, calls this exact endpoint, and validates each response object
// against `simple_match_properties` in
// `src/backend/api/handlers/helpers/model_properties.py`.
const FIXTURE: &str =
	include_str!("fixtures/api_v3_16_0/team_event_matches_simple.json");

fn serve_fixture() -> (String, thread::JoinHandle<String>) {
	let listener = TcpListener::bind("127.0.0.1:0").unwrap();
	let address = listener.local_addr().unwrap();
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
			"HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nETag: \
			 fixture-etag\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
			FIXTURE.len(),
			FIXTURE,
		)
		.unwrap();

		String::from_utf8(request).unwrap()
	});
	(format!("http://{address}"), server)
}

#[tokio::test]
async fn team_event_matches_simple_fixture_uses_short_wire_shape() {
	let wire_value: Value = serde_json::from_str(FIXTURE).unwrap();
	let (base_url, server) = serve_fixture();
	let client =
		APIClient::new_with(Some("fixture-api-key".to_owned()), Some(base_url))
			.await
			.ok()
			.unwrap();
	let response: APIResult<Vec<MatchSimple>> =
		endpoints::team::event_matches_simple(
			&client,
			"frc254".to_owned(),
			"2020casj".to_owned(),
			None,
		)
		.await;
	let request = server.join().unwrap();

	assert!(request.starts_with(
		"GET /team/frc254/event/2020casj/matches/simple HTTP/1.1\r\n"
	));
	let (matches, e_tag) = match response {
		APIResult::Ok { result, e_tag } => (result, e_tag),
		other => panic!("expected fixture response to decode, got {other:?}"),
	};

	assert_eq!(matches.len(), 2);
	assert_eq!(e_tag, "fixture-etag");
	for match_value in wire_value.as_array().unwrap() {
		let match_object = match_value.as_object().unwrap();
		assert_eq!(match_object.len(), SIMPLE_MATCH_FIELDS.len());
		for field in SIMPLE_MATCH_FIELDS {
			assert!(match_object.contains_key(field));
		}
		assert!(!match_object.contains_key("score_breakdown"));
		assert!(!match_object.contains_key("videos"));
		assert!(!match_object.contains_key("post_result_time"));
	}

	assert!(serde_json::from_value::<Vec<Match>>(wire_value.clone()).is_err());
	assert_eq!(serde_json::to_value(matches).unwrap(), wire_value);
}
