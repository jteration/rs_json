#[cfg(test)]
mod tests {
  use rs_json::run;
  use rs_json::JsonValue;

  #[test]
  fn test_good_null() {
    let empt = "".to_string();
    let args1: Vec<String> = vec![empt, "./tests/good_json/null/test-1.json".to_string()];
    let good_null: JsonValue = run(&args1).unwrap();

    match good_null {
      JsonValue::JNull => assert!(true),
      _ => assert!(false),
    }
  }
}
