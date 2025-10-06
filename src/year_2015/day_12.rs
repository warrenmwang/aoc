use serde_json::Value;

use crate::{SolutionInput, terminal};

fn is_num(i: usize, chars: &Vec<char>) -> bool {
    let c = chars[i] as u8;

    // minus sign for negative number
    if c == 45 {
        // ensure its modifying a number
        if i + 1 < chars.len() {
            let c1 = chars[i + 1] as u8;
            return c1 >= 48 && c1 <= 57;
        }
        return false;
    }

    c >= 48 && c <= 57
}

fn parse_nums(input: &str) -> i32 {
    let mut res = 0;
    let chars: Vec<char> = String::from(input).chars().collect();
    let mut start_ind: i32 = -1;

    for i in 0..chars.clone().len() {
        // start of a number
        if start_ind == -1 && is_num(i, &chars) {
            start_ind = i as i32;
            continue;
        }

        // end of a number
        if start_ind != -1 && !is_num(i, &chars) {
            let slice = &input[start_ind as usize..i];
            start_ind = -1;
            let num: i32 = slice.parse().unwrap();
            res += num;
        }
    }

    res
}

fn part_1(input: &str) -> i32 {
    // use a json lexer and then loop over all of the pieces and
    // sum up the numbers...OR...
    // go on the path of least resistance, just loop thru all the chars and find
    // the numbers and add them together
    let res = parse_nums(input);
    res
}

fn parse_json_chunk(v: Value) -> i64 {
    let mut res: i64 = 0;

    match v {
        Value::Null => {}
        Value::Bool(_) => {}
        Value::String(_) => {}
        Value::Number(n) => {
            if n.is_i64() {
                res += n.as_i64().unwrap();
            }
        }
        Value::Array(a) => {
            for x in a {
                res += parse_json_chunk(x);
            }
        }
        Value::Object(o) => {
            let mut ignore_obj = false;
            // check if ignore this entire object
            for v in o.values() {
                match v {
                    Value::String(s) => {
                        if *s == "red".to_string() {
                            ignore_obj = true;
                        }
                    }
                    _ => {}
                }
            }

            if !ignore_obj {
                for v in o.values() {
                    res += parse_json_chunk(v.clone()); // TODO: why need clone here?
                }
            }
        }
    }

    res
}

fn part_2(input: &str) -> i64 {
    let v: Value = serde_json::from_str(&input).unwrap();
    parse_json_chunk(v)
}

pub fn day_12(input: SolutionInput) {
    if input.run_in_standalone {
        println!("2015.12 Part 1: {}", part_1(input.text_input));
        println!("2015.12 Part 2: {}", part_2(input.text_input));
    } else {
        let part_1_result = format!("2015.12 Part 1: {}", part_1(input.text_input));
        terminal::print_at_line_stdout(input.stdout_start_line, part_1_result);
        let part_2_result = format!("2015.12 Part 2: {}", part_2(input.text_input));
        terminal::print_at_line_stdout(input.stdout_start_line + 1, part_2_result);
    }
}
