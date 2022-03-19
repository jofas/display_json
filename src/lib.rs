#![doc = include_str!("../README.md")]

extern crate proc_macro;
use proc_macro::TokenStream;

use quote::quote;
use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::{
  parse_macro_input, DeriveInput, GenericParam, Generics,
  TypeParamBound,
};

macro_rules! derive_as_json {
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

fn serialize_in_generics(
  g: &Generics,
) -> Punctuated<GenericParam, Comma> {
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

#[proc_macro_derive(DisplayAsJson)]
pub fn derive_display_as_json(input: TokenStream) -> TokenStream {
  let input = parse_macro_input!(input as DeriveInput);
  let name = &input.ident;
  let generics = &input.generics;
  let params = serialize_in_generics(generics);

  let result = derive_as_json!(
    std::fmt::Display,
    name,
    generics,
    params,
    serde_json::to_string
  );

  TokenStream::from(result)
}

#[proc_macro_derive(DisplayAsJsonPretty)]
pub fn derive_display_as_json_pretty(
  input: TokenStream,
) -> TokenStream {
  let input = parse_macro_input!(input as DeriveInput);
  let name = &input.ident;
  let generics = &input.generics;
  let params = serialize_in_generics(generics);

  let result = derive_as_json!(
    std::fmt::Display,
    name,
    generics,
    params,
    serde_json::to_string_pretty
  );

  TokenStream::from(result)
}

#[proc_macro_derive(DebugAsJson)]
pub fn derive_debug_as_json(input: TokenStream) -> TokenStream {
  let input = parse_macro_input!(input as DeriveInput);
  let name = &input.ident;
  let generics = &input.generics;
  let params = serialize_in_generics(generics);

  let result = derive_as_json!(
    std::fmt::Debug,
    name,
    generics,
    params,
    serde_json::to_string
  );

  TokenStream::from(result)
}

#[proc_macro_derive(DebugAsJsonPretty)]
pub fn derive_debug_as_json_pretty(
  input: TokenStream,
) -> TokenStream {
  let input = parse_macro_input!(input as DeriveInput);
  let name = &input.ident;
  let generics = &input.generics;
  let params = serialize_in_generics(generics);

  let result = derive_as_json!(
    std::fmt::Debug,
    name,
    generics,
    params,
    serde_json::to_string_pretty
  );

  TokenStream::from(result)
}
