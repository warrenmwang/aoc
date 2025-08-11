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

    println!("Part 1 Answer: {}", results.len());
}

fn part_2(mut molecule: String, mut mappings: Vec<(&str, &str)>) {
    // "How long will it take to make the medicine?
    // Given the available replacements and the medicine
    // molecule in your puzzle input, what is the fewest number
    // of steps to go from e to the medicine molecule?"

    // TODO: simple greedy approach, taking the longest matching reverse rule always?
    let mut mappings = mappings
        .iter_mut()
        .map(|&mut (s1, s2)| (s1.to_string(), s2.to_string()))
        .collect::<Vec<(String, String)>>();
    mappings.sort_by(|t1, t2| t2.1.len().cmp(&t1.1.len()));

    println!("before");
    println!("{:?}", mappings);
    println!("{}", molecule);

    let mut count = 0;

    // while molecule != String::from('e') {
    //     for (map_to, map_from) in mappings.into_iter() {
    //         molecule = molecule.replace(&map_from, &map_to);
    //         count += 1;
    //     }
    // }

    // for (map_to, map_from) in mappings {
    //     while molecule.contains(&map_from) {
    //         molecule = molecule.replace(&map_from, &map_to);
    //         count += 1;

    //         if molecule == String::from("e") {
    //             println!("Part 2: {}", count);
    //             return;
    //         }
    //     }
    // }

    println!("unfinished molecule: {}", molecule);
    println!("count: {}", count)

    // println!("after");
    // println!("{}", molecule);
    // println!("{}", molecule);

    // todo!("Do part 2!");
}

pub fn day_19(input: &str) {
    // https://adventofcode.com/2015/day/19

    // let input = fs::read_to_string("test.txt").unwrap();

    let input = input.replace("\r", "");
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
