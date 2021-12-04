const BINGO_SIZE: usize = 5;

#[derive(Debug)]
enum ReaderState {
    Barrels,
    Card,
}

struct BingoLine {
    numbers: [u8; BINGO_SIZE],
    matched: usize,
}

pub fn solve_4_1(raw_input: &[String]) -> u32 {
    let (barrels, rows, cols) = read_bingo_game(raw_input);
    println!("barrels:{:?}", barrels);
    for xx in rows {
        println!("row {:?}", xx.numbers);
    }

    for yy in cols {
        println!("col {:?}", yy.numbers);
    }

    0
}


fn read_bingo_game(raw_input: &[String]) -> (Vec<u8>, Vec<BingoLine>, Vec<BingoLine>) {
    let mut reader_state = ReaderState::Barrels;
    let mut barrels: Vec<u8> = Vec::new();
    let mut card_rows: Vec<Vec<u8>> = Vec::new();

    let mut out_rows = Vec::new();
    let mut out_cols = Vec::new();

    for line in raw_input {
        // println!("{:?}, {}", reader_state, line);
        match reader_state {
            ReaderState::Barrels => {
                if line.len() == 0 {
                    reader_state = ReaderState::Card;
                } else {
                    barrels = line.split(',').map(|x| x.parse().unwrap()).collect();
                }
            }
            ReaderState::Card => {
                if line.len() == 0 {
                    let mut xxx = [[0; BINGO_SIZE]; BINGO_SIZE];
                    let mut yyy = [[0; BINGO_SIZE]; BINGO_SIZE];
                    for i in 0..BINGO_SIZE {
                        for j in 0..BINGO_SIZE {
                            xxx[i][j] = card_rows[i][j];
                            yyy[j][i] = card_rows[i][j];
                        }
                    }

                    for xx in xxx {
                        out_rows.push(BingoLine {
                            matched: 0,
                            numbers: xx,
                        });
                    }
                    for yy in yyy {
                        out_cols.push(BingoLine {
                            matched: 0,
                            numbers: yy,
                        });
                    }

                    card_rows = Vec::new();
                } else {
                    let row: Vec<u8> = line.split(' ')
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
    (barrels, out_rows, out_cols)
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

