use std::collections::HashMap;

use crate::{SolutionInput, terminal};

fn is_nice(s: &String) -> bool {
    let mut vowels: Vec<char> = Vec::new();
    let mut twice_in_row = false;

    for (i, c) in s.chars().enumerate() {
        // vowel counting
        match c {
            'a' | 'e' | 'i' | 'o' | 'u' => {
                vowels.push(c);
            }
            _ => {}
        }

        if i != s.len() - 1 {
            // twice in row chars
            if !twice_in_row && c == (s.as_bytes()[i + 1] as char) {
                twice_in_row = true;
            }

            // detect bad sequence
            match &s[i..=i + 1] {
                "ab" | "cd" | "pq" | "xy" => return false,
                _ => {}
            }
        }
    }

    vowels.len() >= 3 && twice_in_row
}

fn part_1(input: &str) -> i32 {
    let lines: Vec<&str> = input.trim().split("\n").collect();
    let mut num_nice = 0;
    for line in lines {
        if is_nice(&line.to_string()) {
            num_nice += 1;
        }
    }
    num_nice
}

fn is_nice_too(input: &String) -> bool {
    let mut double_pair = false;
    let mut pairs: HashMap<(char, char), (usize, usize)> = HashMap::new();

    let chars: Vec<char> = input.chars().collect();
    let chars_len = input.chars().count();
    let mut pair: (char, char);

    // find at least one double pair that has no overlap
    for i in 0..chars_len - 1 {
        pair = (chars[i], chars[i + 1]);
        if !pairs.contains_key(&pair) {
            pairs.insert(pair, (i, i + 1));
        } else if i != pairs.get(&pair).unwrap().1 {
            double_pair = true;
            break;
        }
    }

    // find "triplet"
    let mut triplet = false;
    for i in 0..chars_len - 2 {
        if chars[i] == chars[i + 2] {
            triplet = true;
            break;
        }
    }

    double_pair && triplet
}

fn part_2(input: &str) -> i32 {
    let lines: Vec<&str> = input.trim().split("\n").collect();
    let mut num_nice_too = 0;

    for line in lines {
        if is_nice_too(&line.to_string()) {
            num_nice_too += 1
        }
    }
    num_nice_too
}

pub fn day_5(input: SolutionInput) {
    if input.run_in_standalone {
        println!("2015.5 Part 1: {}", part_1(input.text_input));
        println!("2015.5 Part 2: {}", part_2(input.text_input));
    } else {
        let part_1_result = format!("2015.5 Part 1: {}", part_1(input.text_input));
        terminal::print_at_line_stdout(input.stdout_start_line, part_1_result);
        let part_2_result = format!("2015.5 Part 2: {}", part_2(input.text_input));
        terminal::print_at_line_stdout(input.stdout_start_line + 1, part_2_result);
    }
}
