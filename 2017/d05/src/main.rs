fn main() {
    let input = include_str!("../input/input");
    let mut lines: Vec<i32> = input.split("\n").flat_map(|s| s.trim().parse()).collect();
    //lines = vec![0, 3, 0, 1, -3];

    let mut idx: i32 = 0;
    let mut steps: u32 = 0;
    let len = lines.len() as i32;
    while 0 <= idx && idx < len {
        //println!("#{:?} [{}]", lines, idx);
        steps += 1;
        let index = idx as usize;
        let jump = lines[index];
        lines[index] = if jump >= 3 { jump - 1  } else { jump + 1 };
        idx = idx + jump;
    }
        //println!("#{:?} [{}]", lines, idx);

    println!("Number of steps: {}", steps);
}
