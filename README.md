# display_json

A rust crate providing the custom derive procedural macros 
`DisplayAsJson`, `DisplayAsJsonPretty`, `DebugAsJson` and
`DebugAsJsonPretty`.
These custom derives can be used to automatically implement the 
standard library's  `Debug` and `Display` traits for types
that already implement [serde's](https://serde.rs) `Serialize` trait.
The custom derives from the `display_json` crate create a `json` 
string for an object using 
[serde_json](https://github.com/serde-rs/json).
The `serde_json::to_string` and `serde_json::to_string_pretty` 
functions to be precise.
