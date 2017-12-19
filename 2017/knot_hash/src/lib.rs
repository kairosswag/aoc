pub fn knot_hash(input: &[u8]) -> String {
    let mut numbers = [0; 256];
    for i in 0..256 {
        numbers[i] = i;
    }

    let mut curr_pos = 0;
    let mut skip_size = 0;

    let added = vec![17, 31, 73, 47, 23];
    let lengths = [&input[..], &added[..]].concat();
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
    for chunk in 0..16 {
        let mut tmp = 0;
        for index in 0..16 {
            tmp = tmp ^ numbers[16 * chunk + index];
        }
        result[chunk] = tmp;
        res_str.push_str(&format!("{:02x}", tmp));
    }
    res_str
}

fn reverse(numbers: &mut [usize], lenght: usize, curr_pos: usize) {
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
}



#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn test_inputs() {
        let input = ["", "AoC 2017", "1,2,3", "1,2,4"];
        let expected = ["a2582a3a0e66e6e86e3812dcb672a272", "33efeb34ea91902bb2f59c9920caa6cd", "3efbe78a8d82f29979031a4aa0b16a9d", "63960835bcdc130f0b66d7ff4f6a5a8e"];
        for i in 0..input.len() {
            assert_eq!(super::knot_hash(input[i].as_bytes()), expected[i]);
        }
    }
}
