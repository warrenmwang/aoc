use std::{cmp::min, fs};

fn part1(input: &String) {
    let mut total = 0;
    let lines: Vec<_> = input.trim().split("\n").collect();
    for line in lines {
        let x: Vec<_> = line.split("x").collect();
        let l: i32 = x[0].parse().unwrap();
        let w: i32 = x[1].parse().unwrap();
        let h: i32 = x[2].parse().unwrap();

        let s1 = l * w;
        let s2 = w * h;
        let s3 = h * l;

        total += 2 * s1 + 2 * s2 + 2 * s3 + min(s1, min(s2, s3));
    }
    assert_eq!(total, 1606483);
}

fn part2(input: &String) {
    let mut total = 0;
    let lines: Vec<_> = input.trim().split("\n").collect();
    for line in lines {
        let mut x = line
            .split("x")
            .map(|n| n.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        x.sort();

        let s1: i32 = x[0];
        let s2: i32 = x[1];
        let s3: i32 = x[2];

        total += 2 * s1 + 2 * s2 + (s1 * s2 * s3);
    }
    assert_eq!(total, 3842356);
}

fn main() {
    let input = fs::read_to_string("input/2.txt").expect("file not found");
    part1(&input);
    part2(&input);
}
