# display_json

A rust crate providing the custom derive procedural macros 
`DisplayAsJson`, `DisplayAsJsonPretty`, `DebugAsJson` and
`DebugAsJsonPretty`.
These custom derives can be used to automatically implement 
`std::fmt::Debug` and `std::fmt::Display` traits for a struct or 
enum that implements [serde's](https://serde.rs) `Serialize` trait.
