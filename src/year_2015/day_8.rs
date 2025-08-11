// count num chars string given in code form would be in mem form
fn len_str_in_mem(s: &str) -> i32 {
    let s = String::from(s);
    assert!(s.len() >= 2);

    let mut len = 0;

    // remove the start and ending quotations
    let s = &s[1..s.len() - 1];
    let chars: Vec<char> = s.chars().collect();
    let mut i = 0;
    while i < chars.len() {
        if chars[i] == '\\' && chars[i + 1] == '\\' {
            i += 2;
        } else if chars[i] == '\\' && chars[i + 1] == 'x' {
            i += 4;
        } else if chars[i] == '\\' && chars[i + 1] == '"' {
            i += 2;
        } else {
            i += 1
        }
        len += 1;
    }

    len
}

fn len_str_in_code(s: &str) -> i32 {
    // start with 2 to count
    // wrapping the string in quotation marks then
    // count each char, incrementing by 1 normally
    // but increment by 2 for special chars: /, "
    let mut len = 2;

    for c in s.chars() {
        match c {
            '\\' | '"' => len += 2,
            _ => len += 1,
        }
    }

    len
}

pub fn day_8(input: &str) {
    let input: Vec<&str> = input.trim().split("\n").collect();

    let mut num_code_chars: i32 = 0;
    let mut num_mem_chars: i32 = 0;

    let mut num_encoded_chars: i32 = 0;

    for s in input {
        num_code_chars += s.len() as i32;
        num_mem_chars += len_str_in_mem(s);
        num_encoded_chars += len_str_in_code(s);
    }

    println!("2015.8 Part 1: {}", num_code_chars - num_mem_chars);
    println!("2015.8 Part 2: {}", num_encoded_chars - num_code_chars);
}
