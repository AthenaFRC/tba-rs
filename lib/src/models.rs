// These models are generated from The Blue Alliance OpenAPI v3 spec. Run
// `cargo regen-models` from the workspace root after updating the pinned spec
// or code generator. Models requiring discriminator-aware serde behavior live
// in focused manual modules alongside the generated output.

#![allow(clippy::struct_excessive_bools)]

use serde::{Deserialize, Serialize};

mod generated;
mod insight_v2;
mod media;

pub use generated::*;
pub use insight_v2::*;
pub use media::*;

pub type UnknownJsonObject =
	std::collections::HashMap<String, serde_json::Value>;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct APIError {
	#[serde(rename = "Error")]
	pub error: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DistrictDCMPHistoryEntry {
	pub awards: Option<Vec<Award>>,
	pub event: Option<Event>,
}

pub type DistrictAdvancementByTeam =
	std::collections::HashMap<String, DistrictAdvancement>;
pub type RegionalAdvancementByTeam =
	std::collections::HashMap<String, RegionalAdvancement>;
