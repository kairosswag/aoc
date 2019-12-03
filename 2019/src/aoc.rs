use std::collections::HashSet;
use std::cmp::*;

#[aoc_generator(day1)]
pub fn day1_input_gerator(input: &str) -> Vec<i32> {
    input
        .lines()
        .map(|line| line.parse::<i32>().unwrap())
        .collect()
}

#[aoc(day1, part1)]
pub fn part1(input: &[i32]) -> i32 {
    input.iter().map(|i| (i / 3) - 2).fold(0, |sum, i| sum + i)
}

#[aoc(day1, part2)]
pub fn part2(input: &[i32]) -> i32 {
    input.iter().map(|i| calc_fuel(i)).fold(0, |sum, i| sum + i)
}

#[aoc_generator(day2)]
pub fn day2_input_gerator(input: &str) -> Vec<i32> {
    input
        .split(',')
        .map(|i| i.parse::<i32>().unwrap())
        .collect()
}

#[aoc(day2, part1)]
pub fn day2_1(input: &[i32]) -> i32 {
    let mut arr = input.to_vec();
    arr[1] = 12;
    arr[2] = 2;
    let mut idx = 0;
    while arr[idx] != 99 {
        println!("On pos {} with value {}", idx, arr[idx]);
        let res_pos = arr[idx + 3] as usize;
        let arg_1 = arr[idx + 2] as usize;
        let arg_2 = arr[idx + 1] as usize;
        if arr[idx] == 1 {
            arr[res_pos] = arr[arg_1] + arr[arg_2];
        } else if arr[idx] == 2 {
            arr[res_pos] = arr[arg_1] * arr[arg_2];
        } else {
            panic!("Nah");
        }
        idx += 4;
    }
    arr[0]
}

