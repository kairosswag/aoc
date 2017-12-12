extern crate regex;
use regex::*;
use std::collections::HashMap;
use std::collections::HashSet;

fn main() {
    let input = include_str!("../input/input");
    let re = Regex::new(r"([0-9]*)[^0-9]*").unwrap();
    let mut map = HashMap::new();
    for line in input.lines() {
        let mut numbers: HashSet<u32> = HashSet::new();
        let mut is_first = true;
        let mut first = 0;
        for capture in re.captures_iter(line) {
            if is_first {
                first = capture[1].parse().unwrap();
                is_first = false;
            }
            numbers.insert(capture[1].parse().unwrap());
        }
        map.insert(first, numbers);
    }

    create_full_set(&mut map, &0);
    let mut groups = HashSet::new();
    groups.insert(0);
    loop {
        let number = {
            if let Some(number) = map.keys().filter(|key| !groups.contains(key)).next() {
                number
            } else {
                break;
            }
        }.clone();
        create_full_set(&mut map, &number);
        groups.insert(number);
    }
    println!("Number of groups: {}", groups.len());
}

fn create_full_set(map: &mut HashMap<u32, HashSet<u32>>, group: &u32) {
    let mut curr_set = map.get(group).unwrap().clone();
    let mut checked = HashSet::new();
    loop {
        let number = {
                let mut symm_diff = curr_set.symmetric_difference(&checked);
                if let Some(number) = symm_diff.next() {
                    number
                } else {
                    break; // If all numbers are run through, nothing needs to be checked anymore.
                }
            }
            .clone();
        {
            let n_set = map.get(&number).unwrap();
            checked.insert(number);
            curr_set = curr_set.union(&n_set).map(|number| *number).collect();
        }
        map.remove(&number);
    }
    //println!("Last set is {:?}, len: {}", curr_set, curr_set.len());
}
