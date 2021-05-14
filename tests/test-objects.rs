#[cfg(test)]
mod tests {
  use rs_json::run;
  use rs_json::JsonValue;

  #[test]
  fn test_good_object_1() {
    let path = "./tests/good_json/objects/test-1.json".to_string();
    let good_object_one: JsonValue = run(&path).unwrap();

    match good_object_one {
      JsonValue::JObj(good_object_one_val) => assert_eq!(good_object_one_val.is_empty(), true),
      _ => assert!(false),
    }
  }

  #[test]
  fn test_good_object_2() {
    let path = "./tests/good_json/objects/test-2.json".to_string();
    let good_object_two: JsonValue = run(&path).unwrap();

    match good_object_two {
      JsonValue::JObj(good_object_two_val) => {
        let obj = good_object_two_val.get("test").unwrap();

        match obj {
          JsonValue::JObj(obj_val) => assert_eq!(obj_val.is_empty(), true),
          _ => assert!(false),
        }
      }
      _ => assert!(false),
    }
  }

  #[test]
  fn test_good_object_3() {
    let path = "./tests/good_json/objects/test-3.json".to_string();
    let good_object_three: JsonValue = run(&path).unwrap();

    match good_object_three {
      JsonValue::JObj(good_object_three_val) => {
        let string = good_object_three_val.get("key1").unwrap();
        let num = good_object_three_val.get("key2").unwrap();
        let bool = good_object_three_val.get("key3").unwrap();
        let null = good_object_three_val.get("key4").unwrap();

        match string {
          JsonValue::JString(string_val) => assert_eq!(*string_val, "test".to_string()),
          _ => assert!(false),
        }

        match num {
          JsonValue::JNum(num_val) => {
            assert!(*num_val > 122.9f64);
            assert!(*num_val < 123.1f64);
          },
          _ => assert!(false),
        }

        match bool {
          JsonValue::JBool(bool_val) => assert_eq!(*bool_val, false),
          _ => assert!(false),
        }

        match null {
          JsonValue::JNull => assert!(true),
          _ => assert!(false),
        }
      }
      _ => assert!(false),
    }
  }

  #[test]
  fn test_good_object_4() {
    let path = "./tests/good_json/objects/test-4.json".to_string();
    let good_object_four: JsonValue = run(&path).unwrap();

    match good_object_four {
      JsonValue::JObj(val) => {
        let num_one = val.get("num1").unwrap();
        let num_two = val.get("num2").unwrap();
        let num_three = val.get("num3").unwrap();
        let num_four = val.get("num4").unwrap();
        let num_five = val.get("num5").unwrap();
        let num_six = val.get("num6").unwrap();
        let num_seven = val.get("num7").unwrap();
        let num_eight = val.get("num8").unwrap();
        let num_nine = val.get("num9").unwrap();
        let num_ten = val.get("num10").unwrap();
        let num_eleven = val.get("num11").unwrap();
        let num_twelve = val.get("num12").unwrap();
        let num_thirteen = val.get("num13").unwrap();

        match num_one {
          JsonValue::JNum(num_one_val) => {
            assert!(*num_one_val > 11.9f64);
            assert!(*num_one_val < 12.164);
          },
          _ => assert!(false),
        }

        match num_two {
          JsonValue::JNum(num_two_val) => {
            assert!(*num_two_val > 12.0f64);
            assert!(*num_two_val < 12.2f64);
          },
          _ => assert!(false),
        }

        match num_three {
          JsonValue::JNum(num_three_val) => {
            assert!(*num_three_val > -12.1f64);
            assert!(*num_three_val < -11.9f64);
          },
          _ => assert!(false),
        }

        match num_four {
          JsonValue::JNum(num_four_val) => {
            assert!(*num_four_val > -12.2f64);
            assert!(*num_four_val < -12.0f64);
          },
          _ => assert!(false),
        }

        match num_five {
          JsonValue::JNum(num_five_val) => {
            assert!(*num_five_val > 1199.9f64);
            assert!(*num_five_val < 1200.1f64);
          },
          _ => assert!(false),
        }

        match num_six {
          JsonValue::JNum(num_six_val) => {
            assert!(*num_six_val > 1199.9f64);
            assert!(*num_six_val < 1200.1f64);
          },
          _ => assert!(false),
        }

        match num_seven {
          JsonValue::JNum(num_seven_val) => {
            assert!(*num_seven_val > 0.11f64);
            assert!(*num_seven_val < 0.13f64);
          },
          _ => assert!(false),
        }

        match num_eight {
          JsonValue::JNum(num_eight_val) => {
            assert!(*num_eight_val > -1200.1f64);
            assert!(*num_eight_val < -1199.9f64);
          },
          _ => assert!(false),
        }

        match num_nine {
          JsonValue::JNum(num_nine_val) => {
            assert!(*num_nine_val > -0.13f64);
            assert!(*num_nine_val < -0.11f64);
          },
          _ => assert!(false),
        }

        match num_ten {
          JsonValue::JNum(num_ten_val) => {
            assert!(*num_ten_val > 0.02f64);
            assert!(*num_ten_val < 0.04f64);
          },
          _ => assert!(false),
        }

        match num_eleven {
          JsonValue::JNum(num_eleven_val) => {
            assert!(*num_eleven_val > -0.04f64);
            assert!(*num_eleven_val < -0.02f64);
          },
          _ => assert!(false),
        }

        match num_twelve {
          JsonValue::JNum(num_twelve_val) => {
            assert!(*num_twelve_val > -30.1f64);
            assert!(*num_twelve_val < -29.9f64);
          },
          _ => assert!(false),
        }

        match num_thirteen {
          JsonValue::JNum(num_thirteen_val) => {
            assert!(*num_thirteen_val > -0.000031f64);
            assert!(*num_thirteen_val < -0.000029f64);
          },
          _ => assert!(false),
        }
      }
      _ => assert!(false),
    }
  }
}
