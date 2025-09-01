// assume input in grid_1, output next state into grid_2
fn animate(grid_1: &mut Vec<Vec<u8>>, grid_2: &mut Vec<Vec<u8>>) {
    fn check_numbers(grid: &mut Vec<Vec<u8>>, i: usize, j: usize) -> u8 {
        let mut sum = 0;

        let row_mid = grid.get(i);

        let row_above: Option<&Vec<u8>>;
        if i > 0 {
            row_above = Some(&grid[i - 1]);
        } else {
            row_above = None;
        }

        let row_below: Option<&Vec<u8>>;
        if i < 99 {
            row_below = Some(&grid[i + 1]);
        } else {
            row_below = None;
        }

        if let Some(row) = row_above {
            let col_mid = row.get(j);

            let col_left: Option<&u8>;
            if j > 0 {
                col_left = Some(&row[j - 1]);
            } else {
                col_left = None;
            }

            let col_right: Option<&u8>;
            if j < 99 {
                col_right = Some(&row[j + 1])
            } else {
                col_right = None;
            }

            if let Some(col) = col_left {
                sum += col;
            }
            if let Some(col) = col_mid {
                sum += col;
            }
            if let Some(col) = col_right {
                sum += col;
            }
        }

        if let Some(row) = row_mid {
            let col_left: Option<&u8>;
            if j > 0 {
                col_left = Some(&row[j - 1]);
            } else {
                col_left = None;
            }

            let col_right: Option<&u8>;
            if j < 99 {
                col_right = Some(&row[j + 1])
            } else {
                col_right = None;
            }

            if let Some(col) = col_left {
                sum += col;
            }
            if let Some(col) = col_right {
                sum += col;
            }
        }

        if let Some(row) = row_below {
            let col_mid = row.get(j);

            let col_left: Option<&u8>;
            if j > 0 {
                col_left = Some(&row[j - 1]);
            } else {
                col_left = None;
            }

            let col_right: Option<&u8>;
            if j < 99 {
                col_right = Some(&row[j + 1])
            } else {
                col_right = None;
            }

            if let Some(col) = col_left {
                sum += col;
            }
            if let Some(col) = col_mid {
                sum += col;
            }
            if let Some(col) = col_right {
                sum += col;
            }
        }

        sum
    }

    for i in 0..100 {
        for j in 0..100 {
            let count = check_numbers(grid_1, i, j);
            let curr_state = grid_1[i][j];

            // A light which is on stays on when 2 or 3 neighbors are on, and turns off otherwise.
            if curr_state == 1 {
                if count == 2 || count == 3 {
                    grid_2[i][j] = 1;
                } else {
                    grid_2[i][j] = 0;
                }
            } else {
                // A light which is off turns on if exactly 3 neighbors are on, and stays off otherwise.
                if count == 3 {
                    grid_2[i][j] = 1;
                } else {
                    grid_2[i][j] = 0;
                }
            }
        }
    }
}

fn part_1(mut grid_1: Vec<Vec<u8>>, mut grid_2: Vec<Vec<u8>>) {
    // update state by flip flopping between the 2 buffers for arbitrary N times
    let n = 100;
    for i in 0..n {
        if i % 2 == 0 {
            animate(&mut grid_1, &mut grid_2);
        } else {
            animate(&mut grid_2, &mut grid_1);
        }
    }

    // sum "on" cells
    let mut sum: i32 = 0;
    let grid: &Vec<Vec<u8>>;
    if n % 2 == 0 {
        grid = &grid_1;
    } else {
        grid = &grid_2;
    }
    for i in 0..100 {
        for j in 0..100 {
            sum += grid[i][j] as i32;
        }
    }
    println!("2015.18 Part 1: {}", sum);
}

fn part_2(mut grid_1: Vec<Vec<u8>>, mut grid_2: Vec<Vec<u8>>) {
    // update state by flip flopping between the 2 buffers for arbitrary N times
    // BUT we always make sure the corners are on!
    let n = 100;
    for i in 0..n {
        if i % 2 == 0 {
            animate(&mut grid_1, &mut grid_2);
            grid_2[0][0] = 1;
            grid_2[0][99] = 1;
            grid_2[99][0] = 1;
            grid_2[99][99] = 1;
        } else {
            animate(&mut grid_2, &mut grid_1);
            grid_1[0][0] = 1;
            grid_1[0][99] = 1;
            grid_1[99][0] = 1;
            grid_1[99][99] = 1;
        }
    }

    // sum "on" cells
    let mut sum: i32 = 0;
    let grid: &Vec<Vec<u8>>;
    if n % 2 == 0 {
        grid = &grid_1;
    } else {
        grid = &grid_2;
    }
    for i in 0..100 {
        for j in 0..100 {
            sum += grid[i][j] as i32;
        }
    }
    println!("2015.18 Part 2: {}", sum);
}

pub fn day_18(input: &str) {
    let input: Vec<&str> = input.trim().split("\n").collect();

    let mut grid_1: Vec<Vec<u8>> = vec![vec![0; 100]; 100];
    let grid_2: Vec<Vec<u8>> = vec![vec![0; 100]; 100];

    // construct our first grid
    let mut i: usize = 0;
    for row in input {
        let mut j: usize = 0;
        for c in row.trim().chars() {
            match c {
                '#' => {
                    grid_1[i][j] = 1;
                }
                '.' => {
                    grid_1[i][j] = 0;
                }
                _ => {
                    panic!("unexpected character: {}", c);
                }
            }
            j += 1;
        }
        i += 1;
    }

    part_1(grid_1.clone(), grid_2.clone());

    // part 2: turn on the four corners at start
    grid_1[0][0] = 1;
    grid_1[0][99] = 1;
    grid_1[99][0] = 1;
    grid_1[99][99] = 1;
    part_2(grid_1, grid_2);
}
