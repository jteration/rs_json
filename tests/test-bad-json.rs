#[cfg(test)]
mod tests {
  use rs_json::run;

  #[test]
  fn test_bad_1() {
    let path = "./tests/bad_json/test-1.json".to_string();

    match run(&path) {
      Ok(_) => assert!(false),
      Err(e) => assert_eq!(e.to_string(), "Invalid char at position 3"),
    }
  }

  #[test]
  fn test_bad_2() {
    let path = "./tests/bad_json/test-2.json".to_string();

    match run(&path) {
      Ok(_) => assert!(false),
      Err(e) => assert_eq!(e.to_string(), "Invalid char at position 1"),
    }
  }

  #[test]
  fn test_bad_3() {
    let path = "./tests/bad_json/test-3.json".to_string();

    match run(&path) {
      Ok(_) => assert!(false),
      Err(e) => assert_eq!(e.to_string(), "Invalid char at position 1"),
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
      Err(e) => assert_eq!(e.to_string(), "Invalid char at position 4"),
    }
  }

  #[test]
  fn test_bad_6() {
    let path = "./tests/bad_json/test-6.json".to_string();

    match run(&path) {
      Ok(_) => assert!(false),
      Err(e) => assert_eq!(e.to_string(), "Invalid char at position 1"),
    }
  }

  #[test]
  fn test_bad_7() {
    let path = "./tests/bad_json/test-7.json".to_string();

    match run(&path) {
      Ok(_) => assert!(false),
      Err(e) => assert_eq!(e.to_string(), "Invalid char at position 4"),
    }
  }

  #[test]
  fn test_bad_8() {
    let path = "./tests/bad_json/test-8.json".to_string();

    match run(&path) {
      Ok(_) => assert!(false),
      Err(e) => assert_eq!(e.to_string(), "Invalid char at position 2"),
    }
  }

  #[test]
  fn test_bad_9() {
    let path = "./tests/bad_json/test-9.json".to_string();

    match run(&path) {
      Ok(_) => assert!(false),
      Err(e) => assert_eq!(e.to_string(), "Invalid char at position 1"),
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
      Err(e) => assert_eq!(e.to_string(), "Invalid char at position 0"),
    }
  }

  #[test]
  fn test_bad_12() {
    let path = "./tests/bad_json/test-12.json".to_string();

    match run(&path) {
      Ok(_) => assert!(false),
      Err(e) => assert_eq!(e.to_string(), "Invalid char at position 4"),
    }
  }

  #[test]
  fn test_bad_13() {
    let path = "./tests/bad_json/test-13.json".to_string();

    match run(&path) {
      Ok(_) => assert!(false),
      Err(e) => assert_eq!(e.to_string(), "Invalid char at position 6"),
    }
  }

  #[test]
  fn test_bad_14() {
    let path = "./tests/bad_json/test-14.json".to_string();

    match run(&path) {
      Ok(_) => assert!(false),
      Err(e) => assert_eq!(e.to_string(), "Invalid char at position 22"),
    }
  }
}