#![doc = include_str!("../README.md")]

extern crate proc_macro;
use proc_macro::TokenStream;

use quote::quote;
use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::{parse_macro_input, DeriveInput, GenericParam, Generics, TypeParamBound};

macro_rules! derive_fmt_as_json {
    ($tr:path, $name:ident, $generics:ident, $params:ident, $function:path) => {
        quote! {
            impl<#$params> $tr for #$name #$generics {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    let s = $function(&self)
                        .map_err(|_| std::fmt::Error::default())?;
                    write!(f, "{}", s)
                }
            }
        }
    };
}

fn serialize_in_generics(g: &Generics) -> Punctuated<GenericParam, Comma> {
    let mut params = Punctuated::<GenericParam, Comma>::new();

    for param in g.params.iter() {
        let param = match param {
            GenericParam::Type(typ) => {
                let mut typ = typ.clone();
                typ.bounds.push(TypeParamBound::Trait(
                    syn::parse_str("serde::Serialize").unwrap(),
                ));
                GenericParam::Type(typ.clone())
            }
            _ => param.clone(),
        };
        params.push(param);
    }

    params
}

fn deserialize_in_generics(g: &Generics) -> Punctuated<GenericParam, Comma> {
    let mut params = Punctuated::<GenericParam, Comma>::new();

    for param in g.params.iter() {
        let param = match param {
            GenericParam::Type(typ) => {
                let mut typ = typ.clone();
                typ.bounds.push(TypeParamBound::Trait(
                    syn::parse_str("serde::de::DeserializeOwned").unwrap(),
                ));
                GenericParam::Type(typ.clone())
            }
            _ => param.clone(),
        };
        params.push(param);
    }

    params
}

/// Implements [`Display`](std::fmt::Display) as a wrapper around
/// [`serde_json::to_string`].
///
/// See this [crate's](crate) documentation for more examples on
/// how you can use this custom derive procedural macro.
///
/// **Example:**
///
/// ```rust
/// use serde::Serialize;
/// use display_json::DisplayAsJson;
///
/// #[derive(Serialize, DisplayAsJson)]
/// #[serde(tag = "type")]
/// #[serde(content = "val")]
/// #[serde(rename_all = "lowercase")]
/// enum EitherStringOrNum {
///     String(String),
///     Num(f64),
/// }
///
/// let num = EitherStringOrNum::Num(12.);
/// assert_eq!(num.to_string(), r#"{"type":"num","val":12.0}"#);
///
/// let string = EitherStringOrNum::String("hello".to_owned());
/// assert_eq!(
///     string.to_string(),
///     r#"{"type":"string","val":"hello"}"#,
/// );
/// ```
///
#[proc_macro_derive(DisplayAsJson)]
pub fn derive_display_as_json(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let generics = &input.generics;
    let params = serialize_in_generics(generics);

    let result = derive_fmt_as_json!(
        std::fmt::Display,
        name,
        generics,
        params,
        serde_json::to_string
    );

    TokenStream::from(result)
}

/// Implements [`Display`](std::fmt::Display) as a wrapper around
/// [`serde_json::to_string_pretty`].
///
/// See this [crate's](crate) documentation for more examples on
/// how you can use this custom derive procedural macro.
///
/// **Example:**
///
/// ```rust
/// use serde::Serialize;
/// use display_json::DisplayAsJsonPretty;
///
/// #[derive(Serialize, DisplayAsJsonPretty)]
/// #[serde(tag = "type")]
/// #[serde(content = "val")]
/// #[serde(rename_all = "lowercase")]
/// enum EitherStringOrNum {
///     String(String),
///     Num(f64),
/// }
///
/// let res = r#"{
///   "type": "num",
///   "val": 12.0
/// }"#;
///
/// let num = EitherStringOrNum::Num(12.);
/// assert_eq!(num.to_string(), res);
///
/// let res = r#"{
///   "type": "string",
///   "val": "hello"
/// }"#;
///
/// let string = EitherStringOrNum::String("hello".to_owned());
/// assert_eq!(string.to_string(), res);
/// ```
///
#[proc_macro_derive(DisplayAsJsonPretty)]
pub fn derive_display_as_json_pretty(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let generics = &input.generics;
    let params = serialize_in_generics(generics);

    let result = derive_fmt_as_json!(
        std::fmt::Display,
        name,
        generics,
        params,
        serde_json::to_string_pretty
    );

    TokenStream::from(result)
}

