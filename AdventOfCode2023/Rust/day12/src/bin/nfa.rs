use std::collections::{HashMap, HashSet};

// NON-DETERMINISTIC FINITE AUTOMATA METHOD

#[derive(Debug, PartialEq)]
struct Nfa {
    q: HashSet<u32>,                           // set of states
    sigma: HashSet<char>,                      // set of symbols
    delta: HashMap<(u32, char), HashSet<u32>>, // set of transition relations
    s: HashSet<u32>,                           // set of initial states
    f: HashSet<u32>,                           // set of final states
}

impl Nfa {
    fn new(
        q: HashSet<u32>,
        sigma: HashSet<char>,
        delta: HashMap<(u32, char), HashSet<u32>>,
        s: HashSet<u32>,
        f: HashSet<u32>,
    ) -> Self {
        Self {
            q,
            sigma,
            delta,
            s,
            f,
        }
    }

    fn do_delta(&self, q: u32, x: char) -> HashSet<u32> {
        match self.delta.get(&(q, x)) {
            Some(set) => set.clone(),
            None => HashSet::new(),
        }
    }

    fn run(&self, word: &str) -> bool {
        let mut p = self.s.clone();
        for chr in word.chars() {
            let mut p_new: HashSet<_> = HashSet::new();
            for q in p {
                let next_states = self.do_delta(q, chr);
                p_new.extend(next_states);
            }
            p = p_new;
        }
        let intersection: HashSet<_> = self.f.intersection(&p).collect();
        !intersection.is_empty()
    }
}
