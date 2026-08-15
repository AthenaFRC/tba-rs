#[rustfmt::skip]
#[cfg(feature = "cli")]
macro_rules! __tba_endpoint_cli_name {
	($endpoint_name_pascal:ident, $endpoint_name_snake:ident) => {
		$crate::__tba_endpoint_cli_name(stringify!($endpoint_name_snake))
	};
	($endpoint_name_pascal:ident,) => {
		paste::paste! {
			$crate::__tba_endpoint_cli_name(stringify!([<$endpoint_name_pascal:snake>]))
		}
	};
}

#[cfg(feature = "cli")]
pub fn __tba_endpoint_cli_name(name: &'static str) -> String {
	name.trim_end_matches('_').replace('_', "-")
}

#[cfg(feature = "cli")]
pub fn __tba_endpoint_doc_comment(lines: &[&'static str]) -> String {
	lines
		.iter()
		.flat_map(|line| line.split('\n'))
		.map(|line| line.strip_prefix(' ').unwrap_or(line).trim())
		.filter(|line| !line.is_empty())
		.collect::<Vec<_>>()
		.join(" ")
}

#[rustfmt::skip]
macro_rules! __tba_endpoint_call {
	(
		$domain:ident,
		$endpoint_name_pascal:ident,
		$endpoint_name_snake:ident;
		$client:expr,
		$e_tag:expr;
		$($field:ident,)*
	) => {
		paste::paste! {
			[<$domain:snake>]::$endpoint_name_snake(
				$client,
				$($field,)*
				$e_tag,
			).await
		}
	};
	(
		$domain:ident,
		$endpoint_name_pascal:ident;
		$client:expr,
		$e_tag:expr;
		$($field:ident,)*
	) => {
		paste::paste! {
			[<$domain:snake>]::[<$endpoint_name_pascal:snake>](
				$client,
				$($field,)*
				$e_tag,
			).await
		}
	};
}

#[rustfmt::skip]
macro_rules! __tba_endpoint_fn {
	(
		$endpoint_name_pascal:ident,
		$endpoint_name_snake:ident;
		path: $endpoint_path:expr,
		input: {
			$(
				$(#[$field_meta:meta])*
				$field:ident: $field_type:ty,
			)*
		},
		output: $output:ty,
	) => {
		pub async fn $endpoint_name_snake(
			client: &$crate::APIClient,
			$(
				$field: $field_type,
			)*
			e_tag: Option<String>,
		) -> $crate::APIResult<$output> {
			client.get(
				format!($endpoint_path).as_str(),
				e_tag
			).await
		}
	};
	(
		$endpoint_name_pascal:ident;
		path: $endpoint_path:expr,
		input: {
			$(
				$(#[$field_meta:meta])*
				$field:ident: $field_type:ty,
			)*
		},
		output: $output:ty,
	) => {
		paste::paste! {
			pub async fn [<$endpoint_name_pascal:snake>](
				client: &$crate::APIClient,
				$(
					$field: $field_type,
				)*
				e_tag: Option<String>,
			) -> $crate::APIResult<$output> {
				client.get(
					format!($endpoint_path).as_str(),
					e_tag
				).await
			}
		}
	};
}

#[rustfmt::skip]
macro_rules! endpoints {
	($(
		$(#[doc = $domain_doc:expr])*
		$domain:ident {
			$(
				$(#[doc = $endpoint_doc:expr])*
				$endpoint_name_pascal:ident {
					$(snake_case: $endpoint_name_snake:ident,)?
					path: $endpoint_path:expr,
					input: {
						$(
							$(#[doc = $field_doc:expr])*
							$field:ident: $field_type:ty,
						)*
					},
					output: $output:ty,
				}
			)*
		}
	)*) => {

		paste::paste! {
			#[cfg(feature = "cli")]
			#[derive(clap::Subcommand, Debug, Clone)]
			pub enum GetSubcommand {
				$(
					#[command(
						about = $crate::__tba_endpoint_doc_comment(&[$($domain_doc),*])
					)]
					$domain {
						#[command(subcommand)]
						endpoint: [<$domain:snake>]::[<$domain Subcommand>],
					},
				)*
			}
			
			#[cfg(feature = "cli")]
			impl GetSubcommand {
				pub async fn get(
					self,
					client: &$crate::APIClient,
					e_tag: Option<String>,
				) -> $crate::APIResult<serde_json::Value> {
					match self {
						$(
							GetSubcommand::$domain { endpoint } => {
								match endpoint {
									$(
										[<$domain:snake>]::[<$domain Subcommand>]::$endpoint_name_pascal { $($field,)* } => {
											let result = __tba_endpoint_call!(
												$domain,
												$endpoint_name_pascal
												$(, $endpoint_name_snake)?;
												client,
												e_tag;
												$($field,)*
											);
											match result {
												$crate::APIResult::Ok { result, e_tag } => {
													let result = match serde_json::to_value(result) {
														Ok(result) => result,
														Err(e) => return $crate::APIResult::Err(format!("Failed to serialize result to JSON: {}", e)),
													};
													$crate::APIResult::Ok { result, e_tag }
												},
												$crate::APIResult::NotModified =>
													$crate::APIResult::NotModified,
												$crate::APIResult::Unauthorized =>
													$crate::APIResult::Unauthorized,
												$crate::APIResult::Err(error) =>
													$crate::APIResult::Err(error),
											}
										}
									)*
								}
							},
						)*
					}
				}
			}

			$(
				$(#[doc = $domain_doc])*
				pub mod [<$domain:snake>] {
					$(
						__tba_endpoint_fn! {
							$endpoint_name_pascal
							$(, $endpoint_name_snake)?;
							path: $endpoint_path,
							input: {
								$(
									$(#[doc = $field_doc])*
									$field: $field_type,
								)*
							},
							output: $output,
						}
					)*

					#[cfg(feature = "cli")]
					#[derive(clap::Subcommand, Debug, Clone)]
					pub enum [<$domain Subcommand>] {
						$(
							#[command(
								name = __tba_endpoint_cli_name!(
									$endpoint_name_pascal,
									$($endpoint_name_snake)?
								),
								about = $crate::__tba_endpoint_doc_comment(&[$($endpoint_doc),*]),
							)]
							$endpoint_name_pascal {
								$(
									#[arg(
										help = $crate::__tba_endpoint_doc_comment(&[$($field_doc),*])
									)]
									$field: $field_type,
								)*
							},
						)*
					}

				}

			)*
		}
	};
}
