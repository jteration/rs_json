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

fn increment_position(json_chars: &Vec<char>, position: &mut usize, num_incremented: usize) -> Result<(), Box<dyn Error>> {
    // Check if new position past the end of the json string
    *position += num_incremented;

    if *position >= json_chars.len() {
        return Err("Reached end of JSON unexpectedly".into());
    }

    Ok(())
}

fn skip_white_space(json_chars: &Vec<char>, position: &mut usize) -> Result<(), Box<dyn Error>> {
    while json_chars[*position] == ' ' || json_chars[*position] == '\n' || json_chars[*position] == '\r' || json_chars[*position] == '\t' {
        increment_position(json_chars, position, 1)?;
    }

    Ok(())
}

fn get_json_object(json_chars: &Vec<char>, position: &mut usize) -> Result<HashMap<String, Option<JsonValue>>, Box<dyn Error>> {
    // Char will be an open curly bracket
    increment_position(json_chars, position, 1)?;

    let mut json_obj: HashMap<String, Option<JsonValue>> = HashMap::new();
    let mut get_key: bool = true;
    let mut get_val: bool = false;
    let mut key = "".to_string();
    let mut done: bool = false;

    while !done {
        skip_white_space(json_chars, position)?;

        if get_key {
            if json_chars[*position] != '"' {
                return Err(format!("Invalid char at position {}", position).into());
            }

            key = get_json_string(json_chars, position)?;

            skip_white_space(json_chars, position)?;

            if json_chars[*position] == ':' {
                increment_position(json_chars, position, 1)?;

                get_key = false;
                get_val = true;
            } else {
                return Err(format!("Invalid char at position {}", position).into());
            }
        } else if get_val {
            // JsonValue::new calls skip_white_space at the start
            let val: Option<JsonValue> = JsonValue::new(&json_chars, position)?;
            json_obj.insert(key.clone(), val);
            get_val = false;
            key = "".to_string();
        } else {
            if json_chars[*position] == ',' {
                get_key = true;
                increment_position(json_chars, position, 1)?;
            } else if json_chars[*position] == '}' {
                done = true;
            }
        }
    }

    // Put position past closing curly bracket
    increment_position(json_chars, position, 1)?;

    return Ok(json_obj);
}

fn get_json_array(json_chars: &Vec<char>, position: &mut usize) -> Result<Vec<Option<JsonValue>>, Box<dyn Error>> {
    // Char will be an open square bracket
    increment_position(json_chars, position, 1)?;

    let mut json_arr: Vec<Option<JsonValue>> = vec![];
    let mut done: bool = false;

    while !done {
        // JsonValue::new() calls skip_white_space
        json_arr.push(JsonValue::new(&json_chars, position)?);

        skip_white_space(json_chars, position)?;

        if json_chars[*position] == ',' {
            increment_position(json_chars, position, 1)?;
        } else if json_chars[*position] == ']' {
            done = true;
        } else {
            return Err(format!("Invalid char at position {}", position).into());
        }
    }

    // Put position past closing square bracket
    increment_position(json_chars, position, 1)?;

    return Ok(json_arr);
}

fn get_json_string(json_chars: &Vec<char>, position: &mut usize) -> Result<String, Box<dyn Error>> {
    // Char will be an open double quotation
    increment_position(json_chars, position, 1)?;

    let mut token = json_chars[*position];
    let mut new_string: Vec<char> = vec![];
    let mut done: bool = false;

    while !done {
        if token != '"' {
            new_string.push(token);
            increment_position(json_chars, position, 1)?;

            token = json_chars[*position];
        } else {
            done = true;
        }
    }

    // Put position past closed double quotation
    increment_position(json_chars, position, 1)?;

    return Ok(new_string.iter().collect::<String>());
}

