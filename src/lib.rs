use std::collections::HashMap;
use std::error::Error;
use std::fs;

pub enum JsonValue {
    JsonObj,
    JsonArray,
    JsonString,
    JsonNum,
    JsonBool,
}

type JsonKey = String;
type JsonObj = HashMap<JsonKey, Option<JsonValue>>;
type JsonArray = Vec<Option<JsonValue>>;
type JsonString = String;
type JsonNum = f64;
type JsonBool = bool;

pub fn run(args: &[String]) -> Result<Option<JsonValue>, Box<dyn Error>> {
    let path_to_file = &args[1];
    let json_string = get_json_string(path_to_file)?;
    println!("{}", json_string);
    let parsed_json = parse_json(json_string)?;

    Ok(parsed_json)
}

fn get_json_string(path_to_file: &String) -> Result<String, std::io::Error> {
    return fs::read_to_string(path_to_file);
}

fn parse_json(json_string: String) -> Result<Option<JsonValue>, &'static str> {
    let mut parsed_json_string: Option<JsonValue> = None;

    if json_string.len() == 0 {
        return Err("Zero length JSON string");
    }

    Ok(parsed_json_string)
}
