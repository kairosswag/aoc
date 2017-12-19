extern crate knot_hash;
use knot_hash::*;

fn main() {
    let input = "hfdlxzhv";
    let res1 = solve1(&input);
    println!("Solution to #1: {}", res1);
    let res2 = solve2(&input);
    println!("Solution to #2: {}", res2);
}

fn solve1(input: &str) -> u32 {
    let mut total = 0;
    for i in 0..128 {
        let row_name = format!("{}-{}", input, i);
        let row_hash = knot_hash(row_name.as_bytes());
        total += num_squares_used(&row_hash);
    }
    total
}

fn solve2(input: &str) -> u32 {
    let mut grid = [[SquareStatus::EMPTY; 128]; 128];
    let mut total = 0;
    for i in 0..128 {
        let row_name = format!("{}-{}", input, i);
        let row_hash = knot_hash(row_name.as_bytes());
        for (ch_idx, ch) in row_hash.chars().enumerate() {
            let (b1, b2, b3, b4) = is_used(&ch);
            let char_offset = ch_idx * 4;
            if b1 {
                grid[i][char_offset + 0] = SquareStatus::USED;
                total += 1;
            }
            if b2 {
                grid[i][char_offset + 1] = SquareStatus::USED;
                total += 1;
            }
            if b3 {
                grid[i][char_offset + 2] = SquareStatus::USED;
                total += 1;
            }
            if b4 {
                grid[i][char_offset + 3] = SquareStatus::USED;
                total += 1;
            }
        }
    }
    // println!("total: {}", total);
    // print!("\n");
    // for i in 0..16 {
    //     for j in 0..16 {
    //         let print = if grid[i][j] == SquareStatus::EMPTY {
    //             "."
    //         } else {
    //             "#"
    //         };
    //         print!("{}", print);
    //     }
    //     print!("\n");
    // }
    let mut total_regions = 0;
    for x in 0..128 {
        for y in 0..128 {
            if grid[x][y] == SquareStatus::USED {
                total_regions += 1;
                visit(&mut grid, x, y);
            }
        }
    }
    total_regions
}

#[derive(PartialEq, Clone, Copy)]
enum SquareStatus {
    EMPTY,
    USED,
    VISITED,
}

fn visit(board: &mut [[SquareStatus; 128]], x: usize, y: usize) {
    let mut stack = vec![(x, y)];
    while !stack.is_empty() {
        if let Some((cx, cy)) = stack.pop() {
            if cx < 128 && cy < 128 && board[cx][cy] == SquareStatus::USED {
                board[cx][cy] = SquareStatus::VISITED;
                stack.push((cx + 1, cy));
                stack.push((cx, cy + 1));
                if cx > 0 {
                    stack.push((cx - 1, cy));
                }
                if cy > 0 {
                    stack.push((cx, cy - 1));
                }
            }
        }
    }
}

fn is_used(input: &char) -> (bool, bool, bool, bool) {
    match *input {
        '0' => (false, false, false, false),
        '1' => (false, false, false, true),
        '2' => (false, false, true, false),
        '3' => (false, false, true, true),
        '4' => (false, true, false, false),
        '5' => (false, true, false, true),
        '6' => (false, true, true, false),
        '7' => (false, true, true, true),
        '8' => (true, false, false, false),
        '9' => (true, false, false, true),
        'a' => (true, false, true, false),
        'b' => (true, false, true, true),
        'c' => (true, true, false, false),
        'd' => (true, true, false, true),
        'e' => (true, true, true, false),
        'f' => (true, true, true, true),
        _ => panic!("nope"),
    }
}

fn num_squares_used(input: &str) -> u32 {
    let mut res = 0;
    for c in input.chars() {
        res += match c {
            '0' => 0,
            '1' => 1,
            '2' => 1,
            '3' => 2,
            '4' => 1,
            '5' => 2,
            '6' => 2,
            '7' => 3,
            '8' => 1,
            '9' => 2,
            'a' => 2,
            'b' => 3,
            'c' => 2,
            'd' => 3,
            'e' => 3,
            'f' => 4,
            _ => panic!("nope"),
        }
    }
    res
}

#[test]
fn test_1() {
    let test_input = "flqrgnkx";
    let res = solve1(test_input);
    assert_eq!(res, 8108);
}

#[test]
fn test_2() {
    let test_input = "flqrgnkx";
    let res = solve2(test_input);
    assert_eq!(res, 1242);
}
