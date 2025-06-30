use std::thread::available_parallelism;

pub fn run_all_solutions() {
    let default_parallelism_approx = available_parallelism().unwrap().get();
    let num_threads_to_use = default_parallelism_approx / 2;

    println!("{}", num_threads_to_use);

    todo!("Compute all solutions");
}
