use std::io;
use std::collections::HashMap;

fn main() {
    loop {
        let number = {
            let mut maybe_number = None;
            while maybe_number.is_none() {
                let mut buffer = String::new();
                io::stdin().read_line(&mut buffer).unwrap();
                let trimmed = buffer.trim();
                match trimmed.parse::<u32>() {
                    Ok(i) => maybe_number = Some(i),
                    Err(..) => println!("this was not an integer: {}", trimmed),
                };
            }
            maybe_number.unwrap()
        };
        let idx_to_pos = solve1(number*2);
        solve2(number, idx_to_pos);
    }
}

fn solve1(number: u32) -> Vec<(i32, i32)> {
    use Direction::*;
    let mut curr_x_pos = 0;
    let mut curr_y_pos = 0;
    let mut max_x_pos = 0;
    let mut min_x_pos = 0;
    let mut max_y_pos = 0;
    let mut min_y_pos = 0;
    let mut direction = Right;

    let mut idx_to_pos = Vec::new();
    idx_to_pos.push((curr_x_pos, curr_y_pos));
    for _ in 1..number {
        match direction {
            Up => {
                curr_y_pos += 1;
                let (min, max, swap) = next_dir(curr_y_pos, min_y_pos, max_y_pos);
                min_y_pos = min;
                max_y_pos = max;
                if swap {
                    direction = Left;
                }
            }
            Down => {
                curr_y_pos -= 1;
                let (min, max, swap) = next_dir(curr_y_pos, min_y_pos, max_y_pos);
                min_y_pos = min;
                max_y_pos = max;
                if swap {
                    direction = Right;
                }
            }
            Right => {
                curr_x_pos += 1;
                let (min, max, swap) = next_dir(curr_x_pos, min_x_pos, max_x_pos);
                min_x_pos = min;
                max_x_pos = max;
                if swap {
                    direction = Up;
                }
            }
            Left => {
                curr_x_pos -= 1;
                let (min, max, swap) = next_dir(curr_x_pos, min_x_pos, max_x_pos);
                min_x_pos = min;
                max_x_pos = max;
                if swap {
                    direction = Down;
                }
            }
        }
        idx_to_pos.push((curr_x_pos, curr_y_pos));
    }
    let distance = curr_x_pos.abs() + curr_y_pos.abs();
    println!("xpos: {}, ypos: {}, distance: {}",
             curr_x_pos,
             curr_y_pos,
             distance);

    idx_to_pos
}

fn solve2(number : u32, idx_to_pos : Vec<(i32, i32)> ) {
    let mut coord_to_value = HashMap::new();
    coord_to_value.insert((0, 0), 1);
    let mut curr_value = 1;
    let mut curr_index = 1;
    while curr_value < number {
        let (x, y) = idx_to_pos[curr_index];
        curr_index += 1;

        let n  = *coord_to_value.get(&(x, y+1)).unwrap_or(&0);
        let ne = *coord_to_value.get(&(x+1, y+1)).unwrap_or(&0);
        let e  = *coord_to_value.get(&(x+1, y)).unwrap_or(&0);
        let se = *coord_to_value.get(&(x+1, y-1)).unwrap_or(&0);
        let s  = *coord_to_value.get(&(x, y-1)).unwrap_or(&0);
        let sw = *coord_to_value.get(&(x-1, y-1)).unwrap_or(&0);
        let w  = *coord_to_value.get(&(x-1, y)).unwrap_or(&0);
        let nw = *coord_to_value.get(&(x-1, y+1)).unwrap_or(&0);

        curr_value = n + ne + e + se + s + sw + w + nw;
        println!("curr_value: {} @{}", curr_value, curr_index);
        coord_to_value.insert((x, y), curr_value);
    }
    println!("Result2: at index {} value {}", curr_index, curr_value);
}

fn next_dir(curr: i32, min: i32, max: i32) -> (i32, i32, bool) {
    if curr > max {
        (min, curr, true)
    } else if curr < min {
        (curr, max, true)
    } else {
        (min, max, false)
    }
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}
