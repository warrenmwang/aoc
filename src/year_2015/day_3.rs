use std::collections::HashSet;

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

fn part_1(input: &str) {
    let mut pos = (0, 0);
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    visited.insert((0, 0));

    for c in input.trim().chars() {
        update_pos(&mut pos, c);
        visited.insert(pos);
    }

    println!("Part 1: {}", visited.len());
}

fn part_2(input: &str) {
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

    println!("Part 2: {}", visited.len());
}

pub fn day_3(input: &str) {
    part_1(input);
    part_2(input);
}
