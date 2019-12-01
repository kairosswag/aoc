use std::collections::HashSet;

#[aoc_generator(day1)]
pub fn day1_input_gerator(input: &str) -> Vec<i32> {
    input.lines().map(|line| line.parse::<i32>().unwrap()).collect()
}

#[aoc(day1, part1)]
pub fn part1(input: &[i32]) -> i32 {
    input.iter().map(|i| (i / 3) -2).fold(0, |sum, i| sum + i)
}

#[aoc(day1, part2)]
pub fn part2(input: &[i32]) -> i32 {
    input.iter().map(|i| calc_fuel(i)).fold(0, |sum, i| sum + i)
}

fn calc_fuel(inval: &i32) -> i32 {
    let mut res_val = (inval / 3) - 2;
    if res_val > 0 {
        res_val += calc_fuel(&res_val);
    }
    if res_val > 0 {
        res_val
    } else {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() {
        let sample = vec![100756];
        assert_eq!(part2(&sample), 50346);
    }

    
    #[test]
    fn sample2() {
        let sample = vec![1969];
        assert_eq!(part2(&sample), 966);
    }


}