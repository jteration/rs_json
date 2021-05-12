#[cfg(test)]
mod tests {
  use rs_json::run;
  use rs_json::JsonValue;

  #[test]
  fn test_good_arrays() {
    let empt = "".to_string();
    let args1: Vec<String> = vec![empt, "./tests/good_json/arrays/test-1.json".to_string()];
    let good_array_one: JsonValue = run(&args1).unwrap();
    println!("good_array_one: {:?}", good_array_one);
  }
}
