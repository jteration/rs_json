use std::env;
use std::error::Error;
use rs_json::run;
use rs_json::JsonValue;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    let json_value: Result<Option<JsonValue>, Box<dyn Error>> = run(&args);

    println!("{:?}", json_value);
}
