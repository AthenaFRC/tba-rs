use proc_macro2::TokenStream;
use quote::{format_ident, quote};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) enum RustType {
	Bool,
	I64,
	F64,
	String,
	Named(String),
	Vec(Box<Self>),
	Map(Box<Self>),
	Option(Box<Self>),
	UnknownJsonObject,
	JsonValue,
}

impl RustType {
	pub(super) fn optional(self) -> Self {
		match self {
			Self::Option(_) => self,
			_ => Self::Option(Box::new(self)),
		}
	}

	pub(super) fn render(&self) -> TokenStream {
		match self {
			Self::Bool => quote!(bool),
			Self::I64 => quote!(i64),
			Self::F64 => quote!(f64),
			Self::String => quote!(String),
			Self::Named(name) => {
				let name = format_ident!("{name}");
				quote!(#name)
			}
			Self::Vec(item) => {
				let item = item.render();
				quote!(Vec<#item>)
			}
			Self::Map(value) => {
				let value = value.render();
				quote!(std::collections::HashMap<String, #value>)
			}
			Self::Option(value) => {
				let value = value.render();
				quote!(Option<#value>)
			}
			Self::UnknownJsonObject => quote!(super::UnknownJsonObject),
			Self::JsonValue => quote!(serde_json::Value),
		}
	}

	pub(super) fn union_variant_name(&self) -> String {
		match self {
			Self::Bool => "Bool".into(),
			Self::I64 => "I64".into(),
			Self::F64 => "F64".into(),
			Self::String => "String".into(),
			Self::Named(name) => name.clone(),
			Self::Vec(value) => {
				let mut depth = 1;
				let mut value = value.as_ref();
				while let Self::Vec(inner) = value {
					depth += 1;
					value = inner;
				}
				if depth == 1 {
					"Vec".into()
				} else {
					format!("Vec{depth}")
				}
			}
			Self::Map(_) => "Map".into(),
			Self::Option(value) => value.union_variant_name(),
			Self::UnknownJsonObject => "UnknownJsonObject".into(),
			Self::JsonValue => "JsonValue".into(),
		}
	}
}
