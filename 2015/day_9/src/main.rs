use std::fs;

fn main() {
    let input = fs::read_to_string("input/9.txt").expect("file not found");
    let input: Vec<_> = input.trim().split("\n").collect();

    // TODO: shortest hamiltonian path problem...
}
