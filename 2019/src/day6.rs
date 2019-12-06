use itertools::Itertools;
use std::collections::HashMap;


#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub struct OrbitRelationship {
    orbiter: Orb,
    orbitee: Orb
}

#[derive(Clone, Copy, Eq, PartialEq, Debug, Hash)]
pub struct Orb {
    name: u32,
}

impl From<&str> for Orb {
    fn from(input: &str) -> Orb {
        Orb { name : input.as_bytes().iter().zip(1..).fold(0, |state, (bt, idx) | state +  hash_b(idx, *bt))}
    }

}

impl From<&str> for OrbitRelationship {

    fn from(input: &str) -> OrbitRelationship {
        let (orbitee, orbiter) = input.split(')').next_tuple().unwrap();
        OrbitRelationship { orbitee: orbitee.into(), orbiter: orbiter.into() }
    }

}


fn hash_b(idx: u32, bt: u8) -> u32 {
    (100 as u32).pow(idx) * (bt as u32)
}



#[aoc_generator(day6)]
pub fn day6_gerator(input: &str) -> Vec<OrbitRelationship> {
    input.lines().map(|line| OrbitRelationship::from(line)).collect()
}

#[aoc(day6, part1)]
pub fn day6_1(relationships: &[OrbitRelationship]) -> u32 {
    let mut distances = HashMap::new();
    for rel in relationships {
        depths(&relationships, &mut distances, &rel.orbiter);
    }
    distances.values().fold(0, |a, b| a + b)
}

pub fn depths(relationships: &[OrbitRelationship], mut distances: &mut HashMap<Orb, u32>, value: &Orb) -> u32 {
    if let Some(val) = distances.get(value) {
        return *val;
    }

    let mut total = 1;
    for rel in relationships {
        if rel.orbitee.eq(value) {
            total += depths(&relationships, &mut distances, &rel.orbiter);
        }
    }
    distances.insert(*value, total);
    total
}