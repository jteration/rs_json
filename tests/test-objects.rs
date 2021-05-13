#[cfg(test)]
mod tests {
  use rs_json::run;
  use rs_json::JsonValue;

  #[test]
  fn test_good_object_1() {
    let empt = "".to_string();
    let args1: Vec<String> = vec![empt, "./tests/good_json/objects/test-1.json".to_string()];
    let good_object_one: JsonValue = run(&args1).unwrap();

    match good_object_one {
      JsonValue::JObj(val) => assert_eq!(val.is_empty(), true),
      _ => assert!(false)
    }
  }

  #[test]
  fn test_good_object_2() {
    let empt = "".to_string();
    let args1: Vec<String> = vec![empt, "./tests/good_json/objects/test-2.json".to_string()];
    let good_object_two: JsonValue = run(&args1).unwrap();

    match good_object_two {
      JsonValue::JObj(val) => {
				let inner_obj = val.get("test").unwrap();

				match inner_obj {
					JsonValue::JObj(inner_obj_val) => assert_eq!(inner_obj_val.is_empty(), true),
					_ => assert!(false)
				}
			},
      _ => assert!(false)
    }
  }

  #[test]
  fn test_good_object_3() {
    let empt = "".to_string();
    let args1: Vec<String> = vec![empt, "./tests/good_json/objects/test-3.json".to_string()];
    let good_object_three: JsonValue = run(&args1).unwrap();

    match good_object_three {
      JsonValue::JObj(val) => {
				let inner_val_one = val.get("key1").unwrap();
				let inner_val_two = val.get("key2").unwrap();
				let inner_val_three = val.get("key3").unwrap();
				let inner_val_four = val.get("key4").unwrap();

				match inner_val_one {
					JsonValue::JString(val_one) => assert_eq!(*val_one, "test".to_string()),
					_ => assert!(false)
				}

				match inner_val_two {
					JsonValue::JNum(val_two) => assert_eq!(*val_two, 123.0f64),
					_ => assert!(false)
				}

				match inner_val_three {
					JsonValue::JBool(val_three) => assert_eq!(*val_three, false),
					_ => assert!(false)
				}

				match inner_val_four {
					JsonValue::JNull => assert!(true),
					_ => assert!(false)
				}
			},
      _ => assert!(false)
    }
  }

  #[test]
  fn test_good_object_4() {
    let empt = "".to_string();
    let args1: Vec<String> = vec![empt, "./tests/good_json/objects/test-4.json".to_string()];
    let good_object_four: JsonValue = run(&args1).unwrap();

    match good_object_four {
      JsonValue::JObj(val) => {
				let inner_val_one = val.get("num1").unwrap();
				let inner_val_two = val.get("num2").unwrap();
				let inner_val_three = val.get("num3").unwrap();
				let inner_val_four = val.get("num4").unwrap();
				let inner_val_five = val.get("num5").unwrap();
				let inner_val_six = val.get("num6").unwrap();
				let inner_val_seven = val.get("num7").unwrap();
				let inner_val_eight = val.get("num8").unwrap();
				let inner_val_nine = val.get("num9").unwrap();
				let inner_val_ten = val.get("num10").unwrap();
				let inner_val_eleven = val.get("num11").unwrap();
				let inner_val_twelve = val.get("num12").unwrap();
				let inner_val_thirteen = val.get("num13").unwrap();
				
				match inner_val_one {
					JsonValue::JNum(val_one) => assert_eq!(*val_one, 12.0f64),
					_ => assert!(false)
				}
				
				match inner_val_two {
					JsonValue::JNum(val_two) => assert_eq!(*val_two, 12.1f64),
					_ => assert!(false)
				}
				
				match inner_val_three {
					JsonValue::JNum(val_three) => assert_eq!(*val_three, -12.0f64),
					_ => assert!(false)
				}
				
				match inner_val_four {
					JsonValue::JNum(val_four) => assert_eq!(*val_four, -12.1f64),
					_ => assert!(false)
				}
				
				match inner_val_five {
					JsonValue::JNum(val_five) => assert_eq!(*val_five, 1200.0f64),
					_ => assert!(false)
				}
				
				match inner_val_six {
					JsonValue::JNum(val_six) => assert_eq!(*val_six, 1200.0f64),
					_ => assert!(false)
				}
				
				match inner_val_seven {
					JsonValue::JNum(val_seven) => assert_eq!(*val_seven, 0.12f64),
					_ => assert!(false)
				}
				
				match inner_val_eight {
					JsonValue::JNum(val_eight) => assert_eq!(*val_eight, -1200.0f64),
					_ => assert!(false)
				}
				
				match inner_val_nine {
					JsonValue::JNum(val_nine) => assert_eq!(*val_nine, -0.12f64),
					_ => assert!(false)
				}
				
				match inner_val_ten {
					JsonValue::JNum(val_ten) => assert_eq!(*val_ten, 0.03f64),
					_ => assert!(false)
				}
				
				match inner_val_eleven {
					JsonValue::JNum(val_eleven) => assert_eq!(*val_eleven, -0.03f64),
					_ => assert!(false)
				}
				
				match inner_val_twelve {
					JsonValue::JNum(val_twelve) => assert_eq!(*val_twelve, -30.0f64),
					_ => assert!(false)
				}
				
				match inner_val_thirteen {
					JsonValue::JNum(val_thirteen) => assert_eq!(*val_thirteen, -0.000029999999999999997f64),
					_ => assert!(false)
				}
			},
      _ => assert!(false)
    }
  }
}
