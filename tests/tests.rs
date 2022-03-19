use display_json::{
  DebugAsJson, DebugAsJsonPretty, DisplayAsJson, DisplayAsJsonPretty,
};
use serde::Serialize;

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

#[derive(Serialize, DisplayAsJson)]
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

static JSON: &str =
  r#"{"field1":false,"field2":"","field3":null}"#;
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
fn example_generic() {
  let display = format!("{}", ExampleGenerics::<String>::default());
  assert_eq!(display, r#"{"field1":""}"#);

  let display = format!("{}", ExampleGenerics::<i64>::default());
  assert_eq!(display, r#"{"field1":0}"#);
}
