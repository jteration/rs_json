use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::fs;
use std::str;

use crate::JsonValue::*;

#[derive(Debug)]
pub enum JsonValue {
    JObject(HashMap<String, JsonValue>),
    JArray(Vec<JsonValue>),
    JString(String),
    JNum(f64),
    JBool(bool),
    JNull,
}

struct JsonArgs<'a> {
    chars: &'a Vec<char>,
    position: &'a mut usize,
    length: &'a usize,
}

#[derive(Debug)]
enum JsonError {
    IllegalChar(usize),
    UnexpectedEnd,
}

impl fmt::Display for JsonError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            JsonError::IllegalChar(position) => write!(f, "Invalid char at position {}", position),
            JsonError::UnexpectedEnd => write!(f, "Reached end of JSON unexpectedly"),
        }
    }
}

impl Error for JsonError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match *self {
            JsonError::IllegalChar(_) => None,
            JsonError::UnexpectedEnd => None,
        }
    }
}

fn increment_position(json_args: &mut JsonArgs, increment_by: usize) -> Result<(), Box<dyn Error>> {
    // Check if new position is past the end of the json string
    if *json_args.position + increment_by > *json_args.length {
        return Err(Box::new(JsonError::UnexpectedEnd));
    }

    *json_args.position += increment_by;

    Ok(())
}

fn get_char_at_offset(json_args: &mut JsonArgs, offset: usize) -> Result<char, Box<dyn Error>> {
    // Check if char is past the end of the json string
    if *json_args.position + offset > json_args.length - 1 {
        return Err(Box::new(JsonError::UnexpectedEnd));
    }

    Ok(json_args.chars[*json_args.position + offset])
}

fn is_white_space(token: char) -> bool {
    token == ' ' || token == '\n' || token == '\r' || token == '\t'
}

fn skip_white_space(json_args: &mut JsonArgs) -> Result<(), Box<dyn Error>> {
    while is_white_space(get_char_at_offset(json_args, 0)?) {
        increment_position(json_args, 1)?;
    }

    Ok(())
}

fn get_json_object(json_args: &mut JsonArgs) -> Result<HashMap<String, JsonValue>, Box<dyn Error>> {
    // Char will be an open curly bracket
    increment_position(json_args, 1)?;

    let mut new_json_obj: HashMap<String, JsonValue> = HashMap::new();
    let mut expecting_val: bool = false;
    let mut expecting_key_or_end: bool = true;
    let mut key = "".to_string();
    let mut done: bool = false;

    while !done {
        skip_white_space(json_args)?;

        let token = get_char_at_offset(json_args, 0)?;

        match token {
            '"' => {
                if expecting_key_or_end {
                    key = get_json_string(json_args)?;
                    expecting_val = true;
                    expecting_key_or_end = false;
                } else if expecting_val {
                    // Val is string
                    let val: JsonValue = JsonValue::new(json_args)?;
                    new_json_obj.insert(key.clone(), val);
                    expecting_val = false;
                    expecting_key_or_end = true;
                    key = "".to_string();
                } else {
                    return Err(Box::new(JsonError::IllegalChar(*json_args.position)));
                }
            }
            ':' => {
                if expecting_key_or_end || !expecting_val {
                    return Err(Box::new(JsonError::IllegalChar(*json_args.position)));
                } else {
                    increment_position(json_args, 1)?;
                }
            }
            ',' => {
                if expecting_val || !expecting_key_or_end {
                    return Err(Box::new(JsonError::IllegalChar(*json_args.position)));
                }

                expecting_val = true;
                increment_position(json_args, 1)?;
            }
            '}' => {
                if expecting_val || !expecting_key_or_end {
                    // Must not end if we're expecting another value
                    return Err(Box::new(JsonError::IllegalChar(*json_args.position)));
                }

                done = true;
                // Put position past closing curly bracket
                increment_position(json_args, 1)?;
            }
            _ => {
                if expecting_key_or_end || !expecting_val {
                    return Err(Box::new(JsonError::IllegalChar(*json_args.position)));
                }
                // JsonValue::new will check if its a valid value starter
                let val: JsonValue = JsonValue::new(json_args)?;
                new_json_obj.insert(key.clone(), val);
                expecting_val = false;
                expecting_key_or_end = true;
                key = "".to_string();
            }
        }
    }

    Ok(new_json_obj)
}

