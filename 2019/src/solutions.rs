

use std::cmp::*;
use crate::structures::*;
use crate::generators::*;

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
    for (_idx, dir) in b.iter().enumerate() {
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

#[aoc(day4, part1)]
pub fn day4_1(input: &(u32, u32)) -> u32 {
    let &(low, high) = input;
    let mut counter = 0;
    'outer: for i in low ..= high {
        if check_input(i) {
            counter += 1;
        }
    }
    counter
}

pub fn check_input(i: u32) -> bool {
    // take the last pair, then cut off the next value
    let mut curr_val = i;
    let mut found_double = false;
    for _val in 0 .. 5 {
        let pair = curr_val % 100;
        curr_val = curr_val / 10;

        // break if it is not ascending
        if pair / 10 > pair % 10 { 
            return false;
        }

        if pair / 10 == pair % 10 {
            found_double = true;
        }

    }

    found_double
}

#[aoc(day4, part2)]
pub fn day4_2(input: &(u32, u32)) -> u32 {
    let &(low, high) = input;
    let mut counter = 0;
    for i in low ..= high {
        if check_input_2(i) {
            counter += 1;
        }
    }
    counter
}

pub fn check_input_2(i: u32) -> bool {
    // take the last pair, then cut off the next value
    let mut curr_val = i;
    let mut double_count = 0;
    let mut found_true_double = false;
    for _val in 0 .. 5 {
        let pair = curr_val % 100;
        curr_val = curr_val / 10;

        // break if it is not ascending
        if pair / 10 > pair % 10 { 
            return false;
        }

        match (pair / 10 == pair % 10, double_count) {
            (false, x) if x == 1 => found_true_double = true, // true double found, we can disregard all match
            (false, _) => double_count = 0,
            (true, _) => double_count += 1,
        }

    }

    (double_count == 1) || found_true_double
}

#[aoc(day5, part1)]
pub fn day5_1(input: &[i32]) -> i32 {
    day5(input, 1)
}

#[aoc(day5, part2)]
pub fn day5_2(input: &[i32]) -> i32 {
    day5(input, 5)
}

fn day5(input: &[i32], g_input: i32) -> i32 {
    let mut data = input.to_vec();
    let mut sig_end = false;
    let mut pos = 0;
    let mut output = 0;
    while !sig_end {
        let instruction = Instruction::from_slice(&data[pos..]);
        let (res_out, res_pos, res_sig_end) = instruction.execute(&mut data, pos, g_input);
        pos = res_pos;
        sig_end = res_sig_end;
        
        if !sig_end && output != 0 { // check everything is okay
            panic!("Err: Output was: {}", output);
        } else if !sig_end { // don't override the last output
            output = res_out;
        }

    }
    output
}