#[aoc(day2, part2)]
pub fn day2_2(input: &[i32]) -> i32 {
    let mut found = false;
    let mut noun = 0;
    let mut verb = -1;
    while !found {
        let mut arr = input.clone().to_vec();
        if noun == verb {
            noun += 1;
            verb = 0;
        } else {
            verb += 1;
        }
        arr[1] = noun;
        arr[2] = verb;
        let mut idx = 0;
        while arr[idx] != 99 {
            let res_pos = arr[idx + 3] as usize;
            let arg_1 = arr[idx + 2] as usize;
            let arg_2 = arr[idx + 1] as usize;
            if arr[idx] == 1 {
                arr[res_pos] = arr[arg_1] + arr[arg_2];
            } else if arr[idx] == 2 {
                arr[res_pos] = arr[arg_1] * arr[arg_2];
            } else {
                panic!("Nah");
            }
            idx += 4;
        }
        if arr[0] == 19690720 {
            found = true;
        }
    }
    100 * noun + verb
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

#[derive(Debug)]
pub enum Direction {
    UP(u32),
    DOWN(u32),
    LEFT(u32),
    RIGHT(u32),
}

impl Direction {
    pub fn transform(&self, arg: &Pos) -> Pos {
        match &self {
            Direction::UP(val) => Pos::new(arg.x, arg.y + *val as i32),
            Direction::DOWN(val) => Pos::new(arg.x, arg.y - *val as i32),
            Direction::LEFT(val) => Pos::new(arg.x - *val as i32, arg.y),
            Direction::RIGHT(val) => Pos::new(arg.x + *val as i32, arg.y),
        }
    }
}

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub struct Line {
    p_0: Pos,
    p_1: Pos,
}

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub struct Pos {
    x: i32,
    y: i32,
}

impl Pos {
    pub fn new(x: i32, y: i32) -> Pos {
        Pos { x, y }
    }

    // manhattan distance
    pub fn distance_to(&self, other: &Pos) -> u32 {
        ((self.x.abs() - other.x.abs()).abs() + (self.y.abs() - other.y.abs()).abs()) as u32
    }
}

impl Line {
    pub fn new(a: Pos, b: Pos) -> Line {
        // Line is either horizontal or vertial, horizontal points are ordered l2r, vertical t2b
        if a.x == b.x {
            if a.y > b.y {
                return Line { p_0: b, p_1: a };
            } else {
                return Line { p_0: a, p_1: b };
            }
        } else {
            if a.x > b.x {
                return Line { p_0: a, p_1: b };
            } else {
                return Line { p_0: b, p_1: a };
            }
        }
    }

    pub fn is_horizontal(&self) -> bool {
        self.p_0.y == self.p_1.y
    }

    pub fn intersect(&self, other: &Line) -> Option<Pos> {
        match (self.is_horizontal(), other.is_horizontal()) {
            (false, true) => {
                if self.p_0.y < other.p_0.y
                    && self.p_1.y > other.p_0.y
                    && self.p_0.x < other.p_0.x
                    && self.p_0.x > other.p_1.x
                {
                    Some(Pos::new(self.p_0.x, other.p_0.y))
                } else {
                    None
                }
            }, 
            (true, false) => {
                if other.p_0.y < self.p_0.y
                && other.p_1.y > self.p_0.y
                && other.p_0.x < self.p_0.x
                && other.p_0.x > self.p_1.x
                {
                    Some(Pos::new(other.p_0.x, self.p_0.y))
                } else {
                    None
                }
            },
            _ => None,
        }
    }

    pub fn get_steps(&self) -> u32 {
        if self.is_horizontal() {
            (max(self.p_0.x, self.p_1.x) - min(self.p_0.x, self.p_1.x)) as u32
        } else {
            (max(self.p_0.y, self.p_1.y) - min(self.p_0.y, self.p_1.y)) as u32
        }
    }
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
            match ((
                arr[0],
                arr[1..]
                    .into_iter()
                    .collect::<String>()
                    .parse::<u32>()
                    .unwrap(),
            )) {
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
            match ((
                arr[0],
                arr[1..]
                    .into_iter()
                    .collect::<String>()
                    .parse::<u32>()
                    .unwrap(),
            )) {
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

#[aoc(day3, part1)]
pub fn day3_1(input: &(Vec<Direction>, Vec<Direction>)) -> u32 {
    let (a, b) = input;
    let mut line_soup_a = Vec::new();
    let mut curr_pos = Pos::new(0, 0);
    for dir in a {
        let next = dir.transform(&curr_pos);
        line_soup_a.push(Line::new(curr_pos, next));
        curr_pos = next;
    }
    let mut curr_pos = Pos::new(0, 0);
    let mut curr_closest = None;
    for dir in b {
        let next = dir.transform(&curr_pos);
        let b_line = Line::new(curr_pos, next);
        for a_line in &line_soup_a {
            if let Some(pos) = a_line.intersect(&b_line) {
                let nval = pos.x.abs() as u32 + pos.y.abs() as u32;
                if let Some(val) = curr_closest {
                    curr_closest = Some(min(val, nval));
                } else {
                    curr_closest = Some(nval);
                }
            }
        }
        curr_pos = next;
    }
    curr_closest.unwrap()
}

#[aoc(day3, part2)]
pub fn day3_2(input: &(Vec<Direction>, Vec<Direction>)) -> u32 {
    let (a, b) = input;
    let mut line_soup_a = Vec::new();
    let mut curr_pos = Pos::new(0, 0);
    let mut steps = 0;
    for dir in a {
        let next = dir.transform(&curr_pos);
        let line = Line::new(curr_pos, next);
        line_soup_a.push((line, steps, curr_pos));
        steps += line.get_steps();
        curr_pos = next;
    }
    let mut curr_pos = Pos::new(0, 0);
    let mut distances = Vec::new();
    let mut steps = 0;
    for (idx, dir) in b.iter().enumerate() {
        let next = dir.transform(&curr_pos);
        let b_line = Line::new(curr_pos, next);
        for (a_line, a_steps, a_start_pos) in &line_soup_a {
            if let Some(intersection) = a_line.intersect(&b_line) {
                let full_steps_a = a_steps + a_start_pos.distance_to(&intersection);
                let full_steps_b = steps + curr_pos.distance_to(&intersection); 
                distances.push(full_steps_a + full_steps_b)
            }
        }
        steps += b_line.get_steps();
        curr_pos = next;
    }
    *distances.iter().min().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let sample = vec![1, 0, 0, 0, 99];
        assert_eq!(day2_1(&sample), 2);
    }

    #[test]
    fn test2() {
        let sample = vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50];
        assert_eq!(day2_1(&sample), 3500);
    }

    #[test]
    fn test3() {
        let hor = Line::new(Pos::new(0, 5), Pos::new(0, -5));
        let ver = Line::new(Pos::new(-5, 0), Pos::new(5, 0));
        let ver2 = Line::new(Pos::new(0, 5), Pos::new(0, 0));
        assert_eq!(hor.intersect(&ver), Some(Pos::new(0, 0)));
        assert_eq!(hor.intersect(&ver2), None); 
    }
}
