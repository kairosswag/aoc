use std::collections::HashSet;

fn main() {
    let input = include_str!("../input/input");
    let lines = input.lines();
    let mut accumulator = 0;
    for line in lines {
        let mut set = HashSet::new();
        let mut found = false;
        'inner: for word in line.split_whitespace() {
            let mut chars : Vec<char> = word.trim().chars().collect();
            //chars.sort();
            let sorted_word : String = chars.iter().collect();
            if set.contains(&sorted_word) {
                found = true;
                break 'inner;
            } else {
                set.insert(sorted_word);
            }
        }
        if !found {
            accumulator += 1;
        }
    }
    println!("Result1 found {}", accumulator);
}
