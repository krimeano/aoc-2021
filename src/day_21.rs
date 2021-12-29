const DIE_MAX: u8 = 100;
const TRACK_LENGTH: u8 = 10;
const ROLL: u32 = 3;
const TARGET: u32 = 1000;

pub fn solve_21_1(input: [u8; 2]) -> u32 {
    let mut die = Die::new();
    let mut player_ix: usize = 1;
    let mut positions = input.clone();
    let mut score = [0, 0];

    while score[player_ix] < TARGET {
        player_ix = 1 - player_ix;
        print!("Player {}: ", player_ix + 1);
        positions[player_ix] = turn(positions[player_ix], &mut die);
        score[player_ix] += positions[player_ix] as u32;
        println!("{:?} {:?}", positions, score);
    }
    let loser_ix = 1 - player_ix;
    println!("Player {} has score {}, and die rolled {} times", loser_ix + 1, score[loser_ix], die.rolled);
    score[loser_ix] * die.rolled
}

pub fn solve_21_2(_input: [u8; 2]) -> u32 { 0 }

fn turn(from_pos: u8, die: &mut Die) -> u8 {
    let mut steps = 0;
    print!("roll ");
    for _ in 0..ROLL {
        if let Some(x) = die.next() {
            print!("{} ", x);
            steps = (steps + x) % TRACK_LENGTH;
        }
    }
    print!("= {} ", steps);
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


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day_21() {
        assert_eq!(solve_21_1([4, 8]), 739785);
        assert_eq!(solve_21_2([4, 8]), 0);
    }
}
