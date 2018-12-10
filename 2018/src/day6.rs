extern crate regex;
use std::collections::HashMap;

#[derive(new, Ord, Eq, PartialEq, PartialOrd)]
pub struct Coord {
    pub num: usize,
    pub x: usize,
    pub y: usize,
}

pub enum XDirection {
    Left, Right,
}

pub enum YDirection {
    Up, Down,
}

impl Coord {

    pub fn dist(&self, other: &Coord) -> usize {
        let xdiff = other.x as i32 - self.x as i32;
        let ydiff = other.y as i32 - self.y as i32;
        (xdiff.abs() + ydiff.abs()) as usize
    }

    pub fn hashval(&self) -> usize {
        self.x*1000 + self.y
    }
}

#[aoc_generator(day6)]
pub fn day6_input_generator(input: &str) -> Vec<Coord> {
    let re = regex::Regex::new(r"(\d*)..(\d*)").unwrap();
    input.lines().zip(1..).map(|(line, count)| {
        for cap in re.captures_iter(line) {
            return Coord::new(count, cap[1].parse::<usize>().unwrap(), cap[2].parse::<usize>().unwrap());
        }
        panic!("Could not parse");
    }).collect()
}

#[aoc(day6, part1)]
pub fn part1(input: &[Coord]) -> usize {
    let max_x = input.iter().map(|co| co.x).max().unwrap();
    let min_x = input.iter().map(|co| co.x).min().unwrap();
    let max_y = input.iter().map(|co| co.y).max().unwrap();
    let min_y = input.iter().map(|co| co.y).min().unwrap();

    let mut voronoi = vec![vec![0; max_y as usize + 1]; max_x as usize + 1];
    for x in 0 .. voronoi.len() {
        for y in 0 .. voronoi[x as usize].len() {
            let mut min_dist = std::usize::MAX;
            let mut curr_min = 0;
            let mut doubled = false;
            for coo in input {
                let dist = Coord::new(0, x, y).dist(coo);
                if dist < min_dist {
                    min_dist = dist;
                    curr_min = coo.num;
                    doubled = false;
                } else if dist == min_dist {
                    doubled = true;
                }
            }
            curr_min = if doubled {0} else {curr_min};
            voronoi[x as usize][y as usize] = curr_min;
        }
    }
    let mut res = vec![0; input.len() + 1];
    
    voronoi.iter().flatten().for_each(|n| res[*n as usize] += 1);
    for i in 0..voronoi.len() {
        let n = voronoi[i][0];
        res[n] = 0;
        let m = voronoi[i][max_y];
        res[m] = 0;
    }
    for i in 0..voronoi[0].len() {
        let n = voronoi[0][i];
        res[n] = 0;
        let m = voronoi[max_x][i];
        res[m] = 0;
    }
    *res.iter().skip(1).max().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() {
        let input = "1, 1
1, 6
8, 3
3, 4
5, 5
8, 9";
        let gen = day6_input_generator(&input);
        assert_eq!(part1(&gen), 17);
    }
}