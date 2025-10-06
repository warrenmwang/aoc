use crate::{SolutionInput, terminal};

pub fn part_1(input: &str) -> i32 {
    let mut i = 0;
    let mut digest: String;
    loop {
        digest = format!("{:x}", md5::compute(format!("{}{}", input, i)));
        if digest.starts_with("00000") {
            return i;
        }
        i += 1;
    }
}

pub fn part_2(input: &str) -> i32 {
    let mut i = 0;
    let mut digest: String;
    loop {
        digest = format!("{:x}", md5::compute(format!("{}{}", input, i)));
        if digest.starts_with("000000") {
            return i;
        }
        i += 1;
    }
}

pub fn day_4(input: SolutionInput) {
    if input.run_in_standalone {
        println!("2015.4 Part 1: {}", part_1(input.text_input));
        println!("2015.4 Part 2: {}", part_2(input.text_input));
    } else {
        let part_1_result = format!("2015.4 Part 1: {}", part_1(input.text_input));
        terminal::print_at_line_stdout(input.stdout_start_line, part_1_result);
        let part_2_result = format!("2015.4 Part 2: {}", part_2(input.text_input));
        terminal::print_at_line_stdout(input.stdout_start_line + 1, part_2_result);
    }
}
