use std::{collections::HashMap, fs};

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

fn part1(input: &String) {
    let lines: Vec<_> = input.trim().split("\n").collect();
    let mut num_nice = 0;
    for line in lines {
        if is_nice(&line.to_string()) {
            num_nice += 1;
        }
    }
    assert_eq!(num_nice, 255);
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

fn part2(input: &String) {
    let lines: Vec<_> = input.trim().split("\n").collect();
    let mut num_nice_too = 0;

    for line in lines {
        if is_nice_too(&line.to_string()) {
            num_nice_too += 1
        }
    }

    assert_eq!(num_nice_too, 55);
}

fn main() {
    let input = fs::read_to_string("input/5.txt").expect("file not found");

    part1(&input);
    part2(&input);
}
