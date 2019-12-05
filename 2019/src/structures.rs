#![allow(dead_code)]

use std::cmp::*;


#[derive(Debug)]
pub enum Direction {
    UP(u32),
    DOWN(u32),
    LEFT(u32),
    RIGHT(u32),
}

impl Direction {
    pub fn transform(&self, arg: &Pos) -> Pos {
        match &self {
            Direction::UP(val) => Pos::new(arg.x, arg.y + *val as i32),
            Direction::DOWN(val) => Pos::new(arg.x, arg.y - *val as i32),
            Direction::LEFT(val) => Pos::new(arg.x - *val as i32, arg.y),
            Direction::RIGHT(val) => Pos::new(arg.x + *val as i32, arg.y),
        }
    }
}

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub struct Line {
    pub p_0: Pos,
    pub p_1: Pos,
}

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub struct Pos {
    pub x: i32,
    pub y: i32,
}

impl Pos {
    pub fn new(x: i32, y: i32) -> Pos {
        Pos { x, y }
    }

    // manhattan distance
    pub fn distance_to(&self, other: &Pos) -> u32 {
        ((self.x.abs() - other.x.abs()).abs() + (self.y.abs() - other.y.abs()).abs()) as u32
    }
}

impl Line {
    pub fn new(a: Pos, b: Pos) -> Line {
        // Line is either horizontal or vertial, horizontal points are ordered l2r, vertical t2b
        if a.x == b.x {
            if a.y > b.y {
                return Line { p_0: b, p_1: a };
            } else {
                return Line { p_0: a, p_1: b };
            }
        } else {
            if a.x > b.x {
                return Line { p_0: a, p_1: b };
            } else {
                return Line { p_0: b, p_1: a };
            }
        }
    }

    pub fn is_horizontal(&self) -> bool {
        self.p_0.y == self.p_1.y
    }

    pub fn intersect(&self, other: &Line) -> Option<Pos> {
        match (self.is_horizontal(), other.is_horizontal()) {
            (false, true) => {
                if self.p_0.y < other.p_0.y
                    && self.p_1.y > other.p_0.y
                    && self.p_0.x < other.p_0.x
                    && self.p_0.x > other.p_1.x
                {
                    Some(Pos::new(self.p_0.x, other.p_0.y))
                } else {
                    None
                }
            }, 
            (true, false) => {
                if other.p_0.y < self.p_0.y
                && other.p_1.y > self.p_0.y
                && other.p_0.x < self.p_0.x
                && other.p_0.x > self.p_1.x
                {
                    Some(Pos::new(other.p_0.x, self.p_0.y))
                } else {
                    None
                }
            },
            _ => None,
        }
    }

    pub fn get_steps(&self) -> u32 {
        if self.is_horizontal() {
            (max(self.p_0.x, self.p_1.x) - min(self.p_0.x, self.p_1.x)) as u32
        } else {
            (max(self.p_0.y, self.p_1.y) - min(self.p_0.y, self.p_1.y)) as u32
        }
    }
}

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub struct Instruction {
    pub opcode: Opcode, 
    pub mode: [i32; 4],
}

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub enum Opcode {
    ADD,
    MULT,
    INPUT,
    OUTPUT,
    END,
    JUMP_IF_TRUE,
    JUMP_IF_FALSE,
    LESS_THAN,
    EQUALS,
}

impl Instruction {

    pub fn from_slice(part: &[i32]) -> Instruction {
        let code = part[0] % 100;
        let mut mode = [0; 4];
        mode[0] = (part[0] % 1000) / 100;
        mode[1]= (part[0] % 10000) / 1000;
        mode[2] = (part[0] % 100000) / 10000;
        use self::Opcode::*;
        let opcode = match code {
            1 => ADD,
            2 => MULT,
            3 => INPUT,
            4 => OUTPUT,
            5 => JUMP_IF_TRUE,
            6 => JUMP_IF_FALSE,
            7 => LESS_THAN,
            8 => EQUALS,
            99 => END,
            _ => panic!("Invalid opcode {}", part[0])
        };
        Instruction { opcode, mode }
    }

    pub fn execute(&self, data: &mut Vec<i32>, pos: usize, ext_input: i32) -> (i32, usize, bool) {
        use self::Opcode::*;
        let mut output = 0;
        let mut sig_end = false;
        let nextpos = match self.opcode {
            ADD => {
                let res_idx = self.param_n(3, pos, &data, true) as usize;
                data[res_idx] = self.param_n(1, pos, &data, false) + self.param_n(2, pos, &data, false);
                pos + 4
            },
            MULT => {
                let res_idx = self.param_n(3, pos, &data, true) as usize;
                data[res_idx] = self.param_n(1, pos, &data, false) * self.param_n(2, pos, &data, false);
                pos + 4
            },
            INPUT => {
                let res_idx = self.param_n(1, pos, &data, true) as usize;
                data[res_idx] = ext_input;
                pos + 2
            },
            OUTPUT => {
                output = self.param_n(1, pos, &data, false);
                pos + 2
            },
            JUMP_IF_TRUE => {
                if self.param_n(1, pos, &data, false) != 0 {
                    self.param_n(2, pos, &data, false) as usize
                } else {
                    pos + 3
                }
            },
            JUMP_IF_FALSE => {
                if self.param_n(1, pos, &data, false) == 0 {
                    self.param_n(2, pos, &data, false) as usize
                } else {
                    pos + 3
                }
            }, 
            LESS_THAN => {
                let res_idx = self.param_n(3, pos, &data, true) as usize;
                if self.param_n(1, pos, &data, false) < self.param_n(2, pos, &data, false) {
                    data[res_idx] = 1;
                } else {
                    data[res_idx] = 0;
                }
                pos + 4
            }, 
            EQUALS => {
                let res_idx = self.param_n(3, pos, &data, true) as usize;
                if self.param_n(1, pos, &data, false) == self.param_n(2, pos, &data, false) {
                    data[res_idx] = 1;
                } else {
                    data[res_idx] = 0;
                }
                pos + 4
            }
            END => {
                sig_end = true;
                pos
            }
        };
        (output, nextpos, sig_end)
    }

    fn param_n(&self, n: usize, pos: usize, data: &[i32], out: bool) -> i32 {
        if !out && self.mode[n-1] == 0 {
            data[data[pos + n] as usize]
        } else {
            data[pos + n]
        }
    }

}