fn get_json_num(json_chars: &Vec<char>, position: &mut usize) -> Result<f64, Box<dyn Error>> {
    // Char will be a digit or '-'
    let mut token = json_chars[*position];
    let mut num: Vec<char> = vec![];
    let mut reached_dec: bool = false;
    let mut has_exponent: bool = false;
    let mut exponent_negative: bool = false;
    let mut exponent: Vec<char> = vec![];

    // Check if negative
    if token == '-' {
        num.push(token);
        increment_position(json_chars, position, 1)?;
        token = json_chars[*position];
    }

    // Check for leading '0'
    if token == '0' {
        if
            !(json_chars[*position + 1] == '.' ||
            json_chars[*position + 1] == ' ' ||
            json_chars[*position + 1] == ',' ||
            json_chars[*position + 1] == '\n' ||
            json_chars[*position + 1] == '\r' ||
            json_chars[*position + 1] == '\t')
        {
            return Err(format!("Invalid char at position {}", position).into());
        }
    }

    let mut done: bool = false;

    while !done {
        if token.is_digit(10) || token == '.' || token == 'e' || token == 'E' {
            if token == '.' && !reached_dec && !has_exponent {
                // '.' char is only valid in the initial number
                num.push(token);
                reached_dec = true;
            } else if (token == '.' && reached_dec) || (token == '.' && has_exponent) {
                // Valid numbers only contain one '.' and they cannot be in the exponent
                return Err(format!("Invalid char at position {}", position).into());
            } else if (token == 'e' || token == 'E') && !has_exponent {
                has_exponent = true;

                // Check for '-' or '+' after exponent
                if json_chars[*position + 1] == '-' {
                    exponent_negative = true;
                    increment_position(json_chars, position, 1)?;
                } else if json_chars[*position + 1] == '+' {
                    increment_position(json_chars, position, 1)?;
                } else if !json_chars[*position + 1].is_digit(10) {
                    // If next char isn't '-' or '+' ensure its a digit
                    return Err(format!("Invalid char at position {}", position).into());
                }
            } else if (token == 'e' || token == 'E') && has_exponent {
                // Numbers can only have one exponent char
                return Err(format!("Invalid char at position {}", position).into());
            } else if token.is_digit(10) && has_exponent {
                exponent.push(token);
            } else if token.is_digit(10) {
                num.push(token);
            } else {
                return Err(format!("Invalid char at position {}", position).into());
            }

            increment_position(json_chars, position, 1)?;
            token = json_chars[*position];
        } else if token == ' ' || token == '\n' || token == '\r' || token == '\t' || token == ',' {
            done = true;
        } else {
            return Err(format!("Invalid char at position {}", position).into());
        }
    }

    return Ok(
        if has_exponent {
            if exponent_negative {
                (num.iter().collect::<String>().parse::<f64>().unwrap()) / (10.0f64.powf(exponent.iter().collect::<String>().parse::<f64>().unwrap()))
            } else {
                (num.iter().collect::<String>().parse::<f64>().unwrap()) * (10.0f64.powf(exponent.iter().collect::<String>().parse::<f64>().unwrap()))
            }
        } else {
            num.iter().collect::<String>().parse::<f64>().unwrap()
        }
    );
}

fn get_json_bool(json_chars: &Vec<char>, position: &mut usize, t_or_f: bool) -> Result<bool, Box<dyn Error>> {
    // Char will be 'f' or 't'
    increment_position(json_chars, position, 1)?;

    if t_or_f == true {
        // Check if characters are 't' 'r' 'u' 'e'
        if
            *position + 2 < json_chars.len() &&
            json_chars[*position] == 'r' &&
            json_chars[*position + 1] == 'u' &&
            json_chars[*position + 2] == 'e'
        {
            // Increment past 'e'
            increment_position(json_chars, position, 3)?;

            return Ok(true);
        } else {
            return Err(format!("Invalid char at position {}", position).into());
        }
    } else {
        // Check if characters are 'f' 'a' 'l' 's' 'e'
        if
            *position + 3 < json_chars.len() &&
            json_chars[*position] == 'a' &&
            json_chars[*position + 1] == 'l' &&
            json_chars[*position + 2] == 's' &&
            json_chars[*position + 3] == 'e'
        {
            // Increment past 'e'
            increment_position(json_chars, position, 4)?;

            return Ok(false);
        } else {
            return Err(format!("Invalid char at position {}", position).into());
        }
    }
}

fn check_null(json_chars: &Vec<char>, position: &mut usize) -> Result<bool, Box<dyn Error>> {
    // Char will be 'n'
    increment_position(json_chars, position, 1)?;

    // Check if characters are 'n' 'u' 'l' 'l'
    if
        *position + 2 < json_chars.len() &&
        json_chars[*position] == 'u' &&
        json_chars[*position + 1] == 'l' &&
        json_chars[*position + 2] == 'l'
    {
        increment_position(json_chars, position, 3)?;
        return Ok(true);
    } else {
        return Err(format!("Invalid char at position {}", position).into());
    }
}

impl JsonValue {
    fn new(json_chars: &Vec<char>, position: &mut usize) -> Result<Option<JsonValue>, Box<dyn Error>> {
        use crate::JsonValue::*;

        skip_white_space(json_chars, position)?;

        let token: char = json_chars[*position];
        let value: Option<JsonValue>;

        if token == '"' {
            value = Some(JsonString(get_json_string(json_chars, position)?));
        } else if token == 'f' {
            value = Some(JsonBool(get_json_bool(json_chars, position, false)?));
        } else if token == 't' {
            value = Some(JsonBool(get_json_bool(json_chars, position, true)?));
        } else if token.is_digit(10) || token == '-' {
            value = Some(JsonNum(get_json_num(json_chars, position)?));
        } else if token == 'n' {
            check_null(json_chars, position)?;
            value = None;
        } else if token == '{' {
            value = Some(JsonObj(get_json_object(json_chars, position)?))
        } else if token == '[' {
            value = Some(JsonArray(get_json_array(json_chars, position)?))
        } else {
            return Err(format!("Invalid char at position {}", position).into());
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

    // Ensure any characters after end of root value are just whitespace character
    while position < json_length - 1 {
        if
            characters[position] == ' ' ||
            characters[position] == '\n' ||
            characters[position] == '\r' ||
            characters[position] == '\t'
        {
            position += 1;
        } else {
            return Err(format!("Invalid char at position {}", position).into());
        }
    }

    Ok(parsed_json_string)
}
