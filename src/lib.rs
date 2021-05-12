use std::collections::HashMap;
use std::error::Error;
use std::str;
use std::fs;

#[derive(Debug)]
pub enum JsonValue {
    JsonObj(HashMap<String, Option<JsonValue>>),
    JsonArray(Vec<Option<JsonValue>>),
    JsonString(String),
    JsonNum(f64),
    JsonBool(bool),
}

fn increment_position(json_chars: &Vec<char>, position: &mut usize, num_incremented: usize) -> Result<(), Box<dyn Error>> {
    // Check if new position is past the end of the json string
    if *position + num_incremented > json_chars.len() {
        return Err("Reached end of JSON unexpectedly".into());
    }

    *position += num_incremented;

    Ok(())
}

fn get_char_at_offset(json_chars: &Vec<char>, position: &mut usize, offset: usize) -> Result<char, Box<dyn Error>> {
    // Check if new position is past the end of the json string
    if *position + offset > json_chars.len() - 1 {
        return Err("Reached end of JSON unexpectedly".into());
    }

    Ok(json_chars[*position + offset])
}

fn is_white_space(token: char) -> bool {
    token == ' ' || token == '\n' || token == '\r' || token == '\t'
}

fn skip_white_space(json_chars: &Vec<char>, position: &mut usize) -> Result<(), Box<dyn Error>> {
    while is_white_space(json_chars[*position]) {
        increment_position(json_chars, position, 1)?;
    }

    Ok(())
}

fn get_json_object(json_chars: &Vec<char>, position: &mut usize) -> Result<HashMap<String, Option<JsonValue>>, Box<dyn Error>> {
    // Char will be an open curly bracket
    increment_position(json_chars, position, 1)?;
    let mut json_obj: HashMap<String, Option<JsonValue>> = HashMap::new();
    let mut get_val: bool = false;
    let mut key = "".to_string();
    let mut done: bool = false;

    while !done {
        skip_white_space(json_chars, position)?;

        let token = json_chars[*position];

        match token {
            '"' => {
                if !get_val {
                    key = get_json_string(json_chars, position)?;
                    get_val = true;
                } else {
                    // Val is string
                    let val: Option<JsonValue> = JsonValue::new(&json_chars, position)?;
                    json_obj.insert(key.clone(), val);
                    get_val = false;
                    key = "".to_string();
                }
            },
            ':' => {
                if !get_val {
                    return Err(format!("Invalid char at position {}", position).into());
                } else {
                    increment_position(json_chars, position, 1)?;
                }
            },
            ',' => increment_position(json_chars, position, 1)?,
            '}' => done = true,
            _ => {
                // JsonValue::new will check if its a valid value starter
                let val: Option<JsonValue> = JsonValue::new(&json_chars, position)?;
                json_obj.insert(key.clone(), val);
                get_val = false;
                key = "".to_string();
            }
        }
    }

    // Put position past closing curly bracket
    increment_position(json_chars, position, 1)?;

    Ok(json_obj)
}

fn get_json_array(json_chars: &Vec<char>, position: &mut usize) -> Result<Vec<Option<JsonValue>>, Box<dyn Error>> {
    // Char will be an open square bracket
    increment_position(json_chars, position, 1)?;

    let mut json_arr: Vec<Option<JsonValue>> = vec![];
    let mut done: bool = false;

    while !done {
        skip_white_space(json_chars, position)?;

        let token = json_chars[*position];

        match token {
            ',' => increment_position(json_chars, position, 1)?,
            ']' => done = true,
            _ => json_arr.push(JsonValue::new(&json_chars, position)?)
        }
    }

    // Put position past closing square bracket
    increment_position(json_chars, position, 1)?;

    Ok(json_arr)
}

fn get_json_string(json_chars: &Vec<char>, position: &mut usize) -> Result<String, Box<dyn Error>> {
    // Char will be an open double quotation
    increment_position(json_chars, position, 1)?;

    let mut token = json_chars[*position];
    let mut new_string: Vec<u16> = vec![];
    let mut done: bool = false;

    while !done {
        if token == '\\' {
            let escaped_char = get_char_at_offset(json_chars, position, 1)?;

            match escaped_char {
                'b' => new_string.push(0008 as u16),
                'f' => new_string.push(0012 as u16),
                'n' => new_string.push(0010 as u16),
                'r' => new_string.push(0013 as u16),
                't' => new_string.push(0009 as u16),
                '"' => new_string.push(0034 as u16),
                '\\' => new_string.push(0092 as u16),
                'u' => {
                    let first_digit = get_char_at_offset(json_chars, position, 2)? as u8;
                    let second_digit = get_char_at_offset(json_chars, position, 3)? as u8;
                    let third_digit = get_char_at_offset(json_chars, position, 4)? as u8;
                    let fourth_digit = get_char_at_offset(json_chars, position, 5)? as u8;

                    let bytes = [first_digit, second_digit, third_digit, fourth_digit];

                    let from_hex: &str = str::from_utf8(&bytes)?;
                    let as_u16 = u16::from_str_radix(&from_hex, 16)?;
                    new_string.push(as_u16);

                    increment_position(json_chars, position, 4)?;
                }
                _ => return Err(format!("Invalid char at position {}", position).into())
            }

            increment_position(json_chars, position, 2)?;

            token = json_chars[*position];
        } else if token != '"' {
            new_string.push(token as u16);
            increment_position(json_chars, position, 1)?;

            token = json_chars[*position];
        } else {
            done = true;
        }
    }

    // Put position past closed double quotation
    increment_position(json_chars, position, 1)?;

    Ok(String::from_utf16(&new_string)?)
}

