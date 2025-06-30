use std::collections::HashMap;

// cached_results key: (target_sum, current index into the containers array)
// cached_results value: number of valid combinations that sum to target_sum
//
// valid_freq_combination_map key: number of containers
// valid_freq_combination_map value: frequency of this number of containers
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

// for part 2 we cannot cache subtrees bc we gotta "visit" all of the valid solution nodes
// at least, that's what im thinking is how we actually get the full set of freq counts.
fn count_combinations_part_two(
    target: i32,
    current_index: usize,
    containers: &Vec<i32>,
    valid_freq_combination_map: &mut HashMap<u32, u32>,
    num_containers_taken_so_far: u32,
) -> u32 {
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
        return 0;
    }

    // base case: finished, assuming never start with target 0
    if target == 0 {
        // update map for counting the freq of each count of
        // containers for each valid result
        if let Some(prev_count) = valid_freq_combination_map.get(&num_containers_taken_so_far) {
            valid_freq_combination_map.insert(num_containers_taken_so_far, prev_count + 1);
        } else {
            valid_freq_combination_map.insert(num_containers_taken_so_far, 1);
        }

        return 1;
    }

    // base case: reach end of array and target was not zero
    if current_index == containers.len() {
        return 0;
    }

    // compute two subbranches
    let take_result = count_combinations_part_two(
        target - containers[current_index],
        current_index + 1,
        containers,
        valid_freq_combination_map,
        num_containers_taken_so_far + 1,
    );
    let not_take_result = count_combinations_part_two(
        target,
        current_index + 1,
        containers,
        valid_freq_combination_map,
        num_containers_taken_so_far,
    );

    take_result + not_take_result
}

pub fn day_17(input: &str) {
    let target = 150;

    let input = input.replace("\r", "");
    let input: Vec<&str> = input.trim().split("\n").collect();

    let containers: Vec<i32> = input.iter().map(|x| x.parse().unwrap()).collect();
    // println!("target: {}", target);
    // println!("containers: {:?}", containers);

    let current_index: usize = 0;
    let mut cached_results: HashMap<(i32, usize), u32> = HashMap::new();
    let part_1_count = count_combinations(target, current_index, &containers, &mut cached_results);

    println!("Part 1: {}", part_1_count);
    // part 1: 1638

    // part 2: count minimum number of containers amongst valid combinations
    // create a map of counts of valid, unique combinations
    // then get the freq of the minimum container number combinations.
    let current_index: usize = 0;
    let mut valid_freq_map: HashMap<u32, u32> = HashMap::new();
    let part_2_count =
        count_combinations_part_two(target, current_index, &containers, &mut valid_freq_map, 0);

    // the total count of valid combinations should still be the same!
    assert!(part_1_count == part_2_count);

    // but now we have the freq map to get the number of minimum number containers result
    let mut min: Option<u32> = None;
    let mut part_2_result: Option<u32> = None;
    for (&num_containers, &freq) in valid_freq_map.iter() {
        if min == None && part_2_result == None {
            min = Some(num_containers);
            part_2_result = Some(freq);
        } else {
            if num_containers < min.unwrap() {
                min = Some(num_containers);
                part_2_result = Some(freq);
            }
        }
    }

    if let Some(res) = part_2_result {
        // println!("minimum containers needed: {}", min.unwrap());
        println!("Part 2: {}", res);
    } else {
        println!("OH NO! We didn't get a result for part 2!");
    }
}
