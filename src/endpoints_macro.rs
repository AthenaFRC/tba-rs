#[rustfmt::skip]
macro_rules! endpoints {
	($(
		$(#[$domain_meta:meta])*
		$domain:ident {
			$(
				$(#[$endpoint_meta:meta])*
				$endpoint_name_pascal:ident {
					snake_case: $endpoint_name_snake:ident,
					path: $endpoint_path:expr,
					input: {
						$(
							$(#[$field_meta:meta])*
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
			#[command(verbatim_doc_comment)]
			pub enum GetSubcommand {
				$(
					$(#[$domain_meta])*
					#[command(verbatim_doc_comment)]
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
											let result = [<$domain:snake>]::$endpoint_name_snake(
												client,
												$($field,)*
												e_tag,
											).await;
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
				$(#[$domain_meta])*
				pub mod [<$domain:snake>] {
					$(
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
					)*

					#[cfg(feature = "cli")]
					#[derive(clap::Subcommand, Debug, Clone)]
					#[command(verbatim_doc_comment)]
					pub enum [<$domain Subcommand>] {
						$(
							$(#[$endpoint_meta])*
							#[command(verbatim_doc_comment)]
							$endpoint_name_pascal {
								$(
									$(#[$field_meta])*
									#[arg(verbatim_doc_comment)]
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

pub(crate) use endpoints;
