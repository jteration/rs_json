#[cfg(test)]
mod tests {
  use rs_json::run;
  use rs_json::JsonValue;

  #[test]
  fn test_good_boolean_1() {
    let path = "./tests/good_json/booleans/test-1.json".to_string();
    let good_boolean_one: JsonValue = run(&path).unwrap();

    match good_boolean_one {
      JsonValue::JBool(val) => assert_eq!(true, val),
      _ => assert!(false),
    }
  }

  #[test]
  fn test_good_boolean_2() {
    let path = "./tests/good_json/booleans/test-2.json".to_string();
    let good_boolean_two: JsonValue = run(&path).unwrap();

    match good_boolean_two {
      JsonValue::JBool(val) => assert_eq!(false, val),
      _ => assert!(false),
    }
  }
}
