use display_json::{
    DebugAsJson, DebugAsJsonPretty, DisplayAsJson, DisplayAsJsonPretty, FromStrAsJson,
};

use serde::{Deserialize, Serialize};

use std::str::FromStr;

#[derive(Serialize, DisplayAsJson, DebugAsJsonPretty, Default)]
struct Example {
    field1: bool,
    field2: String,
    field3: Option<String>,
}

#[derive(Serialize, DisplayAsJsonPretty, DebugAsJson, Default)]
struct ExamplePretty {
    field1: bool,
    field2: String,
    field3: Option<String>,
}

#[derive(Serialize, Deserialize, FromStrAsJson, DebugAsJson, PartialEq, Default)]
struct ExampleFromStr {
    field1: bool,
    field2: String,
    field3: Option<String>,
}

#[derive(Serialize, Deserialize, DebugAsJson, FromStrAsJson, PartialEq, Default)]
enum ExampleFromStrEnum {
    #[default]
    Foo,
    Bar,
}

#[derive(Serialize, Deserialize, DebugAsJson, FromStrAsJson, PartialEq)]
struct ExampleFromStrStringUnnamed(String);

#[derive(Serialize, Deserialize, DebugAsJson, FromStrAsJson, PartialEq)]
struct ExampleGenerics<T> {
    field1: T,
}

impl<T: Default> Default for ExampleGenerics<T> {
    fn default() -> Self {
        Self {
            field1: T::default(),
        }
    }
}

static JSON: &str = r#"{"field1":false,"field2":"","field3":null}"#;
static JSON_PRETTY: &str = r#"{
  "field1": false,
  "field2": "",
  "field3": null
}"#;

#[test]
fn example() {
    let display = format!("{}", Example::default());
    assert_eq!(display, JSON);
}

#[test]
fn example_debug() {
    let display = format!("{:?}", Example::default());
    assert_eq!(display, JSON_PRETTY);

    let display = format!("{:#?}", Example::default());
    assert_eq!(display, JSON_PRETTY);
}

#[test]
fn example_pretty() {
    let display = format!("{}", ExamplePretty::default());
    assert_eq!(display, JSON_PRETTY);
}

#[test]
fn example_pretty_debug() {
    let display = format!("{:?}", ExamplePretty::default());
    assert_eq!(display, JSON);
}

#[test]
fn example_from_str() {
    assert_eq!(
        ExampleFromStr::from_str(JSON).unwrap(),
        ExampleFromStr::default()
    );
    assert_eq!(
        ExampleFromStr::from_str(JSON_PRETTY).unwrap(),
        ExampleFromStr::default()
    );
}

#[test]
fn example_from_str_newtype_string_unnamed() {
    assert_eq!(
        ExampleFromStrStringUnnamed::from_str("Foo").unwrap(),
        ExampleFromStrStringUnnamed("Foo".into())
    );
}

#[test]
fn example_from_str_enum() {
    assert_eq!(
        ExampleFromStrEnum::from_str("Foo").unwrap(),
        ExampleFromStrEnum::default()
    );
    assert_eq!(
        ExampleFromStrEnum::from_str("Bar").unwrap(),
        ExampleFromStrEnum::Bar
    );
}

#[test]
fn example_generic_debug() {
    let debug = format!("{:?}", ExampleGenerics::<String>::default());
    assert_eq!(debug, r#"{"field1":""}"#);

    let debug = format!("{:?}", ExampleGenerics::<i64>::default());
    assert_eq!(debug, r#"{"field1":0}"#);
}

#[test]
fn example_generic_from_str() {
    let json = r#"{"field1":""}"#;

    assert_eq!(
        ExampleGenerics::<String>::from_str(json).unwrap(),
        ExampleGenerics::<String>::default(),
    );
}
