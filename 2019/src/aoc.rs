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

#[aoc_generator(day2)]
pub fn day2_input_gerator(input: &str) -> Vec<i32> {
    input.split(',').map(|i| i.parse::<i32>().unwrap()).collect()
}

#[aoc(day2, part1)]
pub fn day2_1(input: &[i32]) -> i32 {
    let mut arr = input.to_vec();
    arr[1] = 12;
    arr[2] = 2;
    let mut idx = 0;
    while arr[idx] != 99 {
        println!("On pos {} with value {}", idx, arr[idx]);
        let res_pos = arr[idx+3] as usize;
        let arg_1 = arr[idx+2] as usize;
        let arg_2 = arr[idx+1] as usize;
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
            let res_pos = arr[idx+3] as usize;
            let arg_1 = arr[idx+2] as usize;
            let arg_2 = arr[idx+1] as usize;
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let sample = vec![1,0,0,0,99];
        assert_eq!(day2_1(&sample), 2);        
    }

    #[test]
    fn test2() {
        let sample = vec![1,9,10,3,2,3,11,0,99,30,40,50];
        assert_eq!(day2_1(&sample), 3500);        
    }

}