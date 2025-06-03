use std::{collections::HashMap, fs};

struct Sue {
    id: u16,
    properties: HashMap<String, u8>,
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let input: Vec<&str> = input.trim().split("\r\n").collect();

    let mut sues: Vec<Sue> = Vec::new();
    for line in input {
        let (sue_id_part, sue_properties_part) = line.split_once(": ").unwrap();
        let id = sue_id_part.split(" ").collect::<Vec<&str>>()[1]
            .parse::<u16>()
            .unwrap();

        let mut sue_properties: HashMap<String, u8> = HashMap::new();
        for p in sue_properties_part.split(", ") {
            let parts: Vec<&str> = p.split(": ").collect();
            let property = String::from(parts[0]);
            let value: u8 = parts[1].parse().unwrap();
            sue_properties.insert(property, value);
        }

        sues.push(Sue {
            id,
            properties: sue_properties,
        })
    }

    let mfcsam_result: HashMap<String, u8> = [
        ("children".to_string(), 3),
        ("cats".to_string(), 7),
        ("samoyeds".to_string(), 2),
        ("pomeranians".to_string(), 3),
        ("akitas".to_string(), 0),
        ("vizslas".to_string(), 0),
        ("goldfish".to_string(), 5),
        ("trees".to_string(), 3),
        ("cars".to_string(), 2),
        ("perfumes".to_string(), 1),
    ]
    .into_iter()
    .collect();

    // loop over all sues and check against the MFCSAM detected compounds.
    // denote the correct Sue by the most number of matched exact compounds.
    let mut most_match_num = 0;
    let mut best_sue_match: Option<Sue> = None;
    for sue in sues {
        let mut curr_match_num = 0;
        sue.properties.iter().for_each(|(p, v)| {
            if let Some(mfcsam_value) = mfcsam_result.get(p) {
                if p == "cats" || p == "trees" {
                    if v > mfcsam_value {
                        curr_match_num += 1;
                    }
                } else if p == "pomeranians" || p == "goldfish" {
                    if v < mfcsam_value {
                        curr_match_num += 1;
                    }
                } else if mfcsam_value == v {
                    curr_match_num += 1;
                }
            }
        });
        if curr_match_num > most_match_num {
            most_match_num = curr_match_num;
            best_sue_match = Some(sue);
        }
    }
    if let Some(sue) = best_sue_match {
        println!("Sue {} got us the gift!", sue.id);
    } else {
        println!("Oh no! We couldn't find a matching Sue...");
    }

    // part1: sue 40
    // part2: sue 241
}
