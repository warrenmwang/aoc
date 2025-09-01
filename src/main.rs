use core::panic;
use std::collections::HashMap;
use std::env;
use std::fs;

mod year_2015;
mod year_2016;

fn setup_solutions() -> (
    Vec<(&'static str, fn(&str))>,
    HashMap<&'static str, fn(&str)>,
) {
    let mut vec: Vec<(&'static str, fn(&str))> = Vec::new();
    vec.push(("2015.1", year_2015::day_1));
    vec.push(("2015.2", year_2015::day_2));
    vec.push(("2015.3", year_2015::day_3));
    vec.push(("2015.4", year_2015::day_4));
    vec.push(("2015.5", year_2015::day_5));
    vec.push(("2015.6", year_2015::day_6));
    vec.push(("2015.7", year_2015::day_7));
    vec.push(("2015.8", year_2015::day_8));
    vec.push(("2015.9", year_2015::day_9));
    vec.push(("2015.10", year_2015::day_10));
    vec.push(("2015.11", year_2015::day_11));
    vec.push(("2015.12", year_2015::day_12));
    vec.push(("2015.13", year_2015::day_13));
    vec.push(("2015.14", year_2015::day_14));
    vec.push(("2015.15", year_2015::day_15));
    vec.push(("2015.16", year_2015::day_16));
    vec.push(("2015.17", year_2015::day_17));
    vec.push(("2015.18", year_2015::day_18));
    vec.push(("2015.19", year_2015::day_19));
    vec.push(("2016.1", year_2016::day_1));

    let mut map: HashMap<&'static str, fn(&str)> = HashMap::new();
    for (k, v) in vec.iter().clone() {
        map.insert(k, *v);
    }
    (vec, map)
}

fn run_one_solution(solution_id: &str, solutions: &HashMap<&'static str, fn(&str)>) {
    let parts: Vec<&str> = solution_id.split(".").collect();
    if parts.len() != 2 {
        panic!("Incorrect format for solution id found. Expect `<year>.<day>`");
    }
    let year = parts[0];
    let day = parts[1];
    let input = fs::read_to_string(format!("inputs/year_{}/day_{}.txt", year, day))
        .expect("Input file not found.");
    let input = input.replace("\r", "");
    let input = input.as_str();
    if !solutions.contains_key(solution_id) {
        panic!("solution id not found: {}", solution_id);
    }
    let solution_fn = solutions.get(solution_id).unwrap();
    solution_fn(input);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!(
            "Incorrect number of arguments, need the year and day to compute a particular solution (e.g. 2015.1). Or provide 'all' to indicate running ALL available solutions in repository."
        );
    }
    let solution_id = &args[1];

    let (solutions_vec, solutions_map) = setup_solutions();
    match solution_id.as_str() {
        "all" => {
            for (k, _) in solutions_vec {
                run_one_solution(k, &solutions_map); // TODO: could parallelize this
            }
        }
        _ => run_one_solution(solution_id, &solutions_map),
    }
}
