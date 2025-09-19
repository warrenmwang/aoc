pub fn day_4(input: &str) {
    let mut found_5 = false;
    let mut found_6 = false;

    let mut i = 0;
    let mut digest: String;
    loop {
        digest = format!("{:x}", md5::compute(format!("{}{}", input, i)));

        if !found_5 && digest.starts_with("00000") {
            println!("2015.4 Part 1: {}", i);
            found_5 = true;
        }
        if !found_6 && digest.starts_with("000000") {
            println!("2015.4 Part 2: {}", i);
            found_6 = true;
        }
        if found_5 && found_6 {
            return;
        }

        i += 1;
    }
}
