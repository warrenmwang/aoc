use std::collections::HashSet;

use crate::SolutionInput;

fn update_pos(pos: &mut (i32, i32), c: char) {
    match c {
        '^' => {
            pos.1 += 1;
        }
        '>' => {
            pos.0 += 1;
        }
        'v' => {
            pos.1 -= 1;
        }
        '<' => {
            pos.0 -= 1;
        }
        _ => panic!("unexpected character seen"),
    }
}

fn part_1(input: &str) -> usize {
    let mut pos = (0, 0);
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    visited.insert((0, 0));

    for c in input.trim().chars() {
        update_pos(&mut pos, c);
        visited.insert(pos);
    }

    visited.len()
}

fn part_2(input: &str) -> usize {
    let mut pos1 = (0, 0);
    let mut pos2 = (0, 0);
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    for (i, c) in input.trim().chars().enumerate() {
        if i % 2 == 0 {
            update_pos(&mut pos1, c);
            visited.insert(pos1);
        } else {
            update_pos(&mut pos2, c);
            visited.insert(pos2);
        }
    }
    visited.len()
}

pub fn day_3(input: SolutionInput) {
    let text_input = input.text_input;
    let term = input.term;
    let part_1_result = format!("2015.3 Part 1: {}", part_1(text_input));
    term.update_line(input.stdout_start_line, part_1_result);
    term.render();
    let part_2_result = format!("2015.3 Part 2: {}", part_2(text_input));
    term.update_line(input.stdout_start_line + 1, part_2_result);
    term.render();
}
