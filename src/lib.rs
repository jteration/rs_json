use std::collections::HashMap;
use std::error::Error;
use std::fs;

#[derive(Debug)]
pub enum JsonValue {
    JsonObj(HashMap<String, Option<JsonValue>>),
    JsonArray(Vec<Option<JsonValue>>),
    JsonString(String),
    JsonNum(f64),
    JsonBool(bool),
}

use crate::JsonValue::*;

fn get_json_object(json_chars: &Vec<char>, position: &mut usize) -> Result<HashMap<String, Option<JsonValue>>, Box<dyn Error>> {
    *position += 1;
    let mut json_obj: HashMap<String, Option<JsonValue>> = HashMap::new();
    let mut get_key: bool = true;
    let mut get_val: bool = false;
    let mut key = "".to_string();

    while json_chars[*position] != '}' {
        if get_key && json_chars[*position] == '"' {
            key = get_json_string(&json_chars, position)?;

            while json_chars[*position] != ':' {
                *position += 1;
            }

            *position += 1;
            get_key = false;
            get_val = true;
        } else if get_val {
            let val: Option<JsonValue> = JsonValue::new(&json_chars, position)?;
            json_obj.insert(key.clone(), val);
            get_key = true;
            get_val = false;
            key = "".to_string();
        } else {
            *position += 1;
        }
    }

    *position += 1;

    return Ok(json_obj);
}

fn get_json_array(json_chars: &Vec<char>, position: &mut usize) -> Result<Vec<Option<JsonValue>>, Box<dyn Error>> {
    *position += 1;
    let mut json_arr: Vec<Option<JsonValue>> = vec![];

    while json_chars[*position] != ']' {
        json_arr.push(JsonValue::new(&json_chars, position)?);

        while json_chars[*position] != ',' && json_chars[*position] != ']' {
            *position += 1;
        }

        if json_chars[*position] == ',' {
            *position += 1;
        }
    }

    *position += 1;

    return Ok(json_arr);
}

fn get_json_string(json_chars: &Vec<char>, position: &mut usize) -> Result<String, Box<dyn Error>> {
    *position += 1;
    let mut token = json_chars[*position];
    let mut new_string: Vec<char> = vec![];

    while token != '"' {
        new_string.push(token);
        *position += 1;

        if *position > json_chars.len() - 1 {
            return Err("get_json_string: Invalid Json".into());
        }

        token = json_chars[*position];
    }

    *position += 1;

    return Ok(new_string.iter().collect::<String>());
}

fn get_json_num(json_chars: &Vec<char>, position: &mut usize) -> f64 {
    let mut token = json_chars[*position];
    let mut new_num: Vec<char> = vec![];

    while token.is_digit(10) || token == '.' || token == '-' {
        new_num.push(token);
        *position += 1;
        token = json_chars[*position];
    }

    return new_num.iter().collect::<String>().parse().unwrap();
}

fn get_json_bool(json_chars: &Vec<char>, position: &mut usize, t_or_f: bool) -> Result<bool, Box<dyn Error>> {
    *position += 1;
    let false_test = vec!['a', 'l', 's', 'e'];
    let true_test = vec!['r', 'u', 'e'];

    if t_or_f == true {
        if 
            json_chars[*position] == true_test[0] &&
            json_chars[*position + 1] == true_test[1] &&
            json_chars[*position + 2] == true_test[2]
        {
            *position += 3;
            return Ok(true);
        } else {
            return Err("Invalid character".into());
        }
    } else {
        if 
            json_chars[*position] == false_test[0] &&
            json_chars[*position + 1] == false_test[1] &&
            json_chars[*position + 2] == false_test[2] &&
            json_chars[*position + 3] == false_test[3]
        {
            *position += 4;
            return Ok(false);
        } else {
            return Err("Invalid character".into());
        }
    }
}

fn check_null(json_chars: &Vec<char>, position: &mut usize) -> Result<bool, Box<dyn Error>> {
    *position += 1;
    let null_test = vec!['u', 'l', 'l'];

    if 
        json_chars[*position] == null_test[0] &&
        json_chars[*position + 1] == null_test[1] &&
        json_chars[*position + 2] == null_test[2]
    {
        *position += 3;
        return Ok(true);
    } else {
        return Err("Invalid character".into());
    }
}

fn skip_white_space(json_chars: &Vec<char>, position: &mut usize) {
    while json_chars[*position] == ' ' || json_chars[*position] == '\n' || json_chars[*position] == '\r' {
        *position += 1;
    }
}

impl JsonValue {
    fn new(json_chars: &Vec<char>, position: &mut usize) -> Result<Option<JsonValue>, Box<dyn Error>> {
        skip_white_space(json_chars, position);

        let token: char = json_chars[*position];
        let value: Option<JsonValue>;

        if token == '"' {
            value = Some(JsonString(get_json_string(&json_chars, position)?));
        } else if token == 'f' {
            value = Some(JsonBool(get_json_bool(json_chars, position, false)?));
        } else if token == 't' {
            value = Some(JsonBool(get_json_bool(json_chars, position, true)?));
        } else if token.is_digit(10) || token == '-' {
            value = Some(JsonNum(get_json_num(json_chars, position)));
        } else if token == 'n' {
            check_null(json_chars, position)?;
            value = None;
        } else if token == '{' {
            value = Some(JsonObj(get_json_object(json_chars, position)?))
        } else if token == '[' {
            value = Some(JsonArray(get_json_array(json_chars, position)?))
        } else {
            return Err(format!("Invalid char: {}", token).into());
        }

        return Ok(value);
    }
}

pub fn run(args: &[String]) -> Result<Option<JsonValue>, Box<dyn Error>> {
    let path_to_file = &args[1];
    let json_string = fs::read_to_string(path_to_file)?;
    let parsed_json = parse_json(json_string)?;

    Ok(parsed_json)
}

fn parse_json(json_string: String) -> Result<Option<JsonValue>, Box<dyn Error>> {
    if json_string.len() == 0 {
        return Err("Zero length JSON string".into());
    }

    let characters: Vec<char> = json_string.chars().collect();
    let json_length = characters.len();
    let mut position: usize = 0;
    let parsed_json_string: Option<JsonValue> = JsonValue::new(&characters, &mut position)?;

    while position < json_length - 1 {
        if
            characters[position] == ' ' ||
            characters[position] == '\n' ||
            characters[position] == '\r'
        {   
            position += 1;
        } else {
            return Err("parse_json: Invalid Json".into());
        }
    }

    Ok(parsed_json_string)
}
