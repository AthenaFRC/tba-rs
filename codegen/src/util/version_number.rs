use serde::de::Error;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt::Display;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct VersionNumber {
	pub major: u8,
	pub minor: u8,
	pub patch: u8,
}

impl VersionNumber {
	pub const fn new(major: u8, minor: u8, patch: u8) -> Self {
		Self {
			major,
			minor,
			patch,
		}
	}
}

impl FromStr for VersionNumber {
	type Err = String;

	fn from_str(value: &str) -> Result<Self, Self::Err> {
		let components = value.split('.').collect::<Vec<_>>();
		if components.len() != 3 {
			return Err(version_error(value));
		}
		let parse_component = |component: &str| {
			component.parse::<u8>().map_err(|_| version_error(value))
		};
		Ok(Self {
			major: parse_component(components[0])?,
			minor: parse_component(components[1])?,
			patch: parse_component(components[2])?,
		})
	}
}

impl TryFrom<&str> for VersionNumber {
	type Error = String;

	fn try_from(value: &str) -> Result<Self, Self::Error> {
		value.parse()
	}
}

impl Display for VersionNumber {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
	}
}

impl Serialize for VersionNumber {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: Serializer,
	{
		serializer.serialize_str(&self.to_string())
	}
}

impl<'de> Deserialize<'de> for VersionNumber {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: Deserializer<'de>,
	{
		String::deserialize(deserializer)?
			.parse()
			.map_err(D::Error::custom)
	}
}

fn version_error(value: &str) -> String {
	format!(
		"invalid version `{value}`; expected MAJOR.MINOR.PATCH with components from 0 to 255"
	)
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn parses_and_displays_version_numbers() {
		let version = VersionNumber::try_from("3.16.0").unwrap();
		assert_eq!(version, VersionNumber::new(3, 16, 0));
		assert_eq!(version.to_string(), "3.16.0");
		assert!(VersionNumber::try_from("3.16").is_err());
		assert!(VersionNumber::try_from("3.256.0").is_err());
	}
}
