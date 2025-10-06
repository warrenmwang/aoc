use core::panic;
use std::collections::HashMap;
use std::env;
use std::sync::{Arc, Mutex};
use std::{fs, thread};

use crate::terminal::Terminal;

mod terminal;
mod year_2015;
mod year_2016;

pub struct SolutionInput<'a> {
    text_input: &'a str,
    stdout_start_line: usize,
    term: &'a Terminal,
}

fn setup_solutions() -> (
    Vec<(&'static str, fn(SolutionInput))>,
    HashMap<&'static str, fn(SolutionInput)>,
) {
    let mut vec: Vec<(&'static str, fn(SolutionInput))> = Vec::new();
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

    let mut map: HashMap<&'static str, fn(SolutionInput)> = HashMap::new();
    for (k, v) in vec.iter().clone() {
        map.insert(k, *v);
    }
    (vec, map)
}

fn run_solution_standalone(solution_id: &str, solution_fn: fn(SolutionInput), term: &Terminal) {
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
    let input = SolutionInput {
        text_input: input,
        stdout_start_line: 0,
        term: term,
    };
    solution_fn(input);
}

fn run_solution_parallel(
    solution_id: &str,
    solution_fn: fn(SolutionInput),
    thread_stdout_start_line: usize,
    term: &Terminal,
) {
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
    let input = SolutionInput {
        text_input: input,
        stdout_start_line: thread_stdout_start_line,
        term: term,
    };
    solution_fn(input);
}

pub fn run_all_solutions(solutions: Vec<(&'static str, fn(SolutionInput))>) {
    let num_cpus = match thread::available_parallelism() {
        Ok(num_cpus) => num_cpus.get(),
        Err(_) => 1,
    };

    let num_work = solutions.len();
    let mut num_threads_to_use = num_cpus;
    if num_work < num_threads_to_use {
        num_threads_to_use = num_work;
    }
    let stdout_lines_for_each = 2;

    let header_lines = [
        String::from("--------------------"),
        String::from("Let the solutions roll!..."),
        format!("num sols: {}", num_work),
        format!("num cpus: {}", num_cpus),
        format!("num threads using: {}", num_threads_to_use),
    ];
    let header_lines_length = header_lines.len();

    let term_total_rows = (num_work * stdout_lines_for_each) + header_lines_length;
    let term = Arc::new(Terminal::new(term_total_rows));

    for i in 0..header_lines_length {
        term.update_line(i, header_lines[i].clone());
    }
    term.clean_term();

    if num_cpus == 1 {
        let term = term.clone();
        for (solution_id, solution_fn) in solutions.iter() {
            run_solution_standalone(solution_id, *solution_fn, &term);
        }
        return;
    }

    let solutions = Arc::new(solutions);

    // One thread per solution.
    if num_work <= num_threads_to_use {
        let mut handles = Vec::new();

        for work_index in 0..num_work {
            let solutions_shared = Arc::clone(&solutions);
            let term_shared = Arc::clone(&term);
            let handle = thread::spawn(move || {
                let tuple = solutions_shared.get(work_index).unwrap();
                let solution_id = tuple.0;
                let solution_fn = tuple.1;
                run_solution_parallel(
                    solution_id,
                    solution_fn,
                    (work_index * stdout_lines_for_each) + header_lines_length,
                    &term_shared,
                );
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().expect("Thread panicked.");
        }
    }

    // Use max number of allowed threads to round robin compute all solutions.
    if num_work > num_threads_to_use {
        let work_index = Arc::new(Mutex::new(0));
        let mut handles = Vec::new();

        for _ in 0..num_threads_to_use {
            let work_index = Arc::clone(&work_index);
            let solutions_shared = Arc::clone(&solutions);
            let term_shared = Arc::clone(&term);
            let handle = thread::spawn(move || {
                loop {
                    let mut curr_work_index_guard = work_index.lock().unwrap();
                    if *curr_work_index_guard >= num_work {
                        return;
                    }
                    let curr_work_index_val_copy = *curr_work_index_guard;
                    let tuple = solutions_shared.get(*curr_work_index_guard).unwrap();
                    *curr_work_index_guard += 1;
                    drop(curr_work_index_guard);
                    let solution_id = tuple.0;
                    let solution_fn = tuple.1;
                    run_solution_parallel(
                        solution_id,
                        solution_fn,
                        (curr_work_index_val_copy * stdout_lines_for_each) + header_lines_length,
                        &term_shared,
                    );
                }
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().expect("Thread panicked.");
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!(
            "Incorrect number of arguments, need the year and day to compute a particular solution (e.g. 2015.1). Or provide 'all' to indicate running ALL available solutions in repository."
        );
        return;
    }
    let solution_id = args[1].as_str();

    let (solutions_vec, solutions_map) = setup_solutions();
    match solution_id {
        "all" => {
            run_all_solutions(solutions_vec);
        }
        _ => {
            let solution_fn = match solutions_map.get(solution_id) {
                Some(solution_fn) => *solution_fn,
                None => {
                    println!("Unable to find the solution {}", solution_id);
                    return;
                }
            };
            run_solution_standalone(solution_id, solution_fn, &Terminal::new(2));
        }
    }
}
