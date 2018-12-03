extern crate regex;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Claim {
    id: u32,
    xpos: u32,
    ypos: u32,
    width: u32,
    height: u32,
}

#[aoc_generator(day3)]
pub fn day3_input_gerator(input: &str) -> Vec<Claim> {
    let re = regex::Regex::new(r"#(\d*) @ (\d*),(\d*): (\d*)x(\d*)").unwrap();
    let mut res = Vec::new();
    for line in input.lines() {
        for cap in re.captures_iter(line) {
            res.push(Claim {
                id: cap[1].parse::<u32>().expect(&format!("id: {}", &cap[0])),
                xpos: cap[2].parse::<u32>().expect(&format!("xpos: {}", &cap[0])),
                ypos: cap[3].parse::<u32>().expect(&format!("ypos: {}", &cap[0])),
                width: cap[4].parse::<u32>().expect(&format!("width: {}", &cap[0])),
                height: cap[5].parse::<u32>().expect(&format!("height: {}", &cap[0]))   
            });
        }
    }
    res
}

#[aoc(day3, part1)]
pub fn part1(input: &[Claim]) -> usize {
    let mut overlaps = HashMap::new();
    for claim in input {
        for xc in 0..claim.width {
            for yc in 0..claim.height {
                let count = overlaps.entry((claim.xpos + xc, claim.ypos + yc)).or_insert(0);
                *count += 1;
            }
        }
    }
    overlaps.values().filter(|val| *val >= &2).count()
}

#[aoc(day3, part2)]
pub fn part2(input: &[Claim]) -> u32 {
    let mut overlaps = HashMap::new();
    for claim in input {
        for xc in 0..claim.width {
            for yc in 0..claim.height {
                let count = overlaps.entry((claim.xpos + xc, claim.ypos + yc)).or_insert(0);
                *count += 1;
            }
        }
    }
    'outer: for claim in input {
        for xc in 0..claim.width {
            for yc in 0..claim.height {
                if let Some(val) = overlaps.get(&(claim.xpos + xc, claim.ypos + yc)) {
                    if *val != 1 {
                        continue 'outer;
                    }
                }
            }
        }
        return claim.id;
    }
    0
}