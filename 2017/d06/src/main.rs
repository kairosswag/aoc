use std::collections::HashMap;

fn main() {
    let input = include_str!("../input/input");
    let mut lines: Vec<i32> = input.split_whitespace().flat_map(|s| s.trim().parse()).collect();
    //lines = vec![0, 2, 7, 0];
    let mut set = HashMap::new();
    let mut curr_status = serialize(&lines);
    let mut step = 0;
    while !set.contains_key(&curr_status) {
        set.insert(curr_status.clone(), step);
        step += 1;
        let max = first_max(&lines);
        redistribute(&mut lines, max);
        curr_status = serialize(&lines);
        //println!("#{}", curr_status);
    }
    let found = set.get(&curr_status).unwrap();
    println!("Finished after {} steps, loop size is {}", step, step - found);

}

fn serialize(array : &[i32]) -> String {
    array.iter().map(|n| n.to_string() + ",").collect()
}

fn redistribute(array : &mut [i32], idx : usize) {
    let mut to_redist = array[idx];
    array[idx] = 0;
    let mut curr_idx = (idx + 1) % array.len();
    while to_redist > 0 {
        array[curr_idx] += 1;
        to_redist -= 1;
        curr_idx = (curr_idx + 1) % array.len();
    }
}

fn first_max(array : &[i32]) -> usize {
    let mut max : i32 = 0;
    let mut idx = 0;
    for i in 0..array.len() {
        let number = array[i];
        if number > max {
            max = number;
            idx = i;
        }
    }
    idx
}
