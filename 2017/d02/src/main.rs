fn main() {
    solve1();
    solve2();
}

fn solve2() {
    let input = include_str!("../input/input");
    let lines = input.split("\n");
    let mut accumulator = 0;
    for line in lines {
        let mut numbers : Vec<i32> = line.split_whitespace().map(|s| i32::from_str_radix(s, 10).expect(&format!("Could not parse {}", s))).collect();
        numbers.sort();
        numbers.reverse();
        let len = numbers.len();
        'outer: for idx in 0..len { // This all is not very stable if given slightly invalid input
            for inner_idx in idx + 1 .. len {
                if numbers[idx] % numbers[inner_idx] == 0 {
                    accumulator += numbers[idx] / numbers[inner_idx];
                    break 'outer;
                }
            }
        }
    }
    println!("Result is {}", accumulator);
}

fn solve1() {
    let input = include_str!("../input/input");
    let lines = input.split("\n");
    let mut accumulator = 0;
    for line in lines {
        let mut highest = 0; 
        let mut lowest = std::i32::MAX;
        let numbers : Vec<i32> = line.split_whitespace().map(|s| i32::from_str_radix(s, 10).expect(&format!("Could not parse {}", s))).collect();
        for number in numbers {
            if number > highest {
                highest = number;
            } 
            if number < lowest {
                lowest = number;
            }
        }
        if lowest != std::i32::MAX {
            accumulator = accumulator + highest - lowest;
        }
    }
    println!("Result is {}", accumulator);
}
