use std::io::{self, Write};

pub fn clear_stdout() {
    println!("\x1b[2J");
    io::stdout().flush().unwrap();
}

// TODO: use crossterm instead, idk what im doing?

// TODO: we actually do no need a lock on stdout for this...
// o.w. we do get timing issues
pub fn print_at_line_stdout(line_num: usize, val: impl std::fmt::Display) {
    print!("\x1b[{};1H", line_num); // go to line_num line
    print!("\x1b[2K"); // clear line
    print!("{}", val);
    io::stdout().flush().unwrap(); // TODO: create a single instantation of io and pass a ref to it to all places that could call this? or do that and write an impl on SolutionInput to use this function in an OOP way?
}

pub fn move_cursor_to(line_num: usize) {
    print!("\x1b[{};1H", line_num); // go to line_num line
}

pub fn create_n_newlines(num_lines: usize) {
    for _ in 0..num_lines {
        println!();
    }
    print!("\x1b[H");
    io::stdout().flush().unwrap();
}
