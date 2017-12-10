fn main() {
    //let input=include_bytes!("../input/input");
    let mut numbers = [0; 256];
    for i in 0..256 {
        numbers[i] = i;
    }

    // let mut numbers = [0; 5];
    // for i in 0..5 {
    //     numbers[i] = i;
    // }

    let mut curr_pos = 0;
    let mut skip_size = 0;
    let lengths = vec![199,0,255,136,174,254,227,16,51,85,1,2,22,17,7,192]; //from input
    //let lengths = vec![3, 4, 1, 5]; // tests
    for lenght in lengths {
        reverse(&mut numbers, lenght, curr_pos);
        curr_pos = curr_pos + lenght + skip_size;
        skip_size += 1;
    }
    println!("First: {}; Second: {}; Multipilied: {}", numbers[0], numbers[1], numbers[0] * numbers[1]);
}

pub fn reverse(numbers : &mut [usize], lenght : usize, curr_pos : usize) {
    let mut tmp = vec![0; lenght];
    for i in 0 .. lenght {
        let idx = (curr_pos + i) % numbers.len();
        tmp[i] = numbers[idx as usize];
    }
    for i in 0 .. lenght {
        let idx = (curr_pos + i) % numbers.len();
        let reversed = lenght - (i + 1);
        numbers[idx] = tmp[reversed];
    }
    //println!("Numbers: {:?}, Curr_pos: {}, Len: {}", numbers, curr_pos, lenght);
}
