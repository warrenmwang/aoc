use serde_json::Value;

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

fn part_1(input: &str) {
    // use a json lexer and then loop over all of the pieces and
    // sum up the numbers...OR...
    // go on the path of least resistance, just loop thru all the chars and find
    // the numbers and add them together
    let res = parse_nums(input);
    println!("Part 1: {}", res);
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

fn part_2(input: &str) {
    let v: Value = serde_json::from_str(&input).unwrap();
    println!("Part 2: {}", parse_json_chunk(v));
}

pub fn day_12(input: &str) {
    part_1(input);
    part_2(input);
}
