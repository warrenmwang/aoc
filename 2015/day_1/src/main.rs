use std::fs;

fn part1(input: &String) {
    let mut res = 0;
    for c in input.replace("\n", "").chars() {
        match c {
            '(' => res += 1,
            ')' => res -= 1,
            _ => panic!("unexpected char observed"),
        }
    }

    assert_eq!(res, 280);
}

fn part2(input: &String) {
    let mut floor = 0;
    for (i, c) in input.replace("\n", "").chars().enumerate() {
        match c {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => panic!("unexpected char observed"),
        }
        if floor < 0 {
            assert_eq!(i + 1, 1797);
            break;
        }
    }
}

fn main() {
    let input = fs::read_to_string("input/1.txt").expect("file not found");
    part1(&input);
    part2(&input);
}