/// Implements [`Debug`](std::fmt::Debug) as a wrapper around
/// [`serde_json::to_string`].
///
/// See this [crate's](crate) documentation for more examples on
/// how you can use this custom derive procedural macro.
///
/// **Example:**
///
/// ```rust
/// use serde::Serialize;
/// use display_json::DebugAsJson;
///
/// #[derive(Serialize, DebugAsJson)]
/// #[serde(tag = "type")]
/// #[serde(content = "val")]
/// #[serde(rename_all = "lowercase")]
/// enum EitherStringOrNum {
///     String(String),
///     Num(f64),
/// }
///
/// let num = EitherStringOrNum::Num(12.);
/// assert_eq!(format!("{:?}", num), r#"{"type":"num","val":12.0}"#);
///
/// let string = EitherStringOrNum::String("hello".to_owned());
/// assert_eq!(
///     format!("{:?}", string),
///     r#"{"type":"string","val":"hello"}"#,
/// );
/// ```
///
#[proc_macro_derive(DebugAsJson)]
pub fn derive_debug_as_json(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let generics = &input.generics;
    let params = serialize_in_generics(generics);

    let result = derive_fmt_as_json!(
        std::fmt::Debug,
        name,
        generics,
        params,
        serde_json::to_string
    );

    TokenStream::from(result)
}

/// Implements [`Debug`](std::fmt::Debug) as a wrapper around
/// [`serde_json::to_string_pretty`].
///
/// See this [crate's](crate) documentation for more examples on
/// how you can use this custom derive procedural macro.
///
/// **Example:**
///
/// ```rust
/// use serde::Serialize;
/// use display_json::DebugAsJsonPretty;
///
/// #[derive(Serialize, DebugAsJsonPretty)]
/// #[serde(tag = "type")]
/// #[serde(content = "val")]
/// #[serde(rename_all = "lowercase")]
/// enum EitherStringOrNum {
///     String(String),
///     Num(f64),
/// }
///
/// let res = r#"{
///   "type": "num",
///   "val": 12.0
/// }"#;
///
/// let num = EitherStringOrNum::Num(12.);
/// assert_eq!(format!("{:?}", num), res);
///
/// let res = r#"{
///   "type": "string",
///   "val": "hello"
/// }"#;
///
/// let string = EitherStringOrNum::String("hello".to_owned());
/// assert_eq!(format!("{:?}", string), res);
/// ```
///
#[proc_macro_derive(DebugAsJsonPretty)]
pub fn derive_debug_as_json_pretty(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let generics = &input.generics;
    let params = serialize_in_generics(generics);

    let result = derive_fmt_as_json!(
        std::fmt::Debug,
        name,
        generics,
        params,
        serde_json::to_string_pretty
    );

    TokenStream::from(result)
}

/// Implements [`FromStr`](std::str::FromStr) as a wrapper around
/// [`serde_json::from_str`].
///
/// See this [crate's](crate) documentation for more examples on
/// how you can use this custom derive procedural macro.
///
/// **Example:**
///
/// ```rust
/// use serde::Deserialize;
/// use display_json::FromStrAsJson;
///
/// use std::str::FromStr;
///
/// #[derive(Deserialize, FromStrAsJson, PartialEq, Debug)]
/// #[serde(tag = "type")]
/// #[serde(content = "val")]
/// #[serde(rename_all = "lowercase")]
/// enum EitherStringOrNum {
///     String(String),
///     Num(f64),
/// }
///
/// let s = r#"{"type":"num","val":12.0}"#;
/// let num = EitherStringOrNum::Num(12.);
/// assert_eq!(EitherStringOrNum::from_str(s).unwrap(), num);
///
/// let s = r#"{"type":"string","val":"hello"}"#;
/// let string = EitherStringOrNum::String("hello".to_owned());
/// assert_eq!(EitherStringOrNum::from_str(s).unwrap(), string);
/// ```
///
#[proc_macro_derive(FromStrAsJson)]
pub fn derive_from_str_as_json(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let generics = &input.generics;
    let params = deserialize_in_generics(generics);

    let result = quote! {
        impl<#params> std::str::FromStr for #name #generics {
            type Err = serde_json::Error;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
              serde_json::from_str(s)
            }
        }
    };

    TokenStream::from(result)
}
