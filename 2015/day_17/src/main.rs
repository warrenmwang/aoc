use std::{collections::HashMap, fs};

fn count_combinations(
    target: i32,
    current_index: usize,
    containers: &Vec<i32>,
    cached_results: &mut HashMap<(i32, usize), u32>,
) -> u32 {
    // base case: this computation is cached, return cached result
    if let Some(result) = cached_results.get(&(target, current_index)) {
        return *result;
    }

    // base case: no combinations are valid if sum of remaining
    // containers is less than target
    if containers
        .get(current_index..)
        .unwrap()
        .iter()
        .map(|&x| x as i32)
        .sum::<i32>()
        < target
    {
        cached_results.insert((target, current_index), 0);
        return 0;
    }

    // base case: finished, assuming never start with target 0
    if target == 0 {
        cached_results.insert((target, current_index), 1);
        return 1;
    }

    // base case: reach end of array and target was not zero
    if current_index == containers.len() {
        cached_results.insert((target, current_index), 0);
        return 0;
    }

    // compute two subbranches
    let take_result = count_combinations(
        target - containers[current_index],
        current_index + 1,
        containers,
        cached_results,
    );
    let not_take_result = count_combinations(target, current_index + 1, containers, cached_results);

    // cache results
    cached_results.insert(
        (target - containers[current_index], current_index + 1),
        take_result,
    );
    cached_results.insert((target, current_index + 1), not_take_result);

    let current_result = take_result + not_take_result;
    cached_results.insert((target, current_index), current_result);

    current_result
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let input: Vec<&str> = input.trim().split("\r\n").collect();

    let target = 150;
    let containers: Vec<i32> = input.iter().map(|x| x.parse().unwrap()).collect();
    println!("target: {}", target);
    println!("containers: {:?}", containers);

    let current_index: usize = 0;
    let mut cached_results: HashMap<(i32, usize), u32> = HashMap::new();
    let count = count_combinations(target, current_index, &containers, &mut cached_results);

    println!("Num combinations: {}", count);
    // part 1: 1638

    // for (&(target, idx), &v) in cached_results.iter() {
    //     if v == 0 {
    //         continue;
    //     }
    //     println!("target: {} idx: {} value: {}", target, idx, v);
    // }

    // TODO: part 2: count minimum number of containers amongst valid combinations
}
