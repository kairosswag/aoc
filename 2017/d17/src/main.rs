fn main() {
    solve1naive(3);
    solve1naive(343);
    solve2naive(343);
}

fn solve1naive(step : usize) {
    let mut buffer = [0; 2018];
    let mut curr_step = 1;
    let mut curr_idx = 0;
    while curr_step != 2018 {
        let new_idx = (curr_idx + step) % curr_step + 1;
        move_back_buffer(&mut buffer, curr_step, new_idx);
        buffer[new_idx] = curr_step;
        curr_idx = new_idx;
        curr_step += 1;
    }
    println!("Value after {} is {}", buffer[curr_idx], buffer[curr_idx + 1])
}

fn solve2naive(step : usize) {
    let mut pos_of_zero = 0;
    let mut val_after_zero = 0;
    let mut curr_idx = 0;
    let mut curr_step = 1;
    while curr_step != 50000001 {
        if curr_step % 1000000 == 0 {
            println!(">>>> {}", curr_step);
        }
        let new_idx = (curr_idx + step) % curr_step + 1;
        if new_idx - 1 == pos_of_zero {
            val_after_zero = curr_step;
        }
        if new_idx < pos_of_zero {
            pos_of_zero += 1;
        }
        curr_idx = new_idx;
        curr_step += 1;
    }

    println!("Value after 0 is {}", val_after_zero);
}

fn move_back_buffer_box(mut buffer: Box<[usize; 50000001]>, last_idx : usize, first_idx : usize) -> [usize; 50000001] {
    for i in 0..(last_idx - first_idx) {
        buffer[last_idx - i] = buffer[last_idx - i - 1];
    }
    *buffer
}

fn move_back_buffer(buffer: &mut [usize], last_idx : usize, first_idx : usize) {
    for i in 0..(last_idx - first_idx) {
        buffer[last_idx - i] = buffer[last_idx - i - 1];
    }
}

