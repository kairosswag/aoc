extern crate ascii;
use ascii::{AsciiStr, AsciiChar};
use std::fmt::*;

fn main() {
    let input = include_str!("../input/input");
    if let Ok(ascii_string) = AsciiStr::from_ascii(input) {
        let (group, _, removed) = construct_groups(&ascii_string, 0, 1);
        //println!("{:?}\n\n####################################################\n{}", &group, descend_groups(&group));
        println!("Total: {}; removed {}", descend_groups(&group), removed);
    }

}

fn descend_groups(group : &Group) -> u32 {
    group.tier + group.subgroups.iter().map(|sg| descend_groups(sg)).fold(0, |acc, x| acc + x)
}

fn construct_groups(string : &AsciiStr, idx : usize, tier : u32) -> (Group, usize, usize) {
    let mut garbage_removed = 0;
    let mut curr_idx = idx;
    let mut ch = AsciiChar::CurlyBraceOpen;
    let mut subgroups = Vec::new();
    let mut garbage = false;
    while {curr_idx += 1; ch = string[curr_idx]; garbage || ch != AsciiChar::CurlyBraceClose} {
        //println!("[{}]: Character found: _{}, garbage? _{}", tier, ch, garbage);
        match (garbage, ch) {
            (true, AsciiChar::LessThan) => garbage_removed += 1,
            (false, AsciiChar::LessThan) => garbage = true,
            (true, AsciiChar::GreaterThan) => garbage = false, // Always turns off.
            (false, AsciiChar::GreaterThan) => panic!("Wrong assumption gt!"),
            (true, AsciiChar::Exclamation) => curr_idx += 1, //skip the next
            (false, AsciiChar::Exclamation) => panic!("Wrong assumption exclamation!"),
            (true, AsciiChar::CurlyBraceOpen) => garbage_removed += 1,
            (false, AsciiChar::CurlyBraceOpen) => {
                let (sg, sg_idx, sg_garbage_removed) = construct_groups(string, curr_idx, tier + 1);
                subgroups.push(sg);
                curr_idx = sg_idx;
                garbage_removed += sg_garbage_removed;
            },
            (false, AsciiChar::Comma) => (), // Do nothing; , is the only char allowed outside of garbage besides braces and starting garbage.
            (false, ch) => panic!(format!("Unexpected character outside of Garbage! {}", &ch)),
            (true, _) => garbage_removed += 1,
        }
    }
    //println!("Closing group!");

    let group = Group{tier, subgroups};

    (group, curr_idx, garbage_removed)
}

struct Group {
    tier : u32,
    subgroups : Vec<Group>,
}


impl Debug for Group {
     fn fmt(&self, f: &mut Formatter) -> Result  {
         let mut s = String::new();
         for sg in &self.subgroups {
             s.push_str(&format!("{:?}", *sg));
         }
         write!(f, "{{ {} {} }}", self.tier, s)
     }
}
