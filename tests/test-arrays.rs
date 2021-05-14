#[cfg(test)]
mod tests {
  use rs_json::run;
  use rs_json::JsonValue;

  #[test]
  fn test_good_array_1() {
    let path = "./tests/good_json/arrays/test-1.json".to_string();
    let good_array_one: JsonValue = run(&path).unwrap();

    match good_array_one {
      JsonValue::JArray(good_array_one_val) => assert_eq!(0, good_array_one_val.len()),
      _ => assert!(false),
    }
  }

  #[test]
  fn test_good_array_2() {
    let path = "./tests/good_json/arrays/test-2.json".to_string();
    let good_array_two: JsonValue = run(&path).unwrap();

    match good_array_two {
      JsonValue::JArray(good_array_two_val) => {
        assert_eq!(1, good_array_two_val.len());
        let array = &good_array_two_val[0];

        match array {
          JsonValue::JArray(array_val) => assert_eq!(0, array_val.len()),
          _ => assert!(false),
        }
      }
      _ => assert!(false),
    }
  }

  #[test]
  fn test_good_array_3() {
    let path = "./tests/good_json/arrays/test-3.json".to_string();
    let good_array_three: JsonValue = run(&path).unwrap();

    match good_array_three {
      JsonValue::JArray(good_array_three_val) => {
        assert_eq!(2, good_array_three_val.len());
        let num1 = &good_array_three_val[0];
        let num2 = &good_array_three_val[1];

        match num1 {
          JsonValue::JNum(num1_val) => {
            assert!(1234.1f64 > *num1_val);
            assert!(1233.9f64 < *num1_val);
          },
          _ => assert!(false),
        }

        match num2 {
          JsonValue::JNum(num2_val) => {
            assert!(4321.1f64 > *num2_val);
            assert!(4320.9f64 < *num2_val);
          },
          _ => assert!(false),
        }
      }
      _ => assert!(false),
    }
  }

  #[test]
  fn test_good_array_4() {
    let path = "./tests/good_json/arrays/test-4.json".to_string();
    let good_array_four: JsonValue = run(&path).unwrap();

    match good_array_four {
      JsonValue::JArray(good_array_four_val) => {
        assert_eq!(5, good_array_four_val.len());
        let num = &good_array_four_val[0];
        let null = &good_array_four_val[1];
        let bool_one = &good_array_four_val[2];
        let bool_two = &good_array_four_val[3];
        let string = &good_array_four_val[4];

        match num {
          JsonValue::JNum(num_val) => {
            assert!(1231000000000000.0f64 > *num_val);
            assert!(1229000000000000.0f64 < *num_val);
          },
          _ => assert!(false),
        }

        match null {
          JsonValue::JNull => assert!(true),
          _ => assert!(false),
        }

        match bool_one {
          JsonValue::JBool(bool_one_val) => assert_eq!(false, *bool_one_val),
          _ => assert!(false),
        }

        match bool_two {
          JsonValue::JBool(bool_two_val) => assert_eq!(true, *bool_two_val),
          _ => assert!(false),
        }

        match string {
          JsonValue::JString(string_val) => assert_eq!("✨✨✨".to_string(), *string_val),
          _ => assert!(false),
        }
      }
      _ => assert!(false),
    }
  }

  #[test]
  fn test_good_array_5() {
    let path = "./tests/good_json/arrays/test-5.json".to_string();
    let good_array_five: JsonValue = run(&path).unwrap();

    match good_array_five {
      JsonValue::JArray(good_array_five_val) => {
        assert_eq!(1, good_array_five_val.len());
        let string = &good_array_five_val[0];

        match string {
          JsonValue::JString(string_val) => assert_eq!(*string_val, "Whitespace test".to_string()),
          _ => assert!(false),
        }
      }
      _ => assert!(false),
    }
  }

  #[test]
  fn test_good_array_6() {
    let path = "./tests/good_json/arrays/test-6.json".to_string();
    let good_array_six: JsonValue = run(&path).unwrap();

    match good_array_six {
      JsonValue::JArray(good_array_six_val) => {
        assert_eq!(2, good_array_six_val.len());
        let array_one = &good_array_six_val[0];
        let number = &good_array_six_val[1];

        match array_one {
          JsonValue::JArray(array_one_val) => {
            assert_eq!(1, array_one_val.len());
            let array_two = &array_one_val[0];

            match array_two {
              JsonValue::JArray(array_two_val) => {
                assert_eq!(2, array_two_val.len());
                let array_three = &array_two_val[0];
                let array_four = &array_two_val[1];

                match array_three {
                  JsonValue::JArray(array_three_val) => assert_eq!(0, array_three_val.len()),
                  _ => assert!(false),
                }

                match array_four {
                  JsonValue::JArray(array_four_val) => assert_eq!(0, array_four_val.len()),
                  _ => assert!(false),
                }
              }
              _ => assert!(false),
            };
          }
          _ => assert!(false),
        }

        match number {
          JsonValue::JNum(number_val) => {
            assert!(123500000000000000000000000.0f64 > *number_val);
            assert!(123300000000000000000000000.0f64 < *number_val);
          },
          _ => assert!(false),
        }
      }
      _ => assert!(false),
    }
  }
}
