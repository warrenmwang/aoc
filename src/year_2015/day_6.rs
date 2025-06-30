use std::cmp::max;

fn update_grid(grid: &mut Vec<Vec<u32>>, line: &str) {
    let tmp: Vec<&str> = line.split(" ").collect();
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

fn part_1(input: &str) {
    let mut grid: Vec<Vec<u32>> = vec![vec![0; 1000]; 1000];
    let lines: Vec<&str> = input.trim().split("\n").collect();
    for line in lines {
        update_grid(&mut grid, line);
    }
    println!("Part 1: {}", grid.iter().flatten().sum::<u32>());
}

fn update_grid_2(grid: &mut Vec<Vec<i32>>, line: &str) {
    let tmp: Vec<&str> = line.split(" ").collect();
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

fn part_2(input: &str) {
    let mut grid: Vec<Vec<i32>> = vec![vec![0; 1000]; 1000];
    let lines: Vec<&str> = input.trim().split("\n").collect();
    for line in lines {
        update_grid_2(&mut grid, line);
    }
    println!("Part 2: {}", grid.iter().flatten().sum::<i32>());
}

pub fn day_6(input: &str) {
    part_1(input);
    part_2(input);
}
