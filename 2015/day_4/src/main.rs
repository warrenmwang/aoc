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

fn main() {
    let input = "ckczppom";
    assert_eq!(mine(input, 5), 117946);
    assert_eq!(mine(input, 6), 3938038);
}
