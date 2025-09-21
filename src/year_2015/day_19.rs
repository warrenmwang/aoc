use std::collections::HashSet;

fn part_1(molecule: String, mappings: Vec<(&str, &str)>) {
    // "How many distinct molecules can be created after all the
    // different ways you can do one replacement on the medicine molecule?"

    let mut results: HashSet<String> = HashSet::new();

    for (sub_str, replacement) in mappings {
        let mut p1: usize = 0;
        let mut p2: usize = sub_str.len();

        while p2 <= molecule.len() {
            if &molecule[p1..p2] == sub_str {
                let mut potential_new_molecule = molecule.clone();
                potential_new_molecule.replace_range(p1..p2, replacement);
                results.insert(potential_new_molecule);
            }
            p1 += 1;
            p2 += 1;
        }
    }

    println!("2015.19 Part 1: {}", results.len());
}

fn part_2(mut molecule: String, mut mappings: Vec<(&str, &str)>) {
    // "How long will it take to make the medicine?
    // Given the available replacements and the medicine
    // molecule in your puzzle input, what is the fewest number
    // of steps to go from e to the medicine molecule?"

    // try trimming search space?
    // remove rules that generate sequence including "CRn" which is useless by my analysis.
    // yea idk wtf is going on with this one

    // TODO:
    // println!("before");
    // println!("{:?}", mappings);
    // println!("{}", molecule);
    println!("2015.19 Part 2: WIP");
}

pub fn day_19(input: &str) {
    // https://adventofcode.com/2015/day/19
    let input: Vec<&str> = input.trim().split("\n").collect();

    let mut mappings: Vec<(&str, &str)> = Vec::new();
    let mut molecule: String = String::from("");

    let mut flag = true;
    for line in input {
        if flag {
            let parts: Vec<&str> = line.trim().split(" => ").collect();
            if parts.len() == 2 {
                mappings.push((parts[0], parts[1]));
            } else {
                flag = false;
            }
        } else {
            molecule = String::from(line);
        }
    }

    part_1(molecule.clone(), mappings.clone());
    part_2(molecule, mappings);
}
