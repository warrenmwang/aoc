fn look_and_say(s: String) -> String {
    let mut count = 0;
    let mut res = String::new();

    let mut prev_seen_num: i32 = -1;

    for (i, c) in s.chars().into_iter().enumerate() {
        let num = c.to_digit(10).unwrap() as i32;

        if i == 0 {
            count = 1;
            prev_seen_num = num;
            continue;
        }

        if num == prev_seen_num {
            count += 1;
        } else {
            res.push_str(&format!("{}{}", count, prev_seen_num));
            prev_seen_num = num;
            count = 1;
        }
    }

    res.push_str(&format!("{}{}", count, prev_seen_num));

    res
}

pub fn day_10(input: &str) {
    let mut res = String::from(input);
    let mut part_1_res: String = String::new();

    for i in 0..50 {
        res = look_and_say(res);
        if i == 39 {
            part_1_res = res.clone();
        }
    }

    println!("Part 1: {}", part_1_res.len());
    println!("Part 2: {}", res.len());
}
