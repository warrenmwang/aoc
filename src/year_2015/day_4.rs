fn mine(input: &str, num_leading_zeros: usize) -> i32 {
    let desired_leading = "0".repeat(num_leading_zeros);
    let mut i = 0;
    let mut digest: String;
    loop {
        digest = format!("{:x}", md5::compute(format!("{}{}", input, i)));
        if digest.starts_with(&desired_leading) {
            return i;
        }
        i += 1;
    }
}

pub fn day_4(input: &str) {
    println!("2015.4 Part 1: {}", mine(input, 5));
    println!("2015.4 Part 2: {}", mine(input, 6));
}
