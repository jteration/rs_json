#[cfg(test)]
mod tests {
  use rs_json::run;
  use rs_json::JsonValue;

  #[test]
  fn test_good_number_1() {
    let path = "./tests/good_json/numbers/test-1.json".to_string();
    let good_number_one: JsonValue = run(&path).unwrap();

    match good_number_one {
      JsonValue::JNum(good_number_one_val) => {
        assert!(good_number_one_val > 12344.0f64);
        assert!(good_number_one_val < 12346.0f64);
      },
      _ => assert!(false),
    }
  }

  #[test]
  fn test_good_number_2() {
    let path = "./tests/good_json/numbers/test-2.json".to_string();
    let good_number_two: JsonValue = run(&path).unwrap();

    match good_number_two {
      JsonValue::JNum(good_number_two_val) => {
        assert!(good_number_two_val > -12346.0f64);
        assert!(good_number_two_val < -12344.0f64);
      },
      _ => assert!(false),
    }
  }

  #[test]
  fn test_good_number_3() {
    let path = "./tests/good_json/numbers/test-3.json".to_string();
    let good_number_three: JsonValue = run(&path).unwrap();

    match good_number_three {
      JsonValue::JNum(good_number_three_val) => {
        assert!(good_number_three_val > 12345.122f64);
        assert!(good_number_three_val < 12345.124f64);
      },
      _ => assert!(false),
    }
  }

  #[test]
  fn test_good_number_4() {
    let path = "./tests/good_json/numbers/test-4.json".to_string();
    let good_number_four: JsonValue = run(&path).unwrap();

    match good_number_four {
      JsonValue::JNum(good_number_four_val) => {
        assert!(good_number_four_val > -12345.124);
        assert!(good_number_four_val < -12345.122);
      },
      _ => assert!(false),
    }
  }

  #[test]
  fn test_good_number_5() {
    let path = "./tests/good_json/numbers/test-5.json".to_string();
    let good_number_five: JsonValue = run(&path).unwrap();

    match good_number_five {
      JsonValue::JNum(good_number_five_val) => {
        assert!(good_number_five_val > 123399.9f64);
        assert!(good_number_five_val < 123401.0f64);
      },
      _ => assert!(false),
    }
  }

  #[test]
  fn test_good_number_6() {
    let path = "./tests/good_json/numbers/test-6.json".to_string();
    let good_number_six: JsonValue = run(&path).unwrap();

    match good_number_six {
      JsonValue::JNum(good_number_six_val) => {
        assert!(good_number_six_val > -123401.0f64);
        assert!(good_number_six_val < -123399.9f64);
      },
      _ => assert!(false),
    }
  }

  #[test]
  fn test_good_number_7() {
    let path = "./tests/good_json/numbers/test-7.json".to_string();
    let good_number_seven: JsonValue = run(&path).unwrap();

    match good_number_seven {
      JsonValue::JNum(good_number_seven_val) => {
        assert!(good_number_seven_val > 123412.2f64);
        assert!(good_number_seven_val < 123412.4f64);
      },
      _ => assert!(false),
    }
  }

  #[test]
  fn test_good_number_8() {
    let path = "./tests/good_json/numbers/test-8.json".to_string();
    let good_number_eight: JsonValue = run(&path).unwrap();

    match good_number_eight {
      JsonValue::JNum(good_number_eight_val) => {
        assert!(good_number_eight_val > -123412.4f64);
        assert!(good_number_eight_val < -123412.2f64);
      },
      _ => assert!(false),
    }
  }

  #[test]
  fn test_good_number_9() {
    let path = "./tests/good_json/numbers/test-9.json".to_string();
    let good_number_nine: JsonValue = run(&path).unwrap();

    match good_number_nine {
      JsonValue::JNum(good_number_nine_val) => {
        assert!(good_number_nine_val > 12.34122f64);
        assert!(good_number_nine_val < 12.34124f64);
      },
      _ => assert!(false),
    }
  }

  #[test]
  fn test_good_number_10() {
    let path = "./tests/good_json/numbers/test-10.json".to_string();
    let good_number_ten: JsonValue = run(&path).unwrap();

    match good_number_ten {
      JsonValue::JNum(good_number_ten_val) => {
        assert!(good_number_ten_val > -12.34124f64);
        assert!(good_number_ten_val < -12.34122f64);
      },
      _ => assert!(false),
    }
  }
}
