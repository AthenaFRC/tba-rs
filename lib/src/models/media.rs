use serde::{Deserialize, Serialize};

use super::UnknownJsonObject;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MediaAvatarDetails {
	pub base64_image: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MediaCdPhotoThreadDetails {
	pub image_partial: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MediaCdThreadDetails {
	pub thread_title: String,
	pub image_url: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MediaGrabCadDetails {
	pub model_created: String,
	pub model_description: Option<String>,
	pub model_image: String,
	pub model_name: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum MediaNoDetailsType {
	Youtube,
	Imgur,
	FacebookProfile,
	YoutubeChannel,
	TwitterProfile,
	GithubProfile,
	InstagramProfile,
	PeriscopeProfile,
	GitlabProfile,
	InstagramImage,
	ExternalLink,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MediaOnshapeDetails {
	pub model_created: String,
	pub model_description: Option<String>,
	pub model_image: String,
	pub model_name: String,
}

// The OpenAPI discriminator is authoritative here. A derived untagged union
// cannot distinguish missing details or the identical GrabCAD/Onshape shapes.
#[derive(Debug, Clone)]
pub enum Media {
	Avatar {
		foreign_key: String,
		preferred: Option<bool>,
		team_keys: Vec<String>,
		direct_url: Option<String>,
		view_url: Option<String>,
		details: Option<MediaAvatarDetails>,
	},
	CdPhotoThread {
		foreign_key: String,
		preferred: Option<bool>,
		team_keys: Vec<String>,
		direct_url: Option<String>,
		view_url: Option<String>,
		details: Option<MediaCdPhotoThreadDetails>,
	},
	CdThread {
		foreign_key: String,
		preferred: Option<bool>,
		team_keys: Vec<String>,
		direct_url: Option<String>,
		view_url: Option<String>,
		details: Option<MediaCdThreadDetails>,
	},
	GrabCad {
		foreign_key: String,
		preferred: Option<bool>,
		team_keys: Vec<String>,
		direct_url: Option<String>,
		view_url: Option<String>,
		details: Option<MediaGrabCadDetails>,
	},
	NoDetails {
		type_: MediaNoDetailsType,
		foreign_key: String,
		preferred: Option<bool>,
		team_keys: Vec<String>,
		direct_url: Option<String>,
		view_url: Option<String>,
		details: Option<UnknownJsonObject>,
	},
	Onshape {
		foreign_key: String,
		preferred: Option<bool>,
		team_keys: Vec<String>,
		direct_url: Option<String>,
		view_url: Option<String>,
		details: Option<MediaOnshapeDetails>,
	},
}

#[derive(Deserialize)]
struct MediaFields<D> {
	foreign_key: String,
	preferred: Option<bool>,
	team_keys: Vec<String>,
	direct_url: Option<String>,
	view_url: Option<String>,
	details: Option<D>,
}

#[derive(Serialize)]
struct MediaFieldsRef<'a, T, D> {
	#[serde(rename = "type")]
	type_: T,
	foreign_key: &'a str,
	preferred: &'a Option<bool>,
	team_keys: &'a [String],
	direct_url: &'a Option<String>,
	view_url: &'a Option<String>,
	details: &'a Option<D>,
}

impl<'de> Deserialize<'de> for Media {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: serde::Deserializer<'de>,
	{
		use serde::de::Error;

		let value = serde_json::Value::deserialize(deserializer)?;
		let type_ = value
			.get("type")
			.ok_or_else(|| {
				D::Error::custom("missing media discriminator `type`")
			})?
			.as_str()
			.ok_or_else(|| {
				D::Error::custom("media discriminator `type` must be a string")
			})?
			.to_owned();

		macro_rules! deserialize_variant {
			($details:ty, $variant:ident) => {{
				let fields: MediaFields<$details> =
					serde_json::from_value(value).map_err(D::Error::custom)?;
				Self::$variant {
					foreign_key: fields.foreign_key,
					preferred: fields.preferred,
					team_keys: fields.team_keys,
					direct_url: fields.direct_url,
					view_url: fields.view_url,
					details: fields.details,
				}
			}};
		}

		Ok(match type_.as_str() {
			"avatar" => deserialize_variant!(MediaAvatarDetails, Avatar),
			"cdphotothread" => {
				deserialize_variant!(MediaCdPhotoThreadDetails, CdPhotoThread)
			}
			"cd-thread" => deserialize_variant!(MediaCdThreadDetails, CdThread),
			"grabcad" => deserialize_variant!(MediaGrabCadDetails, GrabCad),
			"onshape" => deserialize_variant!(MediaOnshapeDetails, Onshape),
			"youtube" | "imgur" | "facebook-profile" | "youtube-channel"
			| "twitter-profile" | "github-profile" | "instagram-profile"
			| "periscope-profile" | "gitlab-profile" | "instagram-image"
			| "external-link" => {
				let type_ =
					serde_json::from_value(serde_json::Value::String(type_))
						.map_err(D::Error::custom)?;
				let fields: MediaFields<UnknownJsonObject> =
					serde_json::from_value(value).map_err(D::Error::custom)?;
				Self::NoDetails {
					type_,
					foreign_key: fields.foreign_key,
					preferred: fields.preferred,
					team_keys: fields.team_keys,
					direct_url: fields.direct_url,
					view_url: fields.view_url,
					details: fields.details,
				}
			}
			unknown => {
				return Err(D::Error::custom(format!(
					"unknown media discriminator `{unknown}`"
				)));
			}
		})
	}
}

impl Serialize for Media {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: serde::Serializer,
	{
		macro_rules! serialize_variant {
			($type:expr, $fields:expr) => {{
				let (
					foreign_key,
					preferred,
					team_keys,
					direct_url,
					view_url,
					details,
				) = $fields;
				MediaFieldsRef {
					type_: $type,
					foreign_key,
					preferred,
					team_keys,
					direct_url,
					view_url,
					details,
				}
				.serialize(serializer)
			}};
		}

		match self {
			Self::Avatar {
				foreign_key,
				preferred,
				team_keys,
				direct_url,
				view_url,
				details,
			} => serialize_variant!(
				"avatar",
				(
					foreign_key,
					preferred,
					team_keys,
					direct_url,
					view_url,
					details
				)
			),
			Self::CdPhotoThread {
				foreign_key,
				preferred,
				team_keys,
				direct_url,
				view_url,
				details,
			} => serialize_variant!(
				"cdphotothread",
				(
					foreign_key,
					preferred,
					team_keys,
					direct_url,
					view_url,
					details
				)
			),
			Self::CdThread {
				foreign_key,
				preferred,
				team_keys,
				direct_url,
				view_url,
				details,
			} => serialize_variant!(
				"cd-thread",
				(
					foreign_key,
					preferred,
					team_keys,
					direct_url,
					view_url,
					details
				)
			),
			Self::GrabCad {
				foreign_key,
				preferred,
				team_keys,
				direct_url,
				view_url,
				details,
			} => serialize_variant!(
				"grabcad",
				(
					foreign_key,
					preferred,
					team_keys,
					direct_url,
					view_url,
					details
				)
			),
			Self::NoDetails {
				type_,
				foreign_key,
				preferred,
				team_keys,
				direct_url,
				view_url,
				details,
			} => serialize_variant!(
				type_,
				(
					foreign_key,
					preferred,
					team_keys,
					direct_url,
					view_url,
					details
				)
			),
			Self::Onshape {
				foreign_key,
				preferred,
				team_keys,
				direct_url,
				view_url,
				details,
			} => serialize_variant!(
				"onshape",
				(
					foreign_key,
					preferred,
					team_keys,
					direct_url,
					view_url,
					details
				)
			),
		}
	}
}
