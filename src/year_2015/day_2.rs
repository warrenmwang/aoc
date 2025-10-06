use std::cmp::min;

use crate::{SolutionInput, terminal};

fn part_1(input: &str) -> i32 {
    let mut total = 0;
    let lines: Vec<&str> = input.trim().split("\n").collect();
    for line in lines {
        let x: Vec<&str> = line.split("x").collect();
        let l: i32 = x[0].parse().unwrap();
        let w: i32 = x[1].parse().unwrap();
        let h: i32 = x[2].parse().unwrap();

        let s1 = l * w;
        let s2 = w * h;
        let s3 = h * l;

        total += 2 * s1 + 2 * s2 + 2 * s3 + min(s1, min(s2, s3));
    }
    total
}

fn part_2(input: &str) -> i32 {
    let mut total = 0;
    let lines: Vec<&str> = input.trim().split("\n").collect();
    for line in lines {
        let mut x = line
            .split("x")
            .map(|n| n.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        x.sort();

        let s1: i32 = x[0];
        let s2: i32 = x[1];
        let s3: i32 = x[2];

        total += 2 * s1 + 2 * s2 + (s1 * s2 * s3);
    }
    total
}

pub fn day_2(input: SolutionInput) {
    if input.run_in_standalone {
        println!("2015.2 Part 1: {}", part_1(input.text_input));
        println!("2015.2 Part 2: {}", part_2(input.text_input));
    } else {
        let part_1_result = format!("2015.2 Part 1: {}", part_1(input.text_input));
        terminal::print_at_line_stdout(input.stdout_start_line, part_1_result);
        let part_2_result = format!("2015.2 Part 2: {}", part_2(input.text_input));
        terminal::print_at_line_stdout(input.stdout_start_line + 1, part_2_result);
    }
}