fn get_json_array(json_args: &mut JsonArgs) -> Result<Vec<JsonValue>, Box<dyn Error>> {
    // Char will be an open square bracket
    increment_position(json_args, 1)?;

    let mut new_json_arr: Vec<JsonValue> = vec![];
    let mut expecting_val: bool = false;
    let mut done: bool = false;

    while !done {
        skip_white_space(json_args)?;

        let token = get_char_at_offset(json_args, 0)?;

        match token {
            ',' => {
                expecting_val = true;
                increment_position(json_args, 1)?;
            }
            ']' => {
                if expecting_val {
                    // Must not end if we're expecting another value
                    return Err(Box::new(JsonError::IllegalChar(*json_args.position)));
                }

                done = true;
                // Put position past closing square bracket
                increment_position(json_args, 1)?;
            }
            _ => {
                new_json_arr.push(JsonValue::new(json_args)?);
                expecting_val = false;
            }
        }
    }

    Ok(new_json_arr)
}

fn get_json_string(json_args: &mut JsonArgs) -> Result<String, Box<dyn Error>> {
    // Char will be an open double quotation
    increment_position(json_args, 1)?;

    let mut new_string: Vec<u16> = vec![];
    let mut done: bool = false;

    while !done {
        let token = get_char_at_offset(json_args, 0)?;

        match token {
            '\\' => {
                let escaped_char = get_char_at_offset(json_args, 1)?;

                match escaped_char {
                    'b' => new_string.push(0008 as u16),
                    'f' => new_string.push(0012 as u16),
                    'n' => new_string.push(0010 as u16),
                    'r' => new_string.push(0013 as u16),
                    't' => new_string.push(0009 as u16),
                    '"' => new_string.push(0034 as u16),
                    '\\' => new_string.push(0092 as u16),
                    'u' => {
                        let first_byte = get_char_at_offset(json_args, 2)? as u8;
                        let second_byte = get_char_at_offset(json_args, 3)? as u8;
                        let third_byte = get_char_at_offset(json_args, 4)? as u8;
                        let fourth_byte = get_char_at_offset(json_args, 5)? as u8;

                        let bytes = [first_byte, second_byte, third_byte, fourth_byte];

                        let from_hex: &str = str::from_utf8(&bytes)?;
                        let as_u16 = u16::from_str_radix(&from_hex, 16)?;
                        new_string.push(as_u16);

                        increment_position(json_args, 4)?;
                    }
                    _ => return Err(format!("Invalid char at position {}", json_args.position).into()),
                }

                increment_position(json_args, 2)?;
            }
            '"' => {
                done = true;
                // Put position past closed double quotation
                increment_position(json_args, 1)?;
            }
            _ => {
                new_string.push(token as u16);
                increment_position(json_args, 1)?;
            }
        }
    }

    Ok(String::from_utf16(&new_string)?)
}

fn get_json_num(json_args: &mut JsonArgs) -> Result<f64, Box<dyn Error>> {
    // Char will be a digit or '-'
    let mut token = get_char_at_offset(json_args, 0)?;
    let mut new_num: Vec<char> = vec![];
    let mut has_decimal: bool = false;
    let mut has_exponent: bool = false;
    let mut negative_exponent: bool = false;
    let mut expecting_num: bool = true;
    let mut exponent: Vec<char> = vec![];

    // Check if negative
    if token == '-' {
        new_num.push(token);
        increment_position(json_args, 1)?;
        token = get_char_at_offset(json_args, 0)?;
    }

    // Check for leading '0'
    if token == '0' {
        let next_char = get_char_at_offset(json_args, 1)?;
        expecting_num = false;

        if next_char.is_digit(10) {
            return Err(Box::new(JsonError::IllegalChar(*json_args.position)));
        }
    }

    let mut done: bool = false;

    while !done {
        token = get_char_at_offset(json_args, 0)?;

        match token {
            '.' => {
                // '.' char is only valid in the initial number, and can only have one
                if expecting_num || has_decimal || has_exponent {
                    return Err(Box::new(JsonError::IllegalChar(*json_args.position)));
                }

                new_num.push(token);
                has_decimal = true;
                expecting_num = true;
            }
            'e' | 'E' => {
                // Only one 'e' or 'E' per number
                if expecting_num || has_exponent {
                    return Err(Box::new(JsonError::IllegalChar(*json_args.position)));
                }

                let next_char = get_char_at_offset(json_args, 1)?;

                // Check for '-' or '+' after exponent
                if next_char == '-' {
                    negative_exponent = true;
                    increment_position(json_args, 1)?;
                } else if next_char == '+' {
                    increment_position(json_args, 1)?;
                }

                has_exponent = true;
                expecting_num = true;
            }
            '0'..='9' => {
                if has_exponent {
                    exponent.push(token);
                } else {
                    new_num.push(token);
                }

                expecting_num = false;
            }
            tok if is_white_space(tok) || tok == ',' || token == '}' || token == ']' => {
                if expecting_num {
                    return Err(Box::new(JsonError::IllegalChar(*json_args.position)));
                }

                done = true;
            }
            _ => return Err(format!("Invalid char at position {}", json_args.position).into()),
        }

        if !done {
            if *json_args.position + 1 == json_args.chars.len() {
                done = true;
            }

            increment_position(json_args, 1)?;
        }
    }

    let parsed_num: f64 = new_num.iter().collect::<String>().parse::<f64>()?;

    Ok(if has_exponent {
        let parsed_exponent: f64 = exponent.iter().collect::<String>().parse::<f64>()?;

        if negative_exponent {
            parsed_num / (10.0f64.powf(parsed_exponent))
        } else {
            parsed_num * (10.0f64.powf(parsed_exponent))
        }
    } else {
        parsed_num
    })
}

