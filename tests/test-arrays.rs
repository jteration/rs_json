#[cfg(test)]
mod tests {
  use rs_json::run;
  use rs_json::JsonValue;

  #[test]
  fn test_good_array_1() {
    let empt = "".to_string();
    let args1: Vec<String> = vec![empt, "./tests/good_json/arrays/test-1.json".to_string()];
    let good_array_one: JsonValue = run(&args1).unwrap();

    match good_array_one {
      JsonValue::JArray(val) => assert_eq!(0, val.len()),
      _ => assert!(false),
    }
  }

  #[test]
  fn test_good_array_2() {
    let empt = "".to_string();
    let args1: Vec<String> = vec![empt, "./tests/good_json/arrays/test-2.json".to_string()];
    let good_array_two: JsonValue = run(&args1).unwrap();

    match good_array_two {
      JsonValue::JArray(val) => {
        assert_eq!(1, val.len());

        match &val[0] {
          JsonValue::JArray(inner_val) => assert_eq!(0, inner_val.len()),
          _ => assert!(false),
        }
      }
      _ => assert!(false),
    }
  }

  #[test]
  fn test_good_array_3() {
    let empt = "".to_string();
    let args1: Vec<String> = vec![empt, "./tests/good_json/arrays/test-3.json".to_string()];
    let good_array_three: JsonValue = run(&args1).unwrap();

    match good_array_three {
      JsonValue::JArray(val) => {
        assert_eq!(2, val.len());

        match &val[0] {
          JsonValue::JNum(inner_val) => assert_eq!(1234.0f64, *inner_val),
          _ => assert!(false),
        }

        match &val[1] {
          JsonValue::JNum(inner_val) => assert_eq!(4321.0f64, *inner_val),
          _ => assert!(false),
        }
      }
      _ => assert!(false),
    }
  }

  #[test]
  fn test_good_array_4() {
    let empt = "".to_string();
    let args1: Vec<String> = vec![empt, "./tests/good_json/arrays/test-4.json".to_string()];
    let good_array_four: JsonValue = run(&args1).unwrap();

    match good_array_four {
      JsonValue::JArray(val) => {
        assert_eq!(5, val.len());

        match &val[0] {
          JsonValue::JNum(inner_val) => assert_eq!(1230000000000000.0f64, *inner_val),
          _ => assert!(false),
        }

        match &val[1] {
          JsonValue::JNull => assert!(true),
          _ => assert!(false),
        }

        match &val[2] {
          JsonValue::JBool(inner_val) => assert_eq!(false, *inner_val),
          _ => assert!(false),
        }

        match &val[3] {
          JsonValue::JBool(inner_val) => assert_eq!(true, *inner_val),
          _ => assert!(false),
        }

        match &val[4] {
          JsonValue::JString(inner_val) => assert_eq!("✨✨✨".to_string(), *inner_val),
          _ => assert!(false),
        }
      }
      _ => assert!(false),
    }
  }

  #[test]
  fn test_good_array_5() {
    let empt = "".to_string();
    let args1: Vec<String> = vec![empt, "./tests/good_json/arrays/test-5.json".to_string()];
    let good_array_five: JsonValue = run(&args1).unwrap();

    match good_array_five {
      JsonValue::JArray(val) => {
        assert_eq!(1, val.len());

        match &val[0] {
          JsonValue::JString(inner_val) => assert_eq!(*inner_val, "Whitespace test".to_string()),
          _ => assert!(false),
        }
      }
      _ => assert!(false),
    }
  }

  #[test]
  fn test_good_array_6() {
    let empt = "".to_string();
    let args1: Vec<String> = vec![empt, "./tests/good_json/arrays/test-6.json".to_string()];
    let good_array_six: JsonValue = run(&args1).unwrap();

    match good_array_six {
      JsonValue::JArray(root_array) => {
        assert_eq!(2, root_array.len());

        let inner_array_one = &root_array[0];
        let number = &root_array[1];

        match inner_array_one {
          JsonValue::JArray(inner_array_one_val) => {
            assert_eq!(1, inner_array_one_val.len());

            let inner_array_two = &inner_array_one_val[0];

            match inner_array_two {
              JsonValue::JArray(inner_array_two_val) => {
                assert_eq!(2, inner_array_two_val.len());

                let inner_array_three = &inner_array_two_val[0];
                let inner_array_four = &inner_array_two_val[1];

                match inner_array_three {
                  JsonValue::JArray(inner_array_three_val) => assert_eq!(0, inner_array_three_val.len()),
                  _ => assert!(false),
                }

                match inner_array_four {
                  JsonValue::JArray(inner_array_four_val) => assert_eq!(0, inner_array_four_val.len()),
                  _ => assert!(false),
                }
              }
              _ => assert!(false),
            };
          }
          _ => assert!(false),
        }

        match number {
          JsonValue::JNum(number_val) => assert_eq!(123400000000000020000000000.0f64, *number_val),
          _ => assert!(false),
        }
      }
      _ => assert!(false),
    }
  }
}
