extern crate regex;
use regex::*;
use std::collections::HashSet;
use std::collections::HashMap;

#[derive(Debug)]
struct Program {
    name : String,
    amount : i32,
    followers : Vec<String>,
}

#[derive(Debug)]
enum TotalWeightOrException<A, B> {
    Weight(A),
    Name(B, A),
}

fn main() {
    let input = include_str!("../input/input");
    let programs = parse(input);

    let mut eliminated : HashSet<String> = HashSet::new();
    programs.iter().flat_map(|p| &p.followers).for_each(|s| {eliminated.insert(s.clone()); ()});

    println!("#elim {} of #total {}", eliminated.len(), programs.len());

    let base_program : Vec<&Program> = programs.iter().filter(|p| !eliminated.contains(&p.name)).collect();
    let found = &base_program[0].name;
    for program in base_program {
        println!("Found: {:?}", program);
    }

    let hmap : HashMap<&str, &Program> = programs.iter().map(|p| &(*p.name)).zip(programs.iter()).collect();
    let res = balance_rec(found, &hmap);
    println!("Res: {:?}", res);
    
}

fn balance_rec(curr_program : &str, programs : &HashMap<&str, &Program>) -> TotalWeightOrException<i32, String> {
    let curr_prog = programs.get(curr_program).expect("could not find");
    let children = &curr_prog.followers;
    
    
    if children.len() == 0 { // Shortcut for the uppermost nodes.
        return TotalWeightOrException::Weight(curr_prog.amount);
    }


    let mut children_weight = Vec::new();
    for i in  0..children.len() {
        let result = balance_rec(&children[i], &programs);
        match result {
            TotalWeightOrException::Weight(weight) => children_weight.push(weight),
            TotalWeightOrException::Name(_, _) => return result,
        }
    }

    if children.len() <= 2 {
        return TotalWeightOrException::Weight(curr_prog.amount + children_weight.iter().fold(0, |a, b| a + b));
    }

    let &max = children_weight.iter().max().unwrap();
    let &min = children_weight.iter().min().unwrap();

    if max != min {
        let mut found_max_idx = 0;
        let mut found_min_idx = 0;
        let mut found_max_times = 0;
        let mut found_min_times = 0;
        let mut right_value = 0;
        let mut wrong_value = 0;
        for i in 0..children_weight.len() {
            let weight = children_weight[i];
            if weight == max {
                found_max_idx = i;
                found_max_times += 1;
            } else if weight == min {
                found_min_idx = i;
                found_min_times += 1;
            }
        }

        let mut wrong_idx = 0;
        if found_max_times == 1 {
            wrong_idx = found_max_idx;
            right_value = min;
            wrong_value = max;
        } else if found_min_times == 1 {
            wrong_idx = found_min_idx;
            right_value = max;
            wrong_value = min;
        }
        let wrong_program : &str= &(children[wrong_idx]);
        let wrong_sub_program = programs.get(wrong_program).unwrap();

        let value_diff = (wrong_value - right_value).abs();

        let correct_value = wrong_sub_program.amount - value_diff;

        return TotalWeightOrException::Name(wrong_sub_program.name.clone(), correct_value);

    } else {
        return TotalWeightOrException::Weight(curr_prog.amount + (max * children_weight.len() as i32));
    }
}


fn parse(text : &str) -> Vec<Program> {
    let re = Regex::new(r"([a-z]*)..([\d]*).(.*)[\n]?").unwrap();
    let sre = Regex::new(r"[^a-z]+([a-z]+)").unwrap();
    let mut result = Vec::new();
    for capture in re.captures_iter(text) {
        let name = capture[1].to_string();
        let amount : i32 = *(&capture[2].parse().unwrap());
        let mut followers = Vec::new();
        let substr = &capture[3];
        for sub_capture in sre.captures_iter(substr) {
            followers.push(sub_capture[1].to_string());
        }
        let prog = Program {name, amount, followers};
        result.push(prog);
    }
    result
}

#[test]
fn parse_test() {
    let programs = parse("nzyiue (57)\n");
    assert!(programs.len() == 1);
    assert!(programs[0].name == "nzyiue");
}
