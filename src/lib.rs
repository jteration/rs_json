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

fn get_json_object(json_chars: &Vec<char>, position: &mut usize) -> Result<HashMap<String, Option<JsonValue>>, Box<dyn Error>> {
    // Char will be an open curly bracket
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

    // Put position past closing curly bracket
    *position += 1;

    return Ok(json_obj);
}

fn get_json_array(json_chars: &Vec<char>, position: &mut usize) -> Result<Vec<Option<JsonValue>>, Box<dyn Error>> {
    // Char will be an open square bracket
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

    // Put position past closing square bracket
    *position += 1;

    return Ok(json_arr);
}

fn get_json_string(json_chars: &Vec<char>, position: &mut usize) -> Result<String, Box<dyn Error>> {
    // Char will be an open double quotation
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

    // Put position past closed double quotation
    *position += 1;

    return Ok(new_string.iter().collect::<String>());
}

fn get_json_num(json_chars: &Vec<char>, position: &mut usize) -> Result<f64, Box<dyn Error>> {
    // Char will be a digit or '-'
    let mut token = json_chars[*position];
    let mut num: Vec<char> = vec![];
    let is_neg: bool = token == '-';
    let mut reached_dec: bool = false;
    let mut reached_exponent: bool = false;
    let mut exponent_negative: bool = false;
    let mut exponent: Vec<char> = vec![];

    if is_neg {
        num.push(token);
        *position += 1;
        token = json_chars[*position];
    }

    if token == '0' {
        if 
            !(json_chars[*position + 1] == '.' ||
            json_chars[*position + 1] == ' ' ||
            json_chars[*position + 1] == ',' ||
            json_chars[*position + 1] == '\n' ||
            json_chars[*position + 1] == '\r')
        {
            return Err("Invalid Json".into());
        }
    }

    let mut done: bool = false;

    while !done {
        if token.is_digit(10) || token == '.' || token == 'e' || token == 'E' || token == '-' {
            if token == '.' && !reached_dec && !reached_exponent {
                num.push(token);
                reached_dec = true;
            } else if token == '.' && reached_dec && !reached_exponent {
                return Err("Invalid Json".into());
            } else if token == '-' && reached_exponent {
                return Err("Invalid Json".into());
            } else if (token == 'e' || token == 'E') && !reached_exponent {
                reached_exponent = true;

                if json_chars[*position + 1] == '-' {
                    exponent_negative = true;
                    *position += 1;
                } else if json_chars[*position + 1] == '+' {
                    *position += 1;
                } else if !json_chars[*position + 1].is_digit(10) {
                    return Err("Invalid Json".into());
                }
            } else if token == '-' && reached_exponent {
                return Err("Invalid Json".into());
            } else if (token == 'e' || token == 'E') && reached_exponent {
                return Err("Invalid Json".into());
            } else if token.is_digit(10) && reached_exponent {
                exponent.push(token);
            } else {
                num.push(token);
            }
        } else if token == ' ' || token == '\n' || token == '\r' || token == ',' {
            done = true;
        } else {
            return Err("Invalid Json".into());
        }
        
        *position += 1;
        token = json_chars[*position];
    }

    return Ok(
        if reached_exponent {
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
    // Char will be 'n'
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
        use crate::JsonValue::*;
        skip_white_space(json_chars, position);

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
            return Err(format!("Invalid char at position {}: {}", position, token).into());
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
