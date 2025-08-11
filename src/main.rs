use core::panic;
use std::env;
use std::fs;

mod runners;
mod year_2015;
mod year_2016;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!(
            "Incorrect number of arguments, need the year and day to compute a particular solution (e.g. 2015.1). Or provide 'all' to indicate running ALL available solutions in repository."
        );
    }
    let solution_id = &args[1];
    if solution_id.as_str() == "all" {
        runners::run_all_solutions();
    }

    let parts: Vec<&str> = solution_id.split(".").collect();
    if parts.len() != 2 {
        panic!("Incorrect format for solution id found. Expect `<year>.<day>`");
    }
    let year = parts[0];
    let day = parts[1];
    let input = fs::read_to_string(format!("inputs/year_{}/day_{}.txt", year, day))
        .expect("Input file not found.");

    match solution_id.as_str() {
        "2015.1" => year_2015::day_1(input.as_str()),
        "2015.2" => year_2015::day_2(input.as_str()),
        "2015.3" => year_2015::day_3(input.as_str()),
        "2015.4" => year_2015::day_4(input.as_str()),
        "2015.5" => year_2015::day_5(input.as_str()),
        "2015.6" => year_2015::day_6(input.as_str()),
        "2015.7" => year_2015::day_7(input.as_str()),
        "2015.8" => year_2015::day_8(input.as_str()),
        "2015.9" => year_2015::day_9(input.as_str()),
        "2015.10" => year_2015::day_10(input.as_str()),
        "2015.11" => year_2015::day_11(input.as_str()),
        "2015.12" => year_2015::day_12(input.as_str()),
        "2015.13" => year_2015::day_13(input.as_str()),
        "2015.14" => year_2015::day_14(input.as_str()),
        "2015.15" => year_2015::day_15(input.as_str()),
        "2015.16" => year_2015::day_16(input.as_str()),
        "2015.17" => year_2015::day_17(input.as_str()),
        "2015.18" => year_2015::day_18(input.as_str()),
        "2015.19" => year_2015::day_19(input.as_str()),
        "2016.1" => year_2016::day_1(input.as_str()),
        _ => panic!("Could not find the requested solution to compute."),
    }
}
