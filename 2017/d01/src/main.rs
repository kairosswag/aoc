use std::str::Chars;

fn main() {
    let input = include_str!("../input/input");
    //let input = "123425";
    let chars = input.trim().chars();

    let numbers = as_integer_list(chars);

//--------------------- Part 1 --------------------------

    let mut prev_number = numbers[numbers.len() - 1];
    let mut accum1 : u32 = 0;
    for number in &numbers {
        if *number == prev_number {
            accum1 = accum1 + number;
        } 
        prev_number = *number;
    }
    println!("Result1 is {}", accum1);

//--------------------- Part 2 --------------------------

    let mut accum2 : u32 = 0;
    for idx in 0..numbers.len() {
        let mirror_idx = (idx + (numbers.len()/2)) % numbers.len();
        let mirror_number = numbers[mirror_idx];
        if numbers[idx] == mirror_number {
            accum2 = accum2 + numbers[idx];
        }
    }

    println!("Result2 is {}", accum2);
}

fn as_integer_list(chars : Chars) -> Vec<u32> {
    chars.map(|c| u32::from_str_radix(&format!("{}", c), 10).expect(&format!("snum was: {}", c))).collect()
}
