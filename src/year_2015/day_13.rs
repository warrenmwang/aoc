use itertools::Itertools;
use std::collections::HashMap;

// each is supposed to be the addition of the gained happiness
// of both directions of person1 <-> person2 and person2 <-> person1
fn get_edge(m: &Vec<Vec<i32>>, lookup: &HashMap<String, usize>, k1: &str, k2: &str) -> i32 {
    let i1 = lookup.get(k1).unwrap();
    let i2 = lookup.get(k2).unwrap();

    m[*i1][*i2] + m[*i2][*i1]
}

fn just_do_it(input: &str) -> i32 {
    // directed graph, 2 edges between each node, each node is a person
    // each directed edge is the happiness gained/loss to that person from which
    // the arrow is coming out of

    // parsing input
    let input = String::from(input.trim())
        .replace(".", "")
        .replace("\r", "");
    let lines: Vec<&str> = input.split("\n").collect();

    let mut names1 = Vec::new();
    let mut names2 = Vec::new();
    let mut scores = Vec::new();

    for l in lines {
        let parts: Vec<&str> = l.split(" ").collect();
        let n = parts.len();

        assert!(parts.len() == 11);

        let name1 = parts[0];
        let name2 = parts[n - 1];

        let mut score = parts[3].parse::<i32>().unwrap();
        if parts[2] == "lose" {
            score = -score;
        }

        names1.push(name1);
        names2.push(name2);
        scores.push(score);
    }

    // construct lookup
    let mut lookup: HashMap<String, usize> = HashMap::new();
    let mut counter = 0;
    for (n1, n2) in names1.iter().zip(names2.iter()) {
        if !lookup.contains_key(*n1) {
            lookup.insert(String::from(*n1), counter);
            counter += 1;
        }
        if !lookup.contains_key(*n2) {
            lookup.insert(String::from(*n2), counter);
            counter += 1;
        }
    }

    // construct the 2d matrix for the graph
    let n = lookup.keys().len();
    let mut m: Vec<Vec<i32>> = vec![vec![0; n]; n];

    for ((n1, n2), score) in names1.iter().zip(names2.iter()).zip(scores.iter()) {
        let i1 = lookup.get(*n1).unwrap();
        let i2 = lookup.get(*n2).unwrap();

        m[*i1][*i2] = *score;
    }

    // brute force solution
    let mut max_gained_happiness = 0;

    let keys: Vec<&String> = lookup.keys().collect();
    for curr_names_perm in keys.iter().permutations(keys.len()) {
        let mut curr_gained_happiness = 0;
        let n = curr_names_perm.len();
        for i in 0..(curr_names_perm.len() - 1) {
            let curr_name = curr_names_perm[i];
            let next_name = curr_names_perm[i + 1];
            curr_gained_happiness += get_edge(&m, &lookup, curr_name, next_name);

            if i == 0 {
                let last_name = curr_names_perm[n - 1];
                curr_gained_happiness += get_edge(&m, &lookup, curr_name, last_name);
            }
        }
        if curr_gained_happiness > max_gained_happiness {
            max_gained_happiness = curr_gained_happiness;
        }
    }

    max_gained_happiness
}

pub fn day_13(input: &str) {
    println!("2015.13 Part 1: {}", just_do_it(input));

    let part_2_injection = "\
Alice would gain 0 happiness units by sitting next to YOU.
Bob would gain 0 happiness units by sitting next to YOU.
Carol would gain 0 happiness units by sitting next to YOU.
David would gain 0 happiness units by sitting next to YOU.
Eric would gain 0 happiness units by sitting next to YOU.
Frank would gain 0 happiness units by sitting next to YOU.
George would gain 0 happiness units by sitting next to YOU.
Mallory would gain 0 happiness units by sitting next to YOU.
YOU would gain 0 happiness units by sitting next to Alice.
YOU would gain 0 happiness units by sitting next to Bob.
YOU would gain 0 happiness units by sitting next to Carol.
YOU would gain 0 happiness units by sitting next to Eric.
YOU would gain 0 happiness units by sitting next to Frank.
YOU would gain 0 happiness units by sitting next to George.
YOU would gain 0 happiness units by sitting next to Mallory.
    ";

    let input = format!("{}\n{}", input, part_2_injection);
    println!("2015.13 Part 2: {}", just_do_it(input.as_str()));
}
