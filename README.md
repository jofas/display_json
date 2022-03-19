# display_json

[![Build Status](https://github.com/jofas/display_json/actions/workflows/build.yml/badge.svg)](https://github.com/jofas/display_json/actions/workflows/build.yml)
[![Codecov](https://codecov.io/gh/jofas/display_json/branch/master/graph/badge.svg?token=69YKZ1JIBK)](https://codecov.io/gh/jofas/display_json)
[![Latest Version](https://img.shields.io/crates/v/display_json.svg)](https://crates.io/crates/display_json)
[![Downloads](https://img.shields.io/crates/d/display_json?label=downloads)](https://crates.io/crates/display_json)
[![Docs](https://img.shields.io/badge/docs-latest-blue.svg)](https://docs.rs/display_json/latest/display_json)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)

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
