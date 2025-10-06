use std::collections::HashSet;

use crate::{SolutionInput, terminal};

fn get_next_orientation(direction: char, orientation: &mut u8) {
    match direction {
        'L' => {
            if *orientation == 0 {
                *orientation = 3;
            } else {
                *orientation -= 1;
            }
        }
        'R' => {
            if *orientation == 3 {
                *orientation = 0;
            } else {
                *orientation += 1;
            }
        }
        _ => {
            panic!("unexpected direction change: {}", direction)
        }
    }
}

fn update_position(orientation: &u8, num_steps: u32, x: &mut i32, y: &mut i32) {
    match orientation {
        0 => {
            *y += num_steps as i32;
        }
        1 => {
            *x += num_steps as i32;
        }
        2 => {
            *y -= num_steps as i32;
        }
        3 => {
            *x -= num_steps as i32;
        }
        _ => {
            panic!("unexpected orientation in update_position: {}", orientation);
        }
    }
}

fn part_1(input: &str) -> i32 {
    let mut orientation: u8 = 0; // north 0, east 1, south 2, west 3
    let mut x = 0;
    let mut y = 0;

    let parts = input.trim().split(", ").collect::<Vec<&str>>();
    for part in parts {
        let direction = part.chars().next().unwrap();
        let num_steps = part[1..].parse::<u32>().unwrap();
        get_next_orientation(direction, &mut orientation);
        update_position(&orientation, num_steps, &mut x, &mut y);
    }
    x.abs() + y.abs()
}

fn part_2(input: &str) -> i32 {
    let mut visited: HashSet<String> = HashSet::new();
    let mut found_first_visited_twice = false;

    let mut orientation: u8 = 0; // north 0, east 1, south 2, west 3
    let mut x = 0;
    let mut y = 0;

    visited.insert(format!("{},{}", x, y));

    let parts = input.trim().split(", ").collect::<Vec<&str>>();
    for part in parts {
        let direction = part.chars().next().unwrap();
        let num_steps = part[1..].parse::<u32>().unwrap();
        get_next_orientation(direction, &mut orientation);

        // does the job, but quite inefficient
        for _ in 0..num_steps {
            update_position(&orientation, 1, &mut x, &mut y);
            let new_pos = format!("{},{}", x, y);
            if visited.contains(&new_pos) {
                found_first_visited_twice = true;
                break;
            } else {
                visited.insert(new_pos);
            }
        }

        if found_first_visited_twice {
            break;
        }
    }
    x.abs() + y.abs()
}

pub fn day_1(input: SolutionInput) {
    if input.run_in_standalone {
        println!("2016.1 Part 1: {}", part_1(input.text_input));
        println!("2016.1 Part 2: {}", part_2(input.text_input));
    } else {
        let part_1_result = format!("2016.1 Part 1: {}", part_1(input.text_input));
        terminal::print_at_line_stdout(input.stdout_start_line, part_1_result);
        let part_2_result = format!("2016.1 Part 2: {}", part_2(input.text_input));
        terminal::print_at_line_stdout(input.stdout_start_line + 1, part_2_result);
    }
}
