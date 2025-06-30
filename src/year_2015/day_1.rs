fn part_1(input: &str) {
    let mut res = 0;
    for c in input.replace("\n", "").chars() {
        match c {
            '(' => res += 1,
            ')' => res -= 1,
            _ => panic!("unexpected char observed"),
        }
    }
    println!("Part 1: {}", res);
}

fn part_2(input: &str) {
    let mut floor = 0;
    for (i, c) in input.replace("\n", "").chars().enumerate() {
        match c {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => panic!("unexpected char observed"),
        }
        if floor < 0 {
            println!("Part 2: {}", i + 1);
            break;
        }
    }
}

pub fn day_1(input: &str) {
    part_1(&input);
    part_2(&input);
}
