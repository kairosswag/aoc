fn main() {
    let lengths = vec![49, 57, 57, 44, 48, 44, 50, 53, 53, 44, 49, 51, 54, 44, 49, 55, 52, 44, 50,
                       53, 52, 44, 50, 50, 55, 44, 49, 54, 44, 53, 49, 44, 56, 53, 44, 49, 44, 50,
                       44, 50, 50, 44, 49, 55, 44, 55, 44, 49, 57, 50, 10]; // gotten from input as bytes
    //solve(&lengths);


    let input = include_str!("../input/input");
    for line in input.lines() {
        let bytes = line.as_bytes();
        let hash = solve(&bytes);
        println!("lines:\n");
        println!("{}\n{}", line, hash);
    }
}

fn solve(lengths: &[u8]) -> String {
    let mut numbers = [0; 256];
    for i in 0..256 {
        numbers[i] = i;
    }
    
    let mut curr_pos = 0;
    let mut skip_size = 0;
  
    let added = vec![17, 31, 73, 47, 23];
    let lengths = [&lengths[..], &added[..]].concat();
    let num_len = numbers.len();
    for _ in 0..64 {
        for &lenght in &lengths {
            let lenght = lenght as usize;
            reverse(&mut numbers, lenght, curr_pos);
            curr_pos = (curr_pos + lenght + skip_size) % num_len;
            skip_size += 1;
        }
    }
    let mut result = [0; 16];
    let mut res_str = String::new();
    let mut debug_str = String::new();
    for chunk in 0..16 {
        let mut tmp = 0;
        for index in 0..16 {
            tmp = tmp ^ numbers[16 * chunk + index];
        }
        result[chunk] = tmp;
        res_str.push_str(&format!("{:02x}", tmp));
    }
    // println!("{}", res_str);
    // println!("First: {}; Second: {}; Multipilied: {}",
    //          numbers[0],
    //          numbers[1],
    //          numbers[0] * numbers[1]);
    res_str
}

pub fn reverse(numbers: &mut [usize], lenght: usize, curr_pos: usize) {
    let mut tmp = vec![0; lenght];
    for i in 0..lenght {
        let idx = (curr_pos + i) % numbers.len();
        tmp[i] = numbers[idx as usize];
    }
    for i in 0..lenght {
        let idx = (curr_pos + i) % numbers.len();
        let reversed = lenght - (i + 1);
        numbers[idx] = tmp[reversed];
    }
    // println!("Numbers: {:?}, Curr_pos: {}, Len: {}", numbers, curr_pos, lenght);
}
