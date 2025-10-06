use crate::SolutionInput;

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

// TODO: this takes relatively long time. find a faster way?
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
    let text_input = input.text_input;
    let term = input.term;
    let part_1_result = format!("2015.4 Part 1: {}", part_1(text_input));
    term.update_line(input.stdout_start_line, part_1_result);
    term.render();
    let part_2_result = format!("2015.4 Part 2: {}", part_2(text_input));
    term.update_line(input.stdout_start_line + 1, part_2_result);
    term.render();
}
