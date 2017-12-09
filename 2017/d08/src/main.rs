extern crate regex;
use regex::*;
use std::collections::HashMap;

fn main() {
    let input = include_str!("../input/input");
    let mut program : Vec<Instruction> = Vec::new();
    //let re = Regex::new(r"([a-z]*)\s([a-z]*)\s(^\s*).[a-z]*.([a-z]*).([!=><]+).(^\\s*).*").unwrap();
    let re = Regex::new(r"\n?([a-z]*)\s([a-z]*)\s([^\s]*)\s[a-z]*\s([a-z]*)\s([!<>=]*)\s([^\s]*).*").unwrap();
    for line in input.lines() {
        for capture in re.captures_iter(line) {
            let register = String::from(&capture[1]);
            let operation = get_op(&capture[2]);
            let amount = get_i32(&capture[3]);
            let comp_reg = String::from(&capture[4]);
            let operator = get_comp_op(&capture[5]);
            let comp_magnitude = get_i32(&capture[6]);
            let inst = Instruction {register, operation, amount, comp_reg, operator, comp_magnitude};
            program.push(inst);
        }
    }
    let (res, all_max) = execute(&program);
    let max = res.iter().max();
    //println!("{:?}", res);
    println!("max: {}, all_max: {}", max.unwrap(), all_max);
}

fn execute(program : &[Instruction]) -> (Vec<i32>, i32) {
    let mut reg_idx_map : HashMap<String, u32> = HashMap::new();
    let mut registry : Vec<i32> = Vec::new();

    let mut all_max = 0;

    for instruction in program {
        let reg = instruction.comp_reg.clone();
        let reg_value = if let Some(reg_idx) = reg_idx_map.get(&reg) {
            registry[*reg_idx as usize]
        } else {
            0
        };

        if eval(reg_value, &instruction.operator, instruction.comp_magnitude) {
            let idx = reg_idx_map.entry(instruction.register.clone()).or_insert(registry.len() as u32);
            if *idx as usize == registry.len() {
                registry.push(0);
            }
            match instruction.operation {
                Operation::Inc => registry[*idx as usize] += instruction.amount,
                Operation::Dec => registry[*idx as usize] -= instruction.amount,
            }
            all_max = all_max.max(registry[*idx as usize]);
        }
    }

    (registry, all_max)

}

fn eval(lhs : i32, cmp : &CompOperator, rhs : i32 ) -> bool {
    use CompOperator::*;
    match *cmp {
        Equal => lhs == rhs,
        Neq => lhs != rhs,
        Gt => lhs > rhs,
        GtoE => lhs >= rhs,
        Lt => lhs < rhs,
        LtoE => lhs <= rhs,
    }
}

fn get_op(str_op : &str) -> Operation {
    match str_op {
        "inc" => Operation::Inc,
        "dec" => Operation::Dec,
        _ => panic!(format!("Could not parse {} as Operation!", str_op))
    }
}

fn get_i32(str_int : &str) -> i32 {
    str_int.parse().unwrap()
}

fn get_comp_op(str_cmp_op : &str) -> CompOperator {
    use CompOperator::*;
    match str_cmp_op {
        "<" => Lt,
        ">" => Gt,
        ">=" => GtoE,
        "=>" => GtoE,
        "<=" => LtoE,
        "=<" => LtoE,
        "==" => Equal,
        "!=" => Neq,
        "=!" => Neq,
        _ => panic!(format!("Could not parse {} as Operation!", str_cmp_op))
    }
}

#[derive(Debug)]
struct Instruction {
    register : String,
    operation : Operation,
    amount : i32,
    comp_reg : String,
    operator : CompOperator,
    comp_magnitude : i32,
}
#[derive(Debug)]
enum Operation {
    Inc, Dec
}
#[derive(Debug)]
enum CompOperator {
    Equal, Neq, Gt, Lt, GtoE, LtoE
}
