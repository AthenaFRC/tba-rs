#[rustfmt::skip]
macro_rules! cli_endpoint_manifest {
	(define_enum) => {
		cli_endpoint_manifest!(@entries enum;);
	};

	(define_dispatch) => {
		cli_endpoint_manifest!(@entries dispatch;);
	};

	(@entries $($mode:tt)*) => {
		cli_endpoint_manifest! {
			@consume $($mode)*

			/// Gets a list of DCMP events and awards for the given district abbreviation.
			DistrictDCMPHistory {
				name: "district/dcmp-history",
				function: crate::api::district::district_dcmp_history,
				params: [
					/// The abbreviated district name (e.g. `ne` or `fim`).
					district_abbreviation: String,
				],
			}

			/// Gets a list of District objects with the given district abbreviation.
			///
			/// This accounts for district abbreviation changes, such as MAR to FMA.
			DistrictHistory {
				name: "district/history",
				function: crate::api::district::district_history,
				params: [
					/// The abbreviated district name (e.g. `ne` or `fim`).
					district_abbreviation: String,
				],
			}

			/// Gets insights for a given district.
			DistrictInsights {
				name: "district/insights",
				function: crate::api::district::district_insights,
				params: [
					/// The abbreviated district name (e.g. `ne` or `fim`).
					district_abbreviation: String,
				],
			}

			/// Gets a list of advancement information per team in a district.
			DistrictAdvancement {
				name: "district/advancement",
				function: crate::api::district::district_advancement,
				params: [
					/// The TBA district key (e.g. `2016fim`).
					district_key: String,
				],
			}
		}
	};

	(
		@consume enum;

		$(
			$(#[$variant_meta:meta])*
			$variant:ident {
				name: $name:literal,
				function: $function:path,
				params: [
					$(
						$(#[$param_meta:meta])*
						$param:ident : $param_ty:ty,
					)*
				],
			}
		)*
	) => {
		#[derive(clap::Subcommand, Debug, Clone)]
		#[command(verbatim_doc_comment)]
		#[allow(clippy::enum_variant_names)]
		pub enum CLIEndpoint {
			$(
				$(#[$variant_meta])*
				#[command(name = $name)]
				$variant {
					$(
						$(#[$param_meta])*
						$param: $param_ty,
					)*
				},
			)*
		}
	};

	(
		@consume dispatch;

		$(
			$(#[$variant_meta:meta])*
			$variant:ident {
				name: $name:literal,
				function: $function:path,
				params: [
					$(
						$(#[$param_meta:meta])*
						$param:ident : $param_ty:ty,
					)*
				],
			}
		)*
	) => {
		impl CLIEndpoint {
			pub async fn get(
				self,
				client: &crate::APIClient,
				e_tag: Option<String>,
				format: OutputFormat,
			) -> Result<(), String> {
				match self {
					$(
						CLIEndpoint::$variant { $($param,)* } => crate::app::handlers::print_result(
							$function(client, $($param,)* e_tag).await,
							false,
							format,
						)?,
					)*
				}

				Ok(())
			}
		}
	};
}

pub(crate) use cli_endpoint_manifest;
