use std::cmp::max;
use std::collections::HashMap;

const DIE_MAX: u8 = 100;
const TRACK_LENGTH: u8 = 10;
const ROLL: u32 = 3;
const TARGET: u32 = 1000;
const TARGET_2: u8 = 21;

pub fn solve_21_1(input: [u8; 2]) -> u32 {
    let mut die = Die::new();
    let mut player_ix: usize = 1;
    let mut positions = input.clone();
    let mut score = [0, 0];

    while score[player_ix] < TARGET {
        player_ix = 1 - player_ix;
        positions[player_ix] = turn(positions[player_ix], &mut die);
        score[player_ix] += positions[player_ix] as u32;
    }
    let loser_ix = 1 - player_ix;
    score[loser_ix] * die.rolled
}

pub fn solve_21_2(input: [u8; 2]) -> u64 {
    let dirac = get_dirac_die_outcomes();
    let results: [(u8, u8); 2] = [(0, input[0]), (0, input[1])];
    let mut wins: [u64; 2] = [0, 0];
    let mut outcomes: HashMap<[(u8, u8); 2], u64> = HashMap::from([(results, 1)]);
    let mut player_ix: usize = 1;
    while outcomes.len() > 0 {
        player_ix = 1 - player_ix;
        let mut new_outcomes = HashMap::new();
        for (results, m) in outcomes {
            for (steps, n) in dirac.clone() {
                let pos = (results[player_ix].1 - 1 + steps) % TRACK_LENGTH + 1;
                let score = results[player_ix].0 + pos;
                let total = m * n;
                if score < TARGET_2 {
                    let mut new_result = results.clone();
                    new_result[player_ix] = (score, pos);
                    *new_outcomes.entry(new_result).or_insert(0) += total;
                } else {
                    wins[player_ix] += total;
                }
            }
        }
        outcomes = new_outcomes;
    }

    max(wins[0], wins[1])
}

fn turn(from_pos: u8, die: &mut Die) -> u8 {
    let mut steps = 0;
    for _ in 0..ROLL {
        if let Some(x) = die.next() {
            steps = (steps + x) % TRACK_LENGTH;
        }
    }
    (from_pos - 1 + steps) % TRACK_LENGTH + 1
}

struct Die {
    curr: u8,
    rolled: u32,
}

impl Die {
    fn new() -> Self { Self { curr: DIE_MAX, rolled: 0 } }
}

impl Iterator for Die {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        self.rolled += 1;
        self.curr = self.curr % DIE_MAX + 1;
        Some(self.curr)
    }
}

fn get_dirac_die_outcomes() -> HashMap<u8, u64> {
    let mut out = HashMap::new();
    for ix in 0..3 {
        for jy in 0..3 {
            for kz in 0..3 {
                let a = 3 + ix + jy + kz;
                *out.entry(a).or_insert(0) += 1;
            }
        }
    }
    out
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day_21() {
        assert_eq!(solve_21_1([4, 8]), 739785);
        assert_eq!(solve_21_2([4, 8]), 444356092776315);
    }
}
