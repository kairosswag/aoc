use std::collections::HashSet;

#[aoc_generator(day1)]
pub fn day1_input_gerator(input: &str) -> Vec<i32> {
    input.lines().map(|line| {
        let sign = if line.chars().next().unwrap() == '+' {
            1
        } else {
            -1
        };
        let number : i32 = line.get(1..).unwrap().parse::<i32>().unwrap();
        sign * number
    }).collect()
}

#[aoc(day1, part1)]
pub fn part1(input: &[i32]) -> i32 {
    input.iter().fold(0, |sum, &i| sum + i)
}

#[aoc(day1, part2)]
pub fn part2(input: &[i32]) -> i32 {
    let mut freq = HashSet::new();
    let mut curr = 0;
    freq.insert(curr);
    loop {
        for i in 0..input.len() {
            curr += input[i];
            if freq.contains(&curr) {
                return curr;
            }
            freq.insert(curr);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() {
        let sample = vec![1, 1, 1];
        assert_eq!(part1(&sample), 3);
    }

    #[test]
    fn sample2() {
        let sample = vec![1, 1, -2];
        assert_eq!(part1(&sample), 0);
    }

    #[test]
    fn sample3() {
        let sample = vec![-1, -2, -3];
        assert_eq!(part1(&sample), -6);
    }

    #[test]
    fn sample4() {
        let sample = vec![1, -1];
        assert_eq!(part2(&sample), 0);
    }

    #[test]
    fn sample5() {
        let sample = vec![3, 3, 4, -2, -4];
        assert_eq!(part2(&sample), 10);
    }

    #[test]
    fn sample6() {
        let sample = vec![-6, 3, 8, 5, -6];
        assert_eq!(part2(&sample), 5);
    }

    #[test]
    fn sample7() {
        let sample = vec![7, 7, -2, -7, -4];
        assert_eq!(part2(&sample), 14);
    }


}