

#[aoc_generator(day7)]
pub fn day7_input_generator(input: &str) -> Vec<(char, char)> {
    input.lines().map(str::trim).map(|line| {
        let mut arr = line.chars().skip(5);
        let first = arr.next();
        let second = arr.skip(30).next();
        return (first.unwrap(), second.unwrap());
        
    }).collect()
}

#[aoc(day7, part1)]
pub fn part1(input: &[(char, char)]) -> String {
    let mut res: Vec<char> = Vec::new();
    let mut xin: Vec<&(char, char)> = input.iter().collect();
    let mut last = ' ';
    while xin.len() > 0 {
        let mut cand = Vec::new();  
        'tuples: for i in 0..xin.len() {
            for (_, e2) in &xin {
                if &xin[i].0 == e2 {
                    continue 'tuples;
                }
            }
            cand.push((i, xin[i]));
        }
        if let Some(min_char) = cand.iter().min_by(|(_,(k1, _)), (_,(k2, _))| k1.cmp(&k2)) {
            xin.retain(|(k, _)| k != &(min_char.1).0);
            res.push((min_char.1).0);
            last = (min_char.1).1;
        }
    }
    res.push(last);
    res.iter().collect()
}

#[derive(Debug)]
pub struct Worker {
    pub id : u32,
    pub workload : Option<char>,
    pub eta : u32,
}

impl Worker {

    pub fn new(id : u32) -> Worker {
        Worker { id: id, workload: None, eta: 0 }
    }

    pub fn progress(&mut self, elapsed: u32) -> Option<char> {
        if self.eta == elapsed {
            let worked = self.workload;
            self.complete();
            return worked;
        } else if self.eta != 0 {
            self.eta -= elapsed;
        }
        return None;
    }

    pub fn complete(&mut self) -> u32 {
        let elapsing = self.eta;
        self.workload = None;
        self.eta = 0;
        elapsing
    }

}

#[aoc(day7, part2)]
pub fn part2(input: &[(char, char)]) -> u32 {
    let mut xin: Vec<&(char, char)> = input.iter().collect();
    let mut total_time = 0;
    let mut workers = Vec::new();
    (0..5).for_each(|i| workers.push(Worker::new(i)));
    let mut candidates = Vec::new();
    let mut recalc_candidates = true;
    let mut last_char = None;
    while xin.len() > 0 {
        if xin.len() == 1 {
            last_char = Some(xin[0].1);
        }
        if recalc_candidates {
            recalc_candidates = false;
            candidates = Vec::new();
            // Calculate candidates
            'tuples:
            for i in 0..xin.len() {
                for (_, target) in &xin {
                    if &xin[i].0 == target {
                        continue 'tuples;
                    }
                }
                candidates.push((i, xin[i]));
            }
            candidates.sort_by(|(_, (c1, _)), (_, (c2, _))| c1.cmp(&c2));
        }

        {
            // Worker ready, candidate available -> Run Candidate on Worker
            let next_work = candidates.iter().filter(|(_, (k, _))| !workers.iter().any(|w| w.workload.map_or(false, |wl| &wl == k))).map(|(_, (k,_))| k).next();
            let free_worker = workers.iter_mut().filter(|w| w.workload == None).next();
            if let (Some(ref mut worker), Some(work)) = (free_worker, next_work) {
                    worker.workload = Some(*work);
                    worker.eta = *work as u32 - 4;
                    continue;
            }
        }

        // Worker ready, no candidate available -> Finish fastest
        // No worker ready, candidate available -> Finish fastest
        // No worker ready, no candidate available -> Finish fastest
        {
            let (elapsed, done) = {
                let fastest = workers.iter().filter(|w| w.workload.is_some()).min_by(|w1, w2| w1.eta.cmp(&w2.eta)).unwrap();
                (fastest.eta, fastest.workload.unwrap())
            };
            total_time += elapsed;
            // println!("Workers: {:?}", &workers);
            assert!(elapsed > 0);
            for finished in workers.iter_mut().map(|w| w.progress(elapsed)).filter(|o| o.is_some()) {
                xin.retain(|(k, _)| k != &finished.unwrap());
            }
            recalc_candidates = true;
        }
    }
    total_time + last_char.unwrap() as u32 - 4
}

#[test]
fn test1() {
    assert_eq!(day7_input_generator("Step C must be finished before step A can begin.")[0], ('C', 'A'));
}


#[test]
fn test2() {
    let input = "Step C must be finished before step A can begin.
Step C must be finished before step F can begin.
Step A must be finished before step B can begin.
Step A must be finished before step D can begin.
Step B must be finished before step E can begin.
Step D must be finished before step E can begin.
Step F must be finished before step E can begin.";
    let mthd = day7_input_generator(&input);
    assert_eq!(part2(&mthd), 15);
}