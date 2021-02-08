extern crate proc_macro;
use proc_macro::TokenStream;

use quote::quote;
use syn::{parse_macro_input, DeriveInput};

macro_rules! derive_as_json {
    ($tr:path, $name:ident, $function:path) => {
        quote! {
          impl $tr for #$name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
              let s = $function(&self)
                .map_err(|_| std::fmt::Error::default())?;
              write!(f, "{}", s)
            }
          }
        }
    };
}

#[proc_macro_derive(DisplayAsJson)]
pub fn derive_display_as_json(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let result = derive_as_json!(std::fmt::Display, name, serde_json::to_string);
    TokenStream::from(result)
}

#[proc_macro_derive(DisplayAsJsonPretty)]
pub fn derive_display_as_json_pretty(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let result = derive_as_json!(std::fmt::Display, name, serde_json::to_string_pretty);
    TokenStream::from(result)
}

#[proc_macro_derive(DebugAsJson)]
pub fn derive_debug_as_json(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let result = derive_as_json!(std::fmt::Debug, name, serde_json::to_string);
    TokenStream::from(result)
}

#[proc_macro_derive(DebugAsJsonPretty)]
pub fn derive_debug_as_json_pretty(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let result = derive_as_json!(std::fmt::Debug, name, serde_json::to_string_pretty);
    TokenStream::from(result)
}
