#[cfg(test)]
mod tests {
  use rs_json::run;
  use rs_json::JsonValue;

  #[test]
  fn test_good_number_1() {
    let empt = "".to_string();
    let args1: Vec<String> = vec![empt, "./tests/good_json/numbers/test-1.json".to_string()];
    let good_number_one: JsonValue = run(&args1).unwrap();

    match good_number_one {
      JsonValue::JNum(val) => assert_eq!(12345.0f64, val),
      _ => assert!(false),
    }
  }

  #[test]
  fn test_good_number_2() {
    let empt = "".to_string();
    let args1: Vec<String> = vec![empt, "./tests/good_json/numbers/test-2.json".to_string()];
    let good_number_two: JsonValue = run(&args1).unwrap();

    match good_number_two {
      JsonValue::JNum(val) => assert_eq!(-12345.0f64, val),
      _ => assert!(false),
    }
  }

  #[test]
  fn test_good_number_3() {
    let empt = "".to_string();
    let args1: Vec<String> = vec![empt, "./tests/good_json/numbers/test-3.json".to_string()];
    let good_number_three: JsonValue = run(&args1).unwrap();

    match good_number_three {
      JsonValue::JNum(val) => assert_eq!(12345.123f64, val),
      _ => assert!(false),
    }
  }

  #[test]
  fn test_good_number_4() {
    let empt = "".to_string();
    let args1: Vec<String> = vec![empt, "./tests/good_json/numbers/test-4.json".to_string()];
    let good_number_four: JsonValue = run(&args1).unwrap();

    match good_number_four {
      JsonValue::JNum(val) => assert_eq!(-12345.123f64, val),
      _ => assert!(false),
    }
  }

  #[test]
  fn test_good_number_5() {
    let empt = "".to_string();
    let args1: Vec<String> = vec![empt, "./tests/good_json/numbers/test-5.json".to_string()];
    let good_number_five: JsonValue = run(&args1).unwrap();

    match good_number_five {
      JsonValue::JNum(val) => assert_eq!(123400f64, val),
      _ => assert!(false),
    }
  }

  #[test]
  fn test_good_number_6() {
    let empt = "".to_string();
    let args1: Vec<String> = vec![empt, "./tests/good_json/numbers/test-6.json".to_string()];
    let good_number_six: JsonValue = run(&args1).unwrap();

    match good_number_six {
      JsonValue::JNum(val) => assert_eq!(-123400.0f64, val),
      _ => assert!(false),
    }
  }

  #[test]
  fn test_good_number_7() {
    let empt = "".to_string();
    let args1: Vec<String> = vec![empt, "./tests/good_json/numbers/test-7.json".to_string()];
    let good_number_seven: JsonValue = run(&args1).unwrap();

    match good_number_seven {
      JsonValue::JNum(val) => assert_eq!(123412.3f64, val),
      _ => assert!(false),
    }
  }

  #[test]
  fn test_good_number_8() {
    let empt = "".to_string();
    let args1: Vec<String> = vec![empt, "./tests/good_json/numbers/test-8.json".to_string()];
    let good_number_eight: JsonValue = run(&args1).unwrap();

    match good_number_eight {
      JsonValue::JNum(val) => assert_eq!(-123412.3f64, val),
      _ => assert!(false),
    }
  }

  #[test]
  fn test_good_number_9() {
    let empt = "".to_string();
    let args1: Vec<String> = vec![empt, "./tests/good_json/numbers/test-9.json".to_string()];
    let good_number_nine: JsonValue = run(&args1).unwrap();

    match good_number_nine {
      JsonValue::JNum(val) => assert_eq!(12.341230000000001f64, val),
      _ => assert!(false),
    }
  }

  #[test]
  fn test_good_number_10() {
    let empt = "".to_string();
    let args1: Vec<String> = vec![empt, "./tests/good_json/numbers/test-10.json".to_string()];
    let good_number_ten: JsonValue = run(&args1).unwrap();

    match good_number_ten {
      JsonValue::JNum(val) => assert_eq!(-12.341230000000001f64, val),
      _ => assert!(false),
    }
  }
}
