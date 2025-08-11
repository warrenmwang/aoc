fn increment_string(input: String) -> String {
    fn increment_char(c: char) -> (char, bool) {
        // all chars will be lowercase latin chars
        // ascii range: [97, 122]
        // increments char c, returns that
        // and also returns a bool indicating if we have
        // wrapped around (true = yes, false = no)
        let c = c as u8 + 1;
        if c > 122 {
            ('a', true)
        } else {
            (c as char, false)
        }
    }

    let mut chars: Vec<char> = input.chars().collect();
    let n = chars.len();
    let mut ptr: usize = n - 1;

    let (mut new_char, mut carry_over) = increment_char(chars[ptr]);
    chars[ptr] = new_char;
    ptr -= 1;

    while carry_over {
        (new_char, carry_over) = increment_char(chars[ptr]);
        chars[ptr] = new_char;
        if ptr > 0 {
            ptr -= 1;
        } else {
            break;
        }
    }

    chars.iter().collect::<String>()
}

fn is_increasing_three_straight(c2: char, c1: char, c0: char) -> bool {
    c2 as usize + 1 == c1 as usize && c1 as usize + 1 == c0 as usize
}

// input should be exactly 8 chars long
fn check_password(input: String) -> bool {
    let mut one_three_straight = false;
    let mut pair_count = 0;
    let mut taken_pair_endings: Vec<usize> = Vec::new();

    let chars: Vec<char> = input.chars().collect();

    for i in 0..chars.len() {
        let c = chars[i];

        if c == 'i' || c == 'o' || c == 'l' {
            return false;
        }

        if i == 0 {
            continue;
        }

        // count non-overlapping pairs
        if c == chars[i - 1] && !taken_pair_endings.contains(&(i - 1)) {
            taken_pair_endings.push(i);
            pair_count += 1;
        }

        // check for increasing triple straight
        if i >= 2 && is_increasing_three_straight(chars[i - 2], chars[i - 1], chars[i]) {
            one_three_straight = true;
        }
    }

    one_three_straight && pair_count >= 2
}

fn get_next_password(curr_password: String) -> String {
    let mut input = increment_string(curr_password);
    while !check_password(input.clone()) {
        input = increment_string(input);
    }
    input
}

pub fn day_11(input: &str) {
    let mut input = input.trim().to_string();
    input = get_next_password(input);
    println!("2015.11 Part 1: {}", input);

    input = get_next_password(input);
    println!("2015.11 Part 2: {}", input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn foo() {
        let mut input = String::from("aa");
        input = increment_string(input);
        assert_eq!(input, String::from("ab"));
    }

    #[test]
    fn foo1() {
        let mut input = String::from("az");
        input = increment_string(input);
        assert_eq!(input, String::from("ba"));
    }

    #[test]
    fn foo2() {
        let mut input = String::from("azz");
        input = increment_string(input);
        assert_eq!(input, String::from("baa"));
    }

    #[test]
    fn foo3() {
        let mut input = String::from("zz");
        input = increment_string(input);
        assert_eq!(input, String::from("aa"));
    }

    #[test]
    fn fail_check_password() {
        assert_eq!(check_password(String::from("hijklmmn")), false);
        assert_eq!(check_password(String::from("abbceffg")), false);
        assert_eq!(check_password(String::from("abbcegjk")), false);
    }

    #[test]
    fn pass_check_password() {
        assert_eq!(check_password(String::from("abcdffaa")), true);
        assert_eq!(check_password(String::from("ghjaabcc")), true);
    }

    #[test]
    fn next_password_1() {
        assert_eq!(
            get_next_password(String::from("abcdefgh")),
            String::from("abcdffaa")
        );
        assert_eq!(
            get_next_password(String::from("ghijklmn")),
            String::from("ghjaabcc")
        );
    }
}