fn get_json_num(json_chars: &Vec<char>, position: &mut usize) -> Result<f64, Box<dyn Error>> {
    // Char will be a digit or '-'
    let mut token = json_chars[*position];
    let mut num: Vec<char> = vec![];
    let mut has_decimal: bool = false;
    let mut has_exponent: bool = false;
    let mut negative_exponent: bool = false;
    let mut exponent: Vec<char> = vec![];

    // Check if negative
    if token == '-' {
        num.push(token);
        increment_position(json_chars, position, 1)?;
        token = json_chars[*position];
    }

    // Check for leading '0'
    if token == '0' {
        let next_char = get_char_at_offset(json_chars, position, 1)?;

        if !(next_char == '.' || next_char == ',' || is_white_space(next_char)) {
            return Err(format!("Invalid char at position {}", position).into());
        }
    }

    let mut done: bool = false;

    while !done {
        match token {
            '.' => {
                if !has_decimal && !has_exponent {
                    // '.' char is only valid in the initial number
                    num.push(token);
                    has_decimal = true;
                } else if has_decimal || has_exponent {
                    // Can only have one '.' and it may not appear in the exponent
                    return Err(format!("Invalid char at position {}", position).into());
                }
            }
            'e' | 'E' => {
                if !has_exponent {
                    has_exponent = true;
                    let next_char = get_char_at_offset(json_chars, position, 1)?;

                    // Check for '-' or '+' after exponent
                    if next_char == '-' {
                        negative_exponent = true;
                        increment_position(json_chars, position, 1)?;
                    } else if next_char == '+' {
                        increment_position(json_chars, position, 1)?;
                    } else if !next_char.is_digit(10) {
                        // If next char isn't '-' or '+' ensure its a digit
                        return Err(format!("Invalid char at position {}", position).into());
                    }
                } else {
                    // Can only have one exponent
                    return Err(format!("Invalid char at position {}", position).into());
                }
            }
            '0'..='9' => {
                if has_exponent {
                    exponent.push(token);
                } else {
                    num.push(token);
                }
            }
            tok if is_white_space(tok) || tok == ',' || token == '}' => {
                done = true;
            },
            _ => return Err(format!("Invalid char at position {}", position).into())
        }

        if !done {
            increment_position(json_chars, position, 1)?;
            token = json_chars[*position];
        }
    }

    let parsed_num: f64 = num.iter().collect::<String>().parse::<f64>()?;

    Ok(
        if has_exponent {
            let parsed_exponent: f64 = exponent.iter().collect::<String>().parse::<f64>()?;

            if negative_exponent {
                parsed_num / (10.0f64.powf(parsed_exponent))
            } else {
                parsed_num * (10.0f64.powf(parsed_exponent))
            }
        } else {
            parsed_num
        }
    )
}

fn get_json_bool(json_chars: &Vec<char>, position: &mut usize, t_or_f: bool) -> Result<bool, Box<dyn Error>> {
    // Char will be 'f' or 't'
    increment_position(json_chars, position, 1)?;

    if t_or_f == true {
        // Check if characters are 't' 'r' 'u' 'e'
        let true_test: Vec<char> = vec!['r', 'u', 'e'];

        for i in 0..true_test.len() {
            if json_chars[*position] != true_test[i] {
                return Err(format!("Invalid char at position {}", position).into());
            }

            increment_position(json_chars, position, 1)?;
        }

        return Ok(true);
    } else {
        // Check if characters are 'f' 'a' 'l' 's' 'e'
        let false_test: Vec<char> = vec!['a', 'l', 's', 'e'];

        for i in 0..false_test.len() {
            if json_chars[*position] != false_test[i] {
                return Err(format!("Invalid char at position {}", position).into());
            }

            increment_position(json_chars, position, 1)?;
        }

        return Ok(false);
    }
}

fn check_null(json_chars: &Vec<char>, position: &mut usize) -> Result<(), Box<dyn Error>> {
    // Char will be 'n'
    increment_position(json_chars, position, 1)?;

    let null_test: Vec<char> = vec!['u', 'l', 'l'];

    for i in 0..null_test.len() {
        if json_chars[*position] != null_test[i] {
            return Err(format!("Invalid char at position {}", position).into());
        }

        increment_position(json_chars, position, 1)?;
    }

    Ok(())
}

impl JsonValue {
    fn new(json_chars: &Vec<char>, position: &mut usize) -> Result<Option<JsonValue>, Box<dyn Error>> {
        use crate::JsonValue::*;

        skip_white_space(json_chars, position)?;

        let token: char = json_chars[*position];
        let value: Option<JsonValue> = match token {
            '"' => Some(JsonString(get_json_string(json_chars, position)?)),
            'f' => Some(JsonBool(get_json_bool(json_chars, position, false)?)),
            't' => Some(JsonBool(get_json_bool(json_chars, position, true)?)),
            '-' | '0'..='9' => Some(JsonNum(get_json_num(json_chars, position)?)),
            'n' => {
                check_null(json_chars, position)?;
                None
            },
            '{' => Some(JsonObj(get_json_object(json_chars, position)?)),
            '[' => Some(JsonArray(get_json_array(json_chars, position)?)),
            _ => return Err(format!("Invalid char at position {}", position).into())
        };

        Ok(value)
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
    let root_value: Option<JsonValue> = JsonValue::new(&characters, &mut position)?;

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
