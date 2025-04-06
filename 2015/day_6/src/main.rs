use std::cmp::max;
use std::fs;

fn update_grid(grid: &mut Vec<Vec<u32>>, line: &str) {
    let tmp: Vec<_> = line.split(" ").collect();
    let start = tmp[tmp.len() - 3]
        .split(",")
        .map(|x| x.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();
    let end = tmp[tmp.len() - 1]
        .split(",")
        .map(|x| x.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();
    let start_x = start[0] as usize;
    let start_y = start[1] as usize;
    let end_x = end[0] as usize;
    let end_y = end[1] as usize;

    if tmp[0] == "toggle" {
        for row in start_x..end_x + 1 {
            for col in start_y..end_y + 1 {
                grid[row][col] = if grid[row][col] == 0 { 1 } else { 0 };
            }
        }
        return;
    }

    match tmp[1] {
        "on" => {
            for row in start_x..end_x + 1 {
                for col in start_y..end_y + 1 {
                    grid[row][col] = 1;
                }
            }
        }
        "off" => {
            for row in start_x..end_x + 1 {
                for col in start_y..end_y + 1 {
                    grid[row][col] = 0;
                }
            }
        }
        _ => {}
    }
}

fn part1(input: &String) {
    let mut grid: Vec<Vec<u32>> = vec![vec![0; 1000]; 1000];
    let lines: Vec<_> = input.trim().split("\n").collect();
    for line in lines {
        update_grid(&mut grid, line);
    }
    assert_eq!(grid.iter().flatten().sum::<u32>(), 569999);
}

fn update_grid_2(grid: &mut Vec<Vec<i32>>, line: &str) {
    let tmp: Vec<_> = line.split(" ").collect();
    let start = tmp[tmp.len() - 3]
        .split(",")
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
    let end = tmp[tmp.len() - 1]
        .split(",")
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
    let start_x = start[0] as usize;
    let start_y = start[1] as usize;
    let end_x = end[0] as usize;
    let end_y = end[1] as usize;

    if tmp[0] == "toggle" {
        for row in start_x..end_x + 1 {
            for col in start_y..end_y + 1 {
                grid[row][col] += 2;
            }
        }
        return;
    }

    match tmp[1] {
        "on" => {
            for row in start_x..end_x + 1 {
                for col in start_y..end_y + 1 {
                    grid[row][col] += 1;
                }
            }
        }
        "off" => {
            for row in start_x..end_x + 1 {
                for col in start_y..end_y + 1 {
                    grid[row][col] = max(0, grid[row][col] - 1);
                }
            }
        }
        _ => {}
    }
}

fn part2(input: &String) {
    let mut grid: Vec<Vec<i32>> = vec![vec![0; 1000]; 1000];
    let lines: Vec<_> = input.trim().split("\n").collect();
    for line in lines {
        update_grid_2(&mut grid, line);
    }
    assert_eq!(grid.iter().flatten().sum::<i32>(), 17836115);
}

fn main() {
    let input = fs::read_to_string("input/6.txt").expect("file not found");

    part1(&input);
    part2(&input);
}
