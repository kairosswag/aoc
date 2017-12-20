use std::collections::HashMap;

fn main() {
    let input = include_str!("../input/input");
    let mut programs = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
    let instructions = parse_instructions(&input);
    // execute_instructions(&mut programs, &instructions);
    times_a_billion(&mut programs, &instructions);
    let printed = program_to_print(&programs);
    println!("Program: {}", printed);
}

fn times_a_billion(programs: &mut [usize], instructions: &[Instruction]) {
    let mut map = HashMap::new();
    map.insert(program_to_print(&programs), 0);
    for i in 1..1000 * 1000 * 1000 {
        execute_instructions(programs, instructions);
        let status = program_to_print(&programs);
        if let Some(number) = map.get(&status) {
            // found cycle from number to i. 
            println!("Found cycle! From {} to {}", number, i);
            let mut remaining = ((1000 * 1000 * 1000) - i);
            remaining = remaining % (i - number); 
            for _ in 0..remaining {
                execute_instructions(programs, instructions);
            }
            break;
        } 
        map.insert(status, i);
    }
}

#[derive(Debug)]
enum Instruction {
    Spin(usize),
    Exchange(usize, usize),
    Partner(usize, usize),
}

fn parse_instructions(input: &str) -> Vec<Instruction> {
    let mut instructions = Vec::new();
    for program in input.split(',') {
        if let Some(start) = program.get(0..1) {
            use Instruction::*;
            match start {
                "s" => {
                    let amount = program.get(1..).unwrap().parse().unwrap();
                    instructions.push(Spin(amount));
                }
                "x" => {
                    let mut args = program.get(1..).unwrap().split('/');
                    let first_arg = args.next().unwrap().trim();
                    let first = first_arg.parse().expect(first_arg);
                    let second_arg = args.next().unwrap().trim();
                    let second = second_arg.parse()
                        .expect(&format!("Program: {}, first_arg: {}, second_arg: {}",
                                         program,
                                         first_arg,
                                         second_arg));
                    instructions.push(Exchange(first, second));
                }
                "p" => {
                    let mut args = program.get(1..).unwrap().split('/');
                    let first = char_to_num(args.next().unwrap().chars().next().unwrap());
                    let second = char_to_num(args.next().unwrap().chars().next().unwrap());
                    instructions.push(Partner(first, second));
                } 
                _ => panic!("Input not of sxp"),
            }
        }
    }
    instructions
}

fn execute_instructions(program: &mut [usize], instructions: &[Instruction]) {
    use Instruction::*;
    // println!("P: {}", program_to_print(&program));
    for instruction in instructions {
        match *instruction {
            Spin(amount) => spin(program, amount),
            Exchange(first, second) => exchange(program, first, second),
            Partner(first, second) => partner(program, first, second),
        }
        // println!("P: {}", program_to_print(&program));
    }

}

fn spin(program: &mut [usize], amount: usize) {
    let mut copy = [0; 16];
    let len = program.len();
    copy[..len].clone_from_slice(program);
    for i in 0..len {
        program[i] = copy[(i + len - amount) % len];
    }
}

fn exchange(program: &mut [usize], first: usize, second: usize) {
    let tmp = program[first];
    program[first] = program[second];
    program[second] = tmp;
}

fn partner(program: &mut [usize], first: usize, second: usize) {
    for i in 0..program.len() {
        if program[i] == first {
            program[i] = second;
        } else if program[i] == second {
            program[i] = first;
        }
    }
}

fn program_to_print(program: &[usize]) -> String {
    program.iter().map(|num| num_to_char(*num)).collect()
}

fn num_to_char(number: usize) -> char {
    match number {
        0 => 'a',
        1 => 'b',
        2 => 'c',
        3 => 'd',
        4 => 'e',
        5 => 'f',
        6 => 'g',
        7 => 'h',
        8 => 'i',
        9 => 'j',
        10 => 'k',
        11 => 'l',
        12 => 'm',
        13 => 'n',
        14 => 'o',
        15 => 'p',
        _ => panic!("num to char failed."),
    }
}

fn char_to_num(character: char) -> usize {
    match character {
        'a' => 0,
        'b' => 1,
        'c' => 2,
        'd' => 3,
        'e' => 4,
        'f' => 5,
        'g' => 6,
        'h' => 7,
        'i' => 8,
        'j' => 9,
        'k' => 10,
        'l' => 11,
        'm' => 12,
        'n' => 13,
        'o' => 14,
        'p' => 15,
        _ => panic!("char to num failed"),
    }
}

#[test]
fn spin_test() {
    let mut test = vec![0, 1, 2, 3, 4, 5];
    spin(&mut test, 2);
    assert_eq!(test, vec![4, 5, 0, 1, 2, 3]);
}

#[test]
fn execute_test() {
    use Instruction::*;
    let instructions = vec![Spin(1), Exchange(3, 4), Partner(4, 1)];
    let mut program = [0, 1, 2, 3, 4];
    execute_instructions(&mut program, &instructions);
    assert_eq!(program, [1, 0, 4, 3, 2]);
}