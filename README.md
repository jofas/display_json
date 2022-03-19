# display_json

[![Build Status](https://github.com/jofas/display_json/actions/workflows/build.yml/badge.svg)](https://github.com/jofas/display_json/actions/workflows/build.yml)
[![Codecov](https://codecov.io/gh/jofas/display_json/branch/master/graph/badge.svg?token=69YKZ1JIBK)](https://codecov.io/gh/jofas/display_json)
[![Latest Version](https://img.shields.io/crates/v/display_json.svg)](https://crates.io/crates/display_json)
[![Downloads](https://img.shields.io/crates/d/display_json?label=downloads)](https://crates.io/crates/display_json)
[![Docs](https://img.shields.io/badge/docs-latest-blue.svg)](https://docs.rs/display_json/latest/display_json)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)

A rust crate for automatically deriving the rust standard library's
`Debug` and `Display` traits for types that implement 
[serde's](https://serde.rs) `Serialize` trait.
The implementations of these traits serialize objects to stringified 
json. 

`display_json` provides the custom derive procedural macros 
`DisplayAsJson`, `DisplayAsJsonPretty`, `DebugAsJson` and
`DebugAsJsonPretty`.
These custom derives create a stringified json version of an object
using [serde_json](https://github.com/serde-rs/json).
The four custom derives are basically neat wrappers that wrap the 
`serde_json::to_string` and `serde_json::to_string_pretty` 
functions into an implementation of `Display` or `Debug`.


## Usage

`display_json` provides you with an easy way of combining the 
serialization of your objects to json and the usage of rust's 
built-in formatting features provided by the `Display` and `Debug` 
traits.


### DisplayAsJson

Without `display_json`, you'd have to serialize your object to a json
string like this:

```rust
use serde::Serialize;
use serde_json::{Result, to_string};

#[derive(Serialize)]
struct Foo {
  bar: String,
  baz: i32,
  bat: bool,
}

fn main() -> Result<()> {
  let f = Foo { bar: "bar".to_owned(), baz: 0, bat: true };

  let s = to_string(&f)?;

  assert_eq!(s, r#"{"bar":"bar","baz":0,"bat":true}"#);

  Ok(())
} 
```

The same can be accomplished with less code by using the 
`DisplayAsJson` custom derive on `Foo`:

```rust
use serde::Serialize;
use display_json::DisplayAsJson;

#[derive(Serialize, DisplayAsJson)]
struct Foo {
  bar: String,
  baz: i32,
  bat: bool,
}

let f = Foo { bar: "bar".to_owned(), baz: 0, bat: true };

assert_eq!(f.to_string(), r#"{"bar":"bar","baz":0,"bat":true}"#);
```

`DisplayAsJson` is a wrapper around `serde_json::to_string`. 
It takes the serialized json string and provides it to the `Display`
trait which is implemented by `DisplayAsJson`.
This makes it more convenient to format your objects to json.
For example, you can use this to create well-ingestable log messages
or to serialize your data for sending it as the body of an http
request:

```rust
use serde::Serialize;
use display_json::DisplayAsJson;

#[derive(Serialize, DisplayAsJson)]
struct Foo {
  bar: String,
  baz: i32,
  bat: bool,
}

let f = Foo { bar: "bar".to_owned(), baz: 0, bat: true };

// log `f` to stdout:
println!("{}", f);

// or you could construct an http request body from it

// or process your serialized object any other way you please
```


### DebugAsJson

`DebugAsJson` works the same as `DisplayAsJson`, only instead of 
implementing the `Display` trait, it implements the `Debug` trait:

```rust
use serde::Serialize;
use display_json::DebugAsJson;

#[derive(Serialize, DebugAsJson)]
struct Foo {
  bar: String,
  baz: i32,
  bat: bool,
}

let f = Foo { bar: "bar".to_owned(), baz: 0, bat: true };

// note that we use the debug formatter for serializing `f` to json
let f_ser = format!("{:?}", f);

assert_eq!(f_ser, r#"{"bar":"bar","baz":0,"bat":true}"#);
```


### Pretty json 

`DisplayAsJsonPretty` and `DebugAsJsonPretty` work the same as their
non-pretty counterparts, except producing a multiline, indented json
string instead of a compact json string:

```rust
use serde::Serialize;
use display_json::{DisplayAsJsonPretty, DebugAsJsonPretty};

#[derive(Serialize, DisplayAsJsonPretty, DebugAsJsonPretty)]
struct Foo {
  bar: String,
  baz: i32,
  bat: bool,
}

let f = Foo { bar: "bar".to_owned(), baz: 0, bat: true };

let result = r#"{
  "bar": "bar",
  "baz": 0,
  "bat": true
}"#;

let f_ser = format!("{}", f);
let f_ser_dbg = format!("{:?}", f);

assert_eq!(f_ser, f_ser_dbg);

assert_eq!(f_ser, result);
assert_eq!(f_ser_dbg, result);
```


### Mixing Display and Debug

As you can see in the example above, you can combine the 
`DisplayAsJson` and `DebugAsJson` variants however you like.
For example, you could use `DisplayAsJson` to serialize your object
for the body of an http request and `DebugAsJsonPretty` for creating
well-readable debugging messages for you to debug your code.
