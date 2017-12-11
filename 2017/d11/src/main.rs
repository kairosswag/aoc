extern crate regex;
use regex::*;

fn main() {
    let input = include_str!("../input/input");
    let re = Regex::new(r"[^a-z]*([a-z]*)[^a-z]*").unwrap();
    let origin = HexCell::new(0, 0, 0);
    let mut current = origin.clone();
    let mut furthest = 0;
    for element in re.captures_iter(input) {
        current = match &element[1] {
            "n"  => current.north(),
            "ne" => current.north_east(),
            "se" => current.south_east(),
            "s"  => current.south(),
            "sw" => current.south_west(),
            "nw" => current.north_west(),
            _ => panic!("somethings not right"),
        };
        furthest = furthest.max(origin.distance(&current));
    }
    let distance = origin.distance(&current);
    println!("Origin is {:?}, current is {:?}, distance is {}, furthest is {}", origin, current, distance, furthest);
}

#[derive(Debug, Copy, Clone)]
struct HexCell {
    x : i32,
    y : i32,
    z : i32,
}

impl HexCell {
    fn new(x : i32, y : i32, z : i32) -> HexCell {
        HexCell{x, y, z}
    }

    fn north(&self) -> HexCell {
        HexCell::new(self.x, self.y + 1, self.z - 1)
    }

    fn north_east(&self) -> HexCell {
        HexCell::new(self.x + 1, self.y, self.z - 1)
    }

    fn south_east(&self) -> HexCell {
        HexCell::new(self.x + 1, self.y - 1, self.z)
    }

    fn south(&self) -> HexCell {
        HexCell::new(self.x, self.y - 1, self.z + 1)
    }

    fn south_west(&self) -> HexCell {
        HexCell::new(self.x - 1, self.y, self.z + 1)
    }

    fn north_west(&self) -> HexCell {
        HexCell::new(self.x - 1, self.y + 1, self.z)
    }

    fn distance(&self, other : &HexCell) -> i32 {
        ((self.x - other.x).abs() + (self.y - other.y).abs() + (self.z - other.z).abs()) / 2
    }
}


#[test]
fn test_1() {
    let init = HexCell::new(0, 0, 0);
    let nnneee = init.north_east().north_east().north_east();
    assert_eq!(init.distance(&nnneee), 3);
}

#[test]
fn test_2() {
    let init = HexCell::new(0, 0, 0);
    let other = init.north_east().north_east().south_west().south_west();
    assert_eq!(init.distance(&other), 0);
}

#[test]
fn test_3() {
    let init = HexCell::new(0, 0, 0);
    let other = init.north_east().north_east().south().south();
    assert_eq!(init.distance(&other), 2);
}

#[test]
fn test_4() {
    let init = HexCell::new(0, 0, 0);
    let other = init.south_east().south_west().south_east().south_west().south_west();
    assert_eq!(init.distance(&other), 3);
}


