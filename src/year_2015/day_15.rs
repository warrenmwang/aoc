struct Ingredient {
    capacity: i32,
    durability: i32,
    flavor: i32,
    texture: i32,
    calories: i32,
}

fn calc_score(
    ingredients: &Vec<Ingredient>,
    t1: i32,
    t2: i32,
    t3: i32,
    t4: i32,
    is_part_2: bool,
) -> i32 {
    let calories = ingredients[0].calories * t1
        + ingredients[1].calories * t2
        + ingredients[2].calories * t3
        + ingredients[3].calories * t4;
    if is_part_2 {
        if calories != 500 {
            return 0;
        }
    }

    let mut capacity_score = ingredients[0].capacity * t1
        + ingredients[1].capacity * t2
        + ingredients[2].capacity * t3
        + ingredients[3].capacity * t4;
    capacity_score = if capacity_score < 0 {
        0
    } else {
        capacity_score
    };

    let mut durability_score = ingredients[0].durability * t1
        + ingredients[1].durability * t2
        + ingredients[2].durability * t3
        + ingredients[3].durability * t4;
    durability_score = if durability_score < 0 {
        0
    } else {
        durability_score
    };

    let mut flavor_score = ingredients[0].flavor * t1
        + ingredients[1].flavor * t2
        + ingredients[2].flavor * t3
        + ingredients[3].flavor * t4;
    flavor_score = if flavor_score < 0 { 0 } else { flavor_score };

    let mut texture_score = ingredients[0].texture * t1
        + ingredients[1].texture * t2
        + ingredients[2].texture * t3
        + ingredients[3].texture * t4;
    texture_score = if texture_score < 0 { 0 } else { texture_score };

    capacity_score * durability_score * flavor_score * texture_score
}

pub fn day_15(input: &str) {
    let lines: Vec<&str> = input.trim().split("\n").collect();

    let mut ingredients: Vec<Ingredient> = Vec::new();
    for line in lines {
        let line = line.split(": ").collect::<Vec<&str>>()[1];
        let properties = line.split(", ").collect::<Vec<&str>>();
        let property_values: Vec<i32> = properties
            .iter()
            .map(|p| p.split(" ").collect::<Vec<&str>>()[1].parse().unwrap())
            .collect();

        assert!(property_values.len() == 5);
        ingredients.push(Ingredient {
            capacity: property_values[0],
            durability: property_values[1],
            flavor: property_values[2],
            texture: property_values[3],
            calories: property_values[4],
        });
    }
    assert!(ingredients.len() == 4);

    let mut max_score = 0;
    for t1 in 0..100 {
        for t2 in 0..100 - t1 {
            for t3 in 0..100 - t1 - t2 {
                let t4 = 100 - t1 - t2 - t3;
                let score = calc_score(&ingredients, t1, t2, t3, t4, false);
                if score > max_score {
                    max_score = score;
                }
            }
        }
    }
    println!("2015.15 Part 1: {}", max_score);

    let mut max_score = 0;
    for t1 in 0..100 {
        for t2 in 0..100 - t1 {
            for t3 in 0..100 - t1 - t2 {
                let t4 = 100 - t1 - t2 - t3;
                let score = calc_score(&ingredients, t1, t2, t3, t4, true);
                if score > max_score {
                    max_score = score;
                }
            }
        }
    }
    println!("2015.15 Part 2: {}", max_score);
}
