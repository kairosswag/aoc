use crate::structures::*;

#[aoc_generator(day1)]
pub fn day1_input_gerator(input: &str) -> Vec<i32> {
    input.lines().map(|line| line.parse::<i32>().unwrap()).collect()
}

#[aoc_generator(day2)]
pub fn day2_input_gerator(input: &str) -> Vec<i32> {
    input
        .split(',')
        .map(|i| i.parse::<i32>().unwrap())
        .collect()
}

// to whoever reads that: sorry :)
#[aoc_generator(day3)]
pub fn parse_directions(input: &str) -> (Vec<Direction>, Vec<Direction>) {
    let mut lines = input.lines();
    let a = lines
        .next()
        .unwrap()
        .split(',')
        .map(|d| d.chars().collect())
        .map(|arr: Vec<char>| {
            match (
                arr[0],
                arr[1..]
                    .into_iter()
                    .collect::<String>()
                    .parse::<u32>()
                    .unwrap(),
            ) {
                ('U', x) => Direction::UP(x),
                ('D', x) => Direction::DOWN(x),
                ('L', x) => Direction::LEFT(x),
                ('R', x) => Direction::RIGHT(x),
                _ => panic!("nah"),
            }
        })
        .collect();
    let b = lines
        .next()
        .unwrap()
        .split(',')
        .map(|d| d.chars().collect())
        .map(|arr: Vec<char>| {
            match (
                arr[0],
                arr[1..]
                    .into_iter()
                    .collect::<String>()
                    .parse::<u32>()
                    .unwrap(),
            ) {
                ('U', x) => Direction::UP(x),
                ('D', x) => Direction::DOWN(x),
                ('L', x) => Direction::LEFT(x),
                ('R', x) => Direction::RIGHT(x),
                _ => panic!("nah"),
            }
        })
        .collect();
    (a, b)
}

#[aoc_generator(day4)]
pub fn day4_generator(_input: &str) -> (u32, u32) {
    (136818, 685979)
}

#[aoc_generator(day5)]
pub fn day5_gerator(input: &str) -> Vec<i32> {
    input
        .split(',')
        .map(|i| i.parse::<i32>().unwrap())
        .collect()
}
