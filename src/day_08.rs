use std::collections::{HashMap};

//   a 64 a
// b        c
// 32      16
// b        c
//   d  8 d
// e        f
// 4        2
// e        f
//   g  1 g

const S_A: u8 = 1 << 6;
const S_B: u8 = 1 << 5;
const S_C: u8 = 1 << 4;
const S_D: u8 = 1 << 3;
const S_E: u8 = 1 << 2;
const S_F: u8 = 1 << 1;
const S_G: u8 = 1;
const S_Z: u8 = 0b1111111;

const SIGNAL_CHARS: [char; 7] = ['A', 'B', 'C', 'D', 'E', 'F', 'G'];

const PR_ALLOWED: [(usize, u8); 3] = [
    (2, S_B  + S_F ),
    (3, S_A  + S_C  + S_F ),
    (4, S_B  + S_C  + S_D  + S_F ),
];
const AB_ALLOWED: [(usize, u8); 5] = [
    (2, S_Z  - S_C  - S_F ),
    (3, S_Z  - S_A  - S_C  - S_F ),
    (4, S_Z  - S_B  - S_C  - S_D  - S_F ),
    (5, S_Z  - S_A  - S_D  - S_G ),
    (6, S_Z  - S_A  - S_B  - S_F  - S_G ),
];

const DISPLAY_NUMBER: [(u8, u8); 10] = [
    (S_A  + S_B  + S_C  + S_D  + S_E  + S_F  + S_G , 8),
    (S_A  + S_B  + S_C  + S_D  + S_F  + S_G , 9),
    (S_A  + S_B  + S_C  + S_E  + S_F  + S_G , 0),
    (S_A  + S_B  + S_D  + S_E  + S_F  + S_G , 6),
    (S_A  + S_B  + S_D  + S_F  + S_G , 5),
    (S_A  + S_C  + S_D  + S_E  + S_G , 2),
    (S_A  + S_C  + S_D  + S_F  + S_G , 3),
    (S_A  + S_C  + S_F , 7),
    (S_B  + S_C  + S_D  + S_F , 4),
    (S_C  + S_F , 1)
];

pub fn solve_8_1(raw_input: &[String]) -> u32 {
    let mut total = 0;
    for line in raw_input {
        if line.len() == 0 { continue; }

        let mut is_after_pipe = false;
        let mut counter: usize = 0;
        for x in line.chars() {
            total += match x {
                '|' => {
                    is_after_pipe = true;
                    0
                }
                ' ' => {
                    let inc = if is_after_pipe && is_1478(counter) { 1 } else { 0 };
                    counter = 0;
                    inc
                }
                _ => {
                    counter += if is_after_pipe { 1 } else { 0 };
                    0
                }
            }
        }
        if is_1478(counter) {
            total += 1;
        }
    }
    total
}

pub fn solve_8_2(raw_input: &[String]) -> u32 {
    let mut _total = 0;
    println!("numbers {:?}", DISPLAY_NUMBER);
    for line in raw_input {
        if line.len() == 0 { continue; }
        _total += solve_line(line);
    }
    0
}

fn is_1478(n: usize) -> bool { n > 1 && n != 5 && n != 6 }


fn solve_line(line: &str) -> u32 {
    let mut signals: Vec<String> = line.split(' ')
        .filter(|x| x.len() > 1 && x.len() < 7)
        .map(|x| String::from(x))
        .collect();
    signals.sort_by(|a, b| a.len().partial_cmp(&b.len()).unwrap());
    let _map = map_signals(&signals);
    0
}

fn map_signals(signals: &[String]) -> HashMap<char, char> {
    let out = HashMap::new();
    println!("{:?}", signals);
    let pr_allowed = HashMap::from(PR_ALLOWED);
    let ab_allowed = HashMap::from(AB_ALLOWED);
    let mut left = HashMap::from(SIGNAL_CHARS.map(|x| (x, S_Z )));
    let mut _right = HashMap::from(SIGNAL_CHARS.map(|x| (x, S_Z )));
    for signal in signals {
        let n = signal.len();
        for c in SIGNAL_CHARS {
            if let Some(allowed) = { if let Some(_) = signal.find(c) { &pr_allowed } else { &ab_allowed } }.get(&n) {
                println!("signal {}, size {}, char {}, allowed {:b}", signal, n, c, allowed);
                let new_value = left.get(&c).unwrap() & allowed;
                left.insert(c, new_value);
            }
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use crate::{make_file_name, read_input};

    use super::*;

    #[test]
    fn day_8() {
        let test_data = read_input(make_file_name(true, 8, 1));
        assert_eq!(solve_8_1(&test_data), 26);
        // assert_eq!(solve_8_2(&[String::from("acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf")]), 5353);
        // assert_eq!(solve_8_2(&test_data), 61229);
    }
}
