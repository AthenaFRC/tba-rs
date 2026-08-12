#[derive(clap::Subcommand, Debug, Clone)]
#[command(verbatim_doc_comment)]
#[allow(clippy::enum_variant_names)]
pub enum CLIEndpoint {
	/// Gets a list of DCMP events and awards for the given district.
	/// abbreviation.
	DistrictDCMPHistory {
		/// The abbreviated district name (e.g. `ne` or `fim`).
		district_abbreviation: String,
	},

	/// Gets a list of District objects with the given district abbreviation.
	/// This accounts for district abbreviation changes, such as MAR to FMA.
	DistrictHistory {
		/// The abbreviated district name (e.g. `ne` or `fim`).
		district_abbreviation: String,
	},

	/// Gets insights for a given district.
	DistrictInsights {
		/// The abbreviated district name (e.g. `ne` or `fim`).
		district_abbreviation: String,
	},

	/// Gets a list of advancement information per team in a district.
	DistrictAdvancement {
		/// The TBA district key (e.g. `2016fim`).
		district_key: String,
	},
}