fn get_json_bool(json_args: &mut JsonArgs, t_or_f: &char) -> Result<bool, Box<dyn Error>> {
    // Char will be 'f' or 't'
    increment_position(json_args, 1)?;

    if *t_or_f == 't' {
        // Check if characters are 't' 'r' 'u' 'e'
        let true_test: Vec<char> = vec!['r', 'u', 'e'];

        for i in 0..3 {
            if get_char_at_offset(json_args, 0)? != true_test[i] {
                return Err(Box::new(JsonError::IllegalChar(*json_args.position)));
            }

            increment_position(json_args, 1)?;
        }

        return Ok(true);
    } else {
        // Check if characters are 'f' 'a' 'l' 's' 'e'
        let false_test: Vec<char> = vec!['a', 'l', 's', 'e'];

        for i in 0..4 {
            if get_char_at_offset(json_args, 0)? != false_test[i] {
                return Err(Box::new(JsonError::IllegalChar(*json_args.position)));
            }

            increment_position(json_args, 1)?;
        }

        return Ok(false);
    }
}

fn check_null(json_args: &mut JsonArgs) -> Result<(), Box<dyn Error>> {
    // Char will be 'n'
    increment_position(json_args, 1)?;

    let null_test: Vec<char> = vec!['u', 'l', 'l'];

    for i in 0..3 {
        if get_char_at_offset(json_args, 0)? != null_test[i] {
            return Err(Box::new(JsonError::IllegalChar(*json_args.position)));
        }

        increment_position(json_args, 1)?;
    }

    Ok(())
}

impl JsonValue {
    fn new(json_args: &mut JsonArgs) -> Result<JsonValue, Box<dyn Error>> {
        skip_white_space(json_args)?;

        let token: char = get_char_at_offset(json_args, 0)?;

        let value: JsonValue = match token {
            '"' => JString(get_json_string(json_args)?),
            'f' | 't' => JBool(get_json_bool(json_args, &token)?),
            '-' | '0'..='9' => JNum(get_json_num(json_args)?),
            'n' => {
                check_null(json_args)?;
                JNull
            }
            '{' => JObject(get_json_object(json_args)?),
            '[' => JArray(get_json_array(json_args)?),
            _ => return Err(format!("Invalid char at position {}", json_args.position).into()),
        };

        Ok(value)
    }
}

fn parse_json(json_string: String) -> Result<JsonValue, Box<dyn Error>> {
    if json_string.len() == 0 {
        return Err("Zero length JSON string".into());
    }

    let characters: Vec<char> = json_string.chars().collect();
    let mut position: usize = 0;
    let json_length = characters.len();

    let mut json_args = JsonArgs {
        chars: &characters,
        position: &mut position,
        length: &json_length,
    };

    let root_value: JsonValue = JsonValue::new(&mut json_args)?;

    // Ensure any characters after end of root value are just whitespace characters
    while position < json_length {
        if is_white_space(characters[position]) {
            position += 1;
        } else {
            return Err(format!("Invalid char at position {}", position).into());
        }
    }

    Ok(root_value)
}

pub fn run(path: &String) -> Result<JsonValue, Box<dyn Error>> {
    let json_string = fs::read_to_string(path)?;
    let parsed_json = parse_json(json_string)?;

    Ok(parsed_json)
}
