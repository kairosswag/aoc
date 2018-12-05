extern crate regex;
use std::collections::HashMap;
use std::cmp::Ordering;

#[derive(new, Debug, Clone, Copy)]
pub struct Timestamp {
    year: u32,
    month: u32,
    day: u32,
    hour: u32,
    min: u32,
    event: Event,
}

#[derive(Debug, Clone, Copy)]
pub enum Event {
    Guard(u32),
    Wake,
    Sleep,
}

impl Ord for Timestamp {
    fn cmp(&self, other: &Timestamp) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl PartialOrd for Timestamp {
    fn partial_cmp(&self, other: &Timestamp) -> Option<Ordering> {
        Some((self.year, self.month, self.day, self.hour, self.min).cmp(&(other.year, other.month, other.day, other.hour, other.min)))
    }
}

impl Eq for Timestamp {
    
}

impl PartialEq for Timestamp {
    fn eq(&self, other: &Timestamp) -> bool {
        self.year == other.year && self.month == other.month && self.day == other.day && self.hour == other.hour && self.min == other.min
    }
}


#[aoc_generator(day4)]
pub fn day3_input_gerator(input: &str) -> Vec<Timestamp> {
    let re = regex::Regex::new(r"\[(\d*)-(\d*)-(\d*) (\d*):(\d*)\]([^#]*)#?(\d*).*").unwrap();
    let mut timestamps : Vec<Timestamp> = input.lines().map(|line| {
        for cap in re.captures_iter(line) {
             return Timestamp::new(
                cap[1].parse::<u32>().expect(&format!("id: {}", &cap[0])),
                cap[2].parse::<u32>().expect(&format!("xpos: {}", &cap[0])),
                cap[3].parse::<u32>().expect(&format!("ypos: {}", &cap[0])),
                cap[4].parse::<u32>().expect(&format!("width: {}", &cap[0])),
                cap[5].parse::<u32>().expect(&format!("height: {}", &cap[0])),
                match cap[6].trim().as_ref() {
                    "wakes up" => Event::Wake,
                    "falls asleep" => Event::Sleep,
                    x => Event::Guard(cap[7].parse::<u32>().expect(&("7 went wrong ".to_owned() + x)))
                }
             );
        }
        println!("{}", line);
        unimplemented!();
    }).collect();
    timestamps.sort();
    timestamps
}


#[aoc(day4, part1)]
pub fn part1(protocol: &[Timestamp]) -> u32 {
    let mut schedule = HashMap::new();
    let mut total = HashMap::new();

    let mut guard = 0;
    let mut curr_timest = None;
    
    for &timest in protocol {
        match timest.event {
            Event::Sleep => {
                let times = schedule.entry(guard).or_insert(Vec::new());
                times.push(timest);
                curr_timest = Some(timest);
            }, 
            Event::Wake => {
                let times = schedule.entry(guard).or_insert(Vec::new());
                times.push(timest);
                let cnum = total.entry(guard).or_insert(0);
                *cnum += timest.min - curr_timest.expect("Wake before sleep?").min;
            },
            Event::Guard(guard_id) => {
                guard = guard_id;
            }
        }
    }

    if let Some((k, v)) = total.iter().max_by_key(|&(k, v)| v) {
        let mut guard_sched = schedule.get_mut(k).expect("Noes");
        let mut minutes = vec![0; 60];
        guard_sched.sort();
        for idx in 0..guard_sched.len() / 2 {
            for t_min in guard_sched[2*idx].min..guard_sched[2*idx+1].min {
                minutes[t_min as usize] += 1;
            }
        }
        println!("key: {}, value: {}", k, v);
        if let Some((val, idx)) = minutes.iter().zip(0..).max_by_key(|&(val, idx)| val) {
            // println!("minutes: {} @ {} ---- {:?}", idx, val, &minutes);
            return k * idx;
        }
    } 
    panic!("No Result");
}

#[aoc(day4, part2)]
pub fn part2(protocol: &[Timestamp]) -> u32 {
    let mut schedule = HashMap::new();
    // let mut total = HashMap::new();
    let mut minuteplans = HashMap::new();

    let mut guard = 0;
    let mut curr_timest = None;
    
    for &timest in protocol {
        match timest.event {
            Event::Sleep => {
                let times = schedule.entry(guard).or_insert(Vec::new());
                times.push(timest);
                curr_timest = Some(timest);
            }, 
            Event::Wake => {
                let times = schedule.entry(guard).or_insert(Vec::new());
                times.push(timest);
                let mut minuteplan : &mut [u32] = minuteplans.entry(guard).or_insert(vec![0;60]);
                for idx in curr_timest.unwrap().min..timest.min {
                    minuteplan[idx as usize] += 1;
                }
            },
            Event::Guard(guard_id) => {
                guard = guard_id;
            }
        }
    }
    if let Some((k, v)) = minuteplans.iter().max_by_key(|&(k, v)| v.iter().max().unwrap()) {
        let mut max_idx = 0;
        let mut max = 0;
        for i in 0..v.len() {
            if v[i] > max {
                max = v[i];
                max_idx = i;
            }
        }
        return (max_idx as u32) * k;
    }
    panic!("No result found!");
}

#[cfg(test)]
pub mod test {
    #[test]
    fn regex_test() {
        unimplemented!();
    }
}