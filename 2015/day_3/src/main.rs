use std::{collections::HashSet, fs};

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

fn part1(input: &String) {
    let mut pos = (0, 0);
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    visited.insert((0, 0));

    for c in input.trim().chars() {
        update_pos(&mut pos, c);
        visited.insert(pos);
    }

    assert_eq!(visited.len(), 2572);
}

fn part2(input: &String) {
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

    assert_eq!(visited.len(), 2631);
}

fn main() {
    let input = fs::read_to_string("input/3.txt").expect("file not found");
    part1(&input);
    part2(&input);
}
