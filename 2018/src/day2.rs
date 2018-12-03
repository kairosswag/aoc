use std::collections::HashMap;


#[aoc(day2, part1)]
pub fn part1(input: &str) -> usize {
    let (a, b) = input.lines().map(|line| {
        let mut map = HashMap::new();
        for item in line.chars() {
            let counter = map.entry(item).or_insert(0);
            *counter += 1;
        }
        let twos = if map.values().any(|val| *val == 2) {1} else {0};
        let threes = if map.values().any(|val| *val == 3) {1} else {0};
        (twos, threes)
    }).fold((0, 0), |(twos, threes), (acc_twos, acc_threes)| (acc_twos + twos, acc_threes + threes));
    a * b
}

#[aoc(day2, part2)]
pub fn part2(input: &str) -> String {
    let lines : Vec<&str> = input.lines().collect();
    for first in 0..lines.len() {
        for second in 0..lines.len() {
            if first != second {
                if lines[first].chars().zip(lines[second].chars()).filter(|(f, s)| f != s).count() == 1 {
                    return lines[first].chars().zip(lines[second].chars()).filter(|(f, s)| f == s).map(|(f, _s)| f).collect();
                }                
            }
        }
    }
    return String::from("no result found");
}