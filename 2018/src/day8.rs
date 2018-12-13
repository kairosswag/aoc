use std::slice::Iter;

#[derive(new)]
pub struct Node {
    pub header_num_ch_nodes: u32,
    pub header_num_metadata: u32,
    pub metadata_entry: Vec<u32>,
}

#[aoc_generator(day8)]
pub fn day8_input_generator(input: &str) -> Vec<u32> {
    input.split(" ").flat_map(|s| s.parse::<u32>()).collect()
}

#[aoc(day8, part1)]
pub fn part1(input: &[u32]) -> u32 {
    let mut stack = Vec::new();
    let mut iin = input.iter();
    traverse(&mut iin, &mut stack);

    stack.iter().map(|node| &node.metadata_entry).flatten().fold(0, |acc, m| acc + m)
}

pub fn traverse(input: &mut Iter<u32>, stack: &mut Vec<Node>) {
    let num_children = input.next().unwrap();
    let num_meta = input.next().unwrap();
    for i in 0..*num_children {
        traverse(input, stack);
    }
    let mut meta = Vec::new();
    for _ in 0..*num_meta {
        meta.push(*input.next().unwrap());
    }
    let node = Node::new(*num_children, *num_meta, meta);
    stack.push(node);
}


#[derive(new, Debug)]
pub struct Node2 {
    pub id: u32,
    pub header_num_ch_nodes: u32,
    pub header_num_metadata: u32,
    pub child_nodes: Vec<u32>,
    pub metadata_entry: Vec<u32>,
}

pub struct SeqGen {
    curr_val: u32,
}

impl SeqGen {
    pub fn init() -> SeqGen {
        SeqGen{curr_val: 0}
    }

    pub fn next_val(&mut self) -> u32 {
        self.curr_val += 1;
        self.curr_val
    }
}


#[aoc(day8, part2)]
pub fn part2(input: &[u32]) -> u32 {
    let mut stack = Vec::new();
    let mut iin = input.iter();
    let mut seq = SeqGen::init();
    traverse2(&mut iin, &mut stack, &mut seq);

    // stack.iter().map(|node| &node.metadata_entry).flatten().fold(0, |acc, m| acc + m)

    let first = stack.iter().find(|node| node.id == 1).unwrap();
    calculate_node_value(&first, &stack)
}

pub fn calculate_node_value(node: &Node2, stack: &[Node2]) -> u32 {
    let child_nodes = &node.child_nodes;
    let num_nodes = child_nodes.len() as u32;
    if num_nodes == 0 {
        node.metadata_entry.iter().fold(0, |acc, m| acc + m)
    } else {
        let mut accum = 0;
        for idx in &node.metadata_entry {
            if *idx != 0 && idx-1 < num_nodes {
                let identifier = &child_nodes[(idx-1) as usize];
                let child = stack.iter().find(|node| node.id == *identifier).unwrap();
                accum += calculate_node_value(&child, stack);
            }
        }
        accum
    }
}

pub fn traverse2(input: &mut Iter<u32>, stack: &mut Vec<Node2>, seq: &mut SeqGen) -> u32 {
    let num_children = input.next().unwrap();
    let num_meta = input.next().unwrap();
    let idx = seq.next_val();
    let mut children = Vec::new();
    for i in 0..*num_children {
        children.push(traverse2(input, stack, seq));
    }
    let mut meta = Vec::new();
    for _ in 0..*num_meta {
        meta.push(*input.next().unwrap());
    }
    let node = Node2::new(idx, *num_children, *num_meta, children, meta);
    stack.push(node);
    return idx;
}

#[test]
fn test_1() {
    let gen = day8_input_generator("2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2");
    assert_eq!(part2(&gen), 66);
}