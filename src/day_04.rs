use std::collections::HashMap;

const BINGO_SIZE: usize = 5;

#[derive(Debug)]
enum ReaderState {
    Barrels,
    Card,
}

struct BingoCard {
    rows: Vec<BingoLine>,
    cols: Vec<BingoLine>,
}

impl BingoCard {
    pub fn from(input: Vec<Vec<u32>>) -> BingoCard {
        let mut rows = Vec::new();
        let mut cols = Vec::new();
        let mut xxx = [[0; BINGO_SIZE]; BINGO_SIZE];
        let mut yyy = [[0; BINGO_SIZE]; BINGO_SIZE];

        for i in 0..BINGO_SIZE {
            for j in 0..BINGO_SIZE {
                let number = input[i][j];
                xxx[i][j] = number;
                yyy[j][i] = number;
            }
        }

        for xx in xxx {
            rows.push(BingoLine::from(xx));
        }

        for yy in yyy {
            cols.push(BingoLine::from(yy));
        }

        BingoCard {
            rows,
            cols,
        }
    }

    pub fn verify_number(&mut self, barrel: u32) -> bool {
        let mut won = false;

        for row in &mut self.rows {
            if row.verify_number(barrel) {
                won = true;
            }
        }

        for col in &mut self.cols {
            if col.verify_number(barrel) {
                won = true;
            }
        }

        won
    }

    pub fn sum_unmarked(&self) -> u32 {
        let mut total = 0;
        for x in &self.rows {
            total += x.sum_unmarked();
        }
        total
    }
}

struct BingoLine {
    numbers: HashMap<u32, bool>,
    matched: usize,
}

impl BingoLine {
    pub fn from(xx: [u32; BINGO_SIZE]) -> BingoLine {
        BingoLine {
            numbers: HashMap::from(xx.map(|x| (x, false))),
            matched: 0,
        }
    }

    pub fn verify_number(&mut self, barrel: u32) -> bool {
        match self.numbers.get(&barrel) {
            Some(true) => {
                panic!("number repeated {}", barrel);
            }
            Some(false) => {
                self.numbers.insert(barrel, true);
                self.matched += 1;
                // println!("{} matched {}/{} at {:?}", barrel, self.matched, BINGO_SIZE, self.numbers);
            }
            None => {}
        }
        return self.matched == BINGO_SIZE;
    }

    pub fn sum_unmarked(&self) -> u32 {
        let mut total = 0;
        for (x, is_marked) in &self.numbers {
            if !is_marked {
                total += x;
            }
        }
        total
    }
}

pub fn solve_4_1(raw_input: &[String]) -> u32 {
    let (barrels, mut cards) = read_bingo_game(raw_input);

    // println!("barrels:{:?}", barrels);

    let mut called_barrels: HashMap<u32, bool> = HashMap::new();

    for barrel in barrels {
        assert!(!called_barrels.contains_key(&barrel));
        // println!("BARREL: {}", barrel);
        called_barrels.insert(barrel, true);
        for card in &mut cards {
            if card.verify_number(barrel) {
                let total = card.sum_unmarked();
                // println!("{} WON, sum={}", barrel, total);
                return total * barrel;
            }
        }
    }
    0
}


fn read_bingo_game(raw_input: &[String]) -> (Vec<u32>, Vec<BingoCard>) {
    let mut reader_state = ReaderState::Barrels;
    let mut barrels: Vec<u32> = Vec::new();
    let mut cards = Vec::new();
    let mut card_rows: Vec<Vec<u32>> = Vec::new();

    for line in raw_input {
        // println!("{:?}, {}", reader_state, line);
        match reader_state {
            ReaderState::Barrels => {
                if line.len() == 0 {
                    reader_state = ReaderState::Card;
                } else {
                    barrels = line.split(',')
                        .map(|x| x.parse().unwrap())
                        .collect();
                }
            }

            ReaderState::Card => {
                if line.len() == 0 {
                    cards.push(BingoCard::from(card_rows));
                    card_rows = Vec::new();
                } else {
                    let row: Vec<u32> = line.split(' ')
                        .filter(|x| !x.is_empty())
                        .map(|x| x.parse().unwrap())
                        .collect();

                    assert_eq!(row.len(), BINGO_SIZE);
                    assert!(card_rows.len() < BINGO_SIZE);

                    card_rows.push(row);
                }
            }
        }
    };
    (barrels, cards)
}


#[cfg(test)]
mod tests {
    use crate::{make_file_name, read_input};

    use super::*;

    #[test]
    fn day_4() {
        let test_data = read_input(make_file_name(true, 4, 1));
        assert_eq!(solve_4_1(&test_data), 4512);
    }
}

