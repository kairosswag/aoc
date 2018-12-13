

#[derive(new, Clone)]
struct Marble {
    pub next: Option<usize>,
    pub prev: Option<usize>,
}

struct Arena {
    curr_marble_idx: usize,
    marbles: Vec<Marble>,
    // circle: Vec<usize>,
}

impl Arena {
    pub fn init(maaaaarbles: usize) -> Arena {
        let mut marbles = Vec::with_capacity(maaaaarbles);        
        (0..maaaaarbles).for_each(|_| marbles.push(Marble::new(None, None)));
        marbles[0].next = Some(0);
        marbles[0].prev = Some(0);
        Arena { curr_marble_idx: 0, marbles: marbles }
    }

    pub fn insert_marble(&mut self, marble_id: usize) {

        let curr_marble = &self.marbles[self.curr_marble_idx];
        self.curr_marble_idx = marble_id;
        let prev_marb_idx = curr_marble.next.unwrap();
        let prev_marb = &mut self.marbles[prev_marb_idx];
     
        let afte_marb_idx = prev_marb.next.unwrap();
        prev_marb.next.replace(marble_id);

        let afte_marb = &mut self.marbles[afte_marb_idx];
        afte_marb.prev = Some(marble_id);

        let new_marb = &mut self.marbles[marble_id];
        new_marb.prev = Some(prev_marb_idx);
        new_marb.next = Some(afte_marb_idx);

    }

    pub fn remove_marble_from(&mut self) -> usize {
        let curr = &self.marbles[self.curr_marble_idx];
        let one_cc = &self.marbles[curr.prev.unwrap()];
        let two_cc = &self.marbles[one_cc.prev.unwrap()];
        let thr_cc = &self.marbles[two_cc.prev.unwrap()];
        let fou_cc = &self.marbles[thr_cc.prev.unwrap()];
        let fiv_cc = &self.marbles[fou_cc.prev.unwrap()];
        let six_idx = fiv_cc.prev.unwrap();
        self.curr_marble_idx = six_idx;
        let six_cc = &self.marbles[six_idx];
        let sev_idx = six_cc.prev.unwrap();
        let sev_cc = &self.marbles[sev_idx];
        let eig_idx = sev_cc.prev.unwrap();
        self.marbles[six_idx].prev = Some(eig_idx);
        self.marbles[eig_idx].next = Some(six_idx);

        sev_idx
    }
}



#[aoc(day9, part1)]
pub fn part1(_input: &str) -> usize {
    let max_marble = 71790;
    let players = 459;
    let mut arena = Arena::init(max_marble + 1);
    let mut player_scores = vec![0; 459];
    for i in 1..= max_marble {
        if i % 23 == 0 && i != 0 {
            player_scores[i % players] += i;
            player_scores[i % players] += arena.remove_marble_from();
        } else {
            arena.insert_marble(i);
        }
    }

    *player_scores.iter().max().unwrap()
} 

#[aoc(day9, part2)]
pub fn part2(_input: &str) -> usize {
    let max_marble = 71790 * 100;
    let players = 459;
    let mut arena = Arena::init(max_marble + 1);
    let mut player_scores = vec![0; 459];
    for i in 1..= max_marble {
        if i % 23 == 0 && i != 0 {
            player_scores[i % players] += i;
            player_scores[i % players] += arena.remove_marble_from();
        } else {
            arena.insert_marble(i);
        }
    }

    *player_scores.iter().max().unwrap()
} 