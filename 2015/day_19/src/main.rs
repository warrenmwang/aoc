use std::{collections::HashSet, fs};

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let input: Vec<&str> = input.trim().split("\r\n").collect();

    let mut mappings: Vec<(&str, &str)> = Vec::new();
    let mut molecule: String = String::from("");

    let mut flag = true;
    for line in input {
        if flag {
            let parts: Vec<&str> = line.trim().split(" => ").collect();
            if parts.len() == 2 {
                mappings.push((parts[0], parts[1]));
            } else {
                flag = false;
            }
        } else {
            molecule = String::from(line);
        }
    }

    let mut results: HashSet<String> = HashSet::new();
    for (sub_str, replacement) in mappings {
        // println!("k, v = {}, {}", sub_str, replacement);
        let mut p1: usize = 0;
        let mut p2: usize = sub_str.len();

        while p2 <= molecule.len() {
            if &molecule[p1..p2] == sub_str {
                // println!("sub_str {} match btwn (p1, p2): ({},{})", sub_str, p1, p2);
                let mut value = molecule.clone();
                value.replace_range(p1..p2, replacement);
                results.insert(value);
            }
            p1 += 1;
            p2 += 1;
        }
    }

    println!("{}", results.len());
    // 573 is too low
    // 576 correct

    // TODO: Part 2 ????????
}
