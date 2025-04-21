use std::fs;

fn is_num(i: usize, chars: &Vec<char>) -> bool {
    let c = chars[i] as u8;

    // minus sign for negative number
    if c == 45 {
        // ensure its modifying a number
        if i + 1 < chars.len() {
            let c1 = chars[i + 1] as u8;
            return c1 >= 48 && c1 <= 57;
        }
        return false;
    }

    c >= 48 && c <= 57
}

fn parse_nums(input: String) -> i32 {
    let mut res = 0;
    let chars: Vec<_> = input.chars().collect();
    let mut start_ind: i32 = -1;

    for i in 0..chars.clone().len() {
        // start of a number
        if start_ind == -1 && is_num(i, &chars) {
            start_ind = i as i32;
            continue;
        }

        // end of a number
        if start_ind != -1 && !is_num(i, &chars) {
            let slice = &input[start_ind as usize..i];
            start_ind = -1;
            let num: i32 = slice.parse().unwrap();
            res += num;
        }
    }

    res
}

fn part_1(input: String) {
    // let input = String::from("[123,2, -10, 0]"); // 115

    // use a json lexer and then loop over all of the pieces and
    // sum up the numbers...OR...
    // go on the path of least resistance, just loop thru all the chars and find
    // the numbers and add them together
    let res = parse_nums(input);

    println!("{}", res);

    // 176852
    // too high
    // 156366
    // correct
}

fn part_2(input: String) {
    // TODO: no shortcuts now, it looks like I actually have to lex/parse
    // the json now and know where we are in the json structure
}

fn main() {
    let input = fs::read_to_string("input.json").expect("file not found");
    let input = String::from(input.trim());
    part_1(input);
    // part_2(input);
}
