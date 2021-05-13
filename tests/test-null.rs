#[cfg(test)]
mod tests {
  use rs_json::run;
  use rs_json::JsonValue;

  #[test]
  fn test_good_null() {
    let path = "./tests/good_json/null/test-1.json".to_string();
    let good_null: JsonValue = run(&path).unwrap();

    match good_null {
      JsonValue::JNull => assert!(true),
      _ => assert!(false),
    }
  }
}
