fn main() {
    let input = include_str!("../input/input");
    let input_list : Vec<&str> = input.split(", ").collect();
    let mut g_state = State { x_pos : 0, y_pos : 0, direction : Cardinal::NORTH};
    for direction in input_list {
        let (lr, steps) = deconstruct(direction);
        go_direction(steps, lr, &mut g_state);
    }
    println!("{:?}", g_state);
    println!("And the answer is: {}", g_state.x_pos.abs() + g_state.y_pos.abs());
}

fn go_direction(steps : i32, direction : LR, state : &mut State) {
    let dir = state.direction.clone();
    match (dir, direction) {
        (Cardinal::NORTH, LR::LEFT) => state.walk_hor(-steps),
        (Cardinal::NORTH, LR::RIGHT) => state.walk_hor(steps),
        (Cardinal::EAST, LR::LEFT) => state.walk_ver(steps),
        (Cardinal::EAST, LR::RIGHT) => state.walk_ver(-steps),
        (Cardinal::SOUTH, LR::LEFT) => state.walk_hor(steps),
        (Cardinal::SOUTH, LR::RIGHT) => state.walk_hor(-steps),
        (Cardinal::WEST, LR::LEFT) => state.walk_ver(-steps),
        (Cardinal::WEST, LR::RIGHT) => state.walk_ver(steps)
    }
    println!("Walked to {:?} ({} steps)", state, steps);
}

fn deconstruct(dir : &str) -> (LR, i32) {
    let (s_lr, s_num) = dir.split_at(1);
    let lr = match s_lr.chars().nth(0).unwrap() {
        'L' => LR::LEFT,
        'R' => LR::RIGHT,
        _ => panic!("Not a valid token.")
    };
    let step = i32::from_str_radix(s_num.trim(), 10).expect(&format!("snum was: {}", s_num));
    (lr, step)
}

#[derive(Debug)]
struct State {
    x_pos : i32,
    y_pos : i32,
    direction : Cardinal
}

impl State {
    fn walk_ver(&mut self, y : i32) {
        self.y_pos += y;
        self.direction = if y > 0 {
            Cardinal::NORTH 
        } else {
            Cardinal::SOUTH
        };
    }

    fn walk_hor(&mut self, x : i32) {
        self.x_pos += x;
        self.direction = if x > 0 {
            Cardinal::EAST
        } else {
            Cardinal::WEST
        };
    }
}

enum LR {
    LEFT,
    RIGHT
}

#[derive(Debug, Clone)]
enum Cardinal {
    NORTH,
    EAST,
    SOUTH,
    WEST
}
