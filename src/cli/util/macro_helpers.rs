#[rustfmt::skip]
macro_rules! __tba_endpoint_cli_name {
	($endpoint_name_pascal:ident, $endpoint_name_snake:ident) => {
		$crate::cli::util::attribute_helpers::__tba_endpoint_cli_name(stringify!($endpoint_name_snake))
	};
	($endpoint_name_pascal:ident,) => {
		paste::paste! {
			$crate::cli::util::attribute_helpers::__tba_endpoint_cli_name(stringify!([<$endpoint_name_pascal:snake>]))
		}
	};
}

pub(crate) use __tba_endpoint_cli_name;
