use crate::SolutionInput;

fn look_and_say(s: String) -> String {
    let mut count = 0;
    let mut res = String::new();

    let mut prev_seen_num: i32 = -1;

    for (i, c) in s.chars().into_iter().enumerate() {
        let num = c.to_digit(10).unwrap() as i32;

        if i == 0 {
            count = 1;
            prev_seen_num = num;
            continue;
        }

        if num == prev_seen_num {
            count += 1;
        } else {
            res.push_str(&format!("{}{}", count, prev_seen_num));
            prev_seen_num = num;
            count = 1;
        }
    }

    res.push_str(&format!("{}{}", count, prev_seen_num));

    res
}

pub fn day_10(input: SolutionInput) {
    let mut res = String::from(input.text_input);
    let mut part_1_res: String = String::new();

    for i in 0..50 {
        res = look_and_say(res);
        if i == 39 {
            part_1_res = res.clone();
        }
    }

    let part_1_result = format!("2015.10 Part 1: {}", part_1_res.len());
    let part_2_result = format!("2015.10 Part 2: {}", res.len());
    let term = input.term;
    term.update_line(input.stdout_start_line, part_1_result);
    term.update_line(input.stdout_start_line + 1, part_2_result);
    term.render();
}
