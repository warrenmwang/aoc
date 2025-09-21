use core::panic;
use std::collections::HashMap;
use std::env;
use std::sync::{Arc, Mutex};
use std::{fs, num::NonZeroUsize, thread};

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

fn run_solution(solution_id: &str, solution_fn: &fn(&str)) {
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
    solution_fn(input);
}

pub fn run_all_solutions(solutions: Vec<(&'static str, fn(&str))>) {
    let num_cpus = match thread::available_parallelism() {
        Ok(num_cpus) => num_cpus.get(),
        Err(_) => 1,
    };

    if num_cpus == 1 {
        for (solution_id, solution_fn) in solutions.iter() {
            run_solution(solution_id, solution_fn);
        }
        return;
    }

    let num_work = solutions.len();

    let num_cpus = num_cpus / 2; // NOTE: use half of available cores

    println!("num cpus: {}", num_cpus);
    println!("num sols: {}", num_work);

    let mut num_threads_to_use = num_cpus;
    let solutions = Arc::new(solutions);

    // TODO: want a nice way to print solutions in an ordered way when run in parallel
    // with different finish times for each solution computed
    if num_work <= num_cpus {
        num_threads_to_use = num_work;
        let mut handles = Vec::new();

        for i in 0..num_threads_to_use {
            let solutions_shared = Arc::clone(&solutions);
            let handle = thread::spawn(move || {
                let tuple = solutions_shared.get(i).unwrap();
                let solution_id = tuple.0;
                let solution_fn = &tuple.1;
                run_solution(solution_id, solution_fn);
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().expect("Thread panicked.");
        }

        return;
    }

    let work_index = Arc::new(Mutex::new(0));
    let mut handles = Vec::new();

    for _ in 0..num_threads_to_use {
        let work_index = Arc::clone(&work_index);
        let solutions_shared = Arc::clone(&solutions);
        let handle = thread::spawn(move || {
            loop {
                let mut curr_work_index = work_index.lock().unwrap();
                if *curr_work_index >= num_work {
                    return;
                }
                let tuple = solutions_shared.get(*curr_work_index).unwrap();
                *curr_work_index += 1;
                drop(curr_work_index);
                let solution_id = tuple.0;
                let solution_fn = &tuple.1;
                run_solution(solution_id, solution_fn);
            }
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().expect("Thread panicked.");
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!(
            "Incorrect number of arguments, need the year and day to compute a particular solution (e.g. 2015.1). Or provide 'all' to indicate running ALL available solutions in repository."
        );
    }
    let solution_id = args[1].as_str();

    let (solutions_vec, solutions_map) = setup_solutions();
    match solution_id {
        "all" => {
            run_all_solutions(solutions_vec);
        }
        _ => {
            let solution_fn = match solutions_map.get(solution_id) {
                Some(solution_fn) => solution_fn,
                None => {
                    println!("Unable to find the solution {}", solution_id);
                    return;
                }
            };
            run_solution(solution_id, solution_fn);
        }
    }
}
