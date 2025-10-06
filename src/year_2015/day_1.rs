use crate::{SolutionInput, terminal};

fn part_1(input: &str) -> i32 {
    let mut res = 0;
    for c in input.replace("\n", "").chars() {
        match c {
            '(' => res += 1,
            ')' => res -= 1,
            _ => panic!("unexpected char observed"),
        }
    }
    res
}

fn part_2(input: &str) -> usize {
    let mut floor = 0;
    let mut res = 0;
    for (i, c) in input.replace("\n", "").chars().enumerate() {
        match c {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => panic!("unexpected char observed"),
        }
        if floor < 0 {
            res = i + 1;
            break;
        }
    }
    res
}

pub fn day_1(input: SolutionInput) {
    if input.run_in_standalone {
        println!("2015.1 Part 1: {}", part_1(input.text_input));
        println!("2015.1 Part 2: {}", part_2(input.text_input));
    } else {
        let part_1_result = format!("2015.1 Part 1: {}", part_1(input.text_input));
        terminal::print_at_line_stdout(input.stdout_start_line, part_1_result);
        let part_2_result = format!("2015.1 Part 2: {}", part_2(input.text_input));
        terminal::print_at_line_stdout(input.stdout_start_line + 1, part_2_result);
    }
}
