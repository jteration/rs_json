#[cfg(test)]
mod tests {
  use rs_json::run;
  use rs_json::JsonError;

  #[test]
  fn test_bad_1() {
    let path = "./tests/bad_json/test-1.json".to_string();

    match run(&path) {
      Ok(_) => assert!(false),
      Err(e) => assert_eq!(e.to_string(), JsonError::IllegalChar(3).to_string()),
    }
  }

  #[test]
  fn test_bad_2() {
    let path = "./tests/bad_json/test-2.json".to_string();

    match run(&path) {
      Ok(_) => assert!(false),
      Err(e) => assert_eq!(e.to_string(), JsonError::IllegalChar(1).to_string()),
    }
  }

  #[test]
  fn test_bad_3() {
    let path = "./tests/bad_json/test-3.json".to_string();

    match run(&path) {
      Ok(_) => assert!(false),
      Err(e) => assert_eq!(e.to_string(), JsonError::IllegalChar(1).to_string()),
    }
  }

  #[test]
  fn test_bad_4() {
    let path = "./tests/bad_json/test-4.json".to_string();

    match run(&path) {
      Ok(_) => assert!(false),
      Err(e) => assert_eq!(e.to_string(), "Reached end of JSON unexpectedly"),
    }
  }

  #[test]
  fn test_bad_5() {
    let path = "./tests/bad_json/test-5.json".to_string();

    match run(&path) {
      Ok(_) => assert!(false),
      Err(e) => assert_eq!(e.to_string(), JsonError::IllegalChar(4).to_string()),
    }
  }

  #[test]
  fn test_bad_6() {
    let path = "./tests/bad_json/test-6.json".to_string();

    match run(&path) {
      Ok(_) => assert!(false),
      Err(e) => assert_eq!(e.to_string(), JsonError::IllegalChar(1).to_string()),
    }
  }

  #[test]
  fn test_bad_7() {
    let path = "./tests/bad_json/test-7.json".to_string();

    match run(&path) {
      Ok(_) => assert!(false),
      Err(e) => assert_eq!(e.to_string(), JsonError::IllegalChar(4).to_string()),
    }
  }

  #[test]
  fn test_bad_8() {
    let path = "./tests/bad_json/test-8.json".to_string();

    match run(&path) {
      Ok(_) => assert!(false),
      Err(e) => assert_eq!(e.to_string(), JsonError::IllegalChar(2).to_string()),
    }
  }

  #[test]
  fn test_bad_9() {
    let path = "./tests/bad_json/test-9.json".to_string();

    match run(&path) {
      Ok(_) => assert!(false),
      Err(e) => assert_eq!(e.to_string(), JsonError::IllegalChar(1).to_string()),
    }
  }

  #[test]
  fn test_bad_10() {
    let path = "./tests/bad_json/test-10.json".to_string();

    match run(&path) {
      Ok(_) => assert!(false),
      Err(e) => assert_eq!(e.to_string(), "Reached end of JSON unexpectedly"),
    }
  }

  #[test]
  fn test_bad_11() {
    let path = "./tests/bad_json/test-11.json".to_string();

    match run(&path) {
      Ok(_) => assert!(false),
      Err(e) => assert_eq!(e.to_string(), JsonError::IllegalChar(0).to_string()),
    }
  }

  #[test]
  fn test_bad_12() {
    let path = "./tests/bad_json/test-12.json".to_string();

    match run(&path) {
      Ok(_) => assert!(false),
      Err(e) => assert_eq!(e.to_string(), JsonError::IllegalChar(4).to_string()),
    }
  }

  #[test]
  fn test_bad_13() {
    let path = "./tests/bad_json/test-13.json".to_string();

    match run(&path) {
      Ok(_) => assert!(false),
      Err(e) => assert_eq!(e.to_string(), JsonError::IllegalChar(6).to_string()),
    }
  }

  #[test]
  fn test_bad_14() {
    let path = "./tests/bad_json/test-14.json".to_string();

    match run(&path) {
      Ok(_) => assert!(false),
      Err(e) => assert_eq!(e.to_string(), JsonError::IllegalChar(22).to_string()),
    }
  }

  #[test]
  fn test_bad_15() {
    let path = "./tests/bad_json/test-15.json".to_string();

    match run(&path) {
      Ok(_) => assert!(false),
      Err(e) => assert_eq!(e.to_string(), JsonError::IllegalChar(1).to_string()),
    }
  }

  #[test]
  fn test_bad_16() {
    let path = "./tests/bad_json/test-16.json".to_string();

    match run(&path) {
      Ok(_) => assert!(false),
      Err(e) => assert_eq!(e.to_string(), JsonError::IllegalChar(1).to_string()),
    }
  }

  #[test]
  fn test_bad_17() {
    let path = "./tests/bad_json/test-17.json".to_string();

    match run(&path) {
      Ok(_) => assert!(false),
      Err(e) => assert_eq!(e.to_string(), JsonError::IllegalChar(5).to_string()),
    }
  }

  #[test]
  fn test_bad_18() {
    let path = "./tests/bad_json/test-18.json".to_string();

    match run(&path) {
      Ok(_) => assert!(false),
      Err(e) => assert_eq!(e.to_string(), JsonError::IllegalChar(1).to_string()),
    }
  }

  #[test]
  fn test_bad_19() {
    let path = "./tests/bad_json/test-19.json".to_string();

    match run(&path) {
      Ok(_) => assert!(false),
      Err(e) => assert_eq!(e.to_string(), JsonError::IllegalChar(1).to_string()),
    }
  }

  #[test]
  fn test_bad_20() {
    let path = "./tests/bad_json/test-20.json".to_string();

    match run(&path) {
      Ok(_) => assert!(false),
      Err(e) => assert_eq!(e.to_string(),JsonError::IllegalChar(16).to_string()),
    }
  }
}
