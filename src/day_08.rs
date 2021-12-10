use std::collections::{HashMap, HashSet};

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

const SIGNAL_CHARS: [char; 7] = ['a', 'b', 'c', 'd', 'e', 'f', 'g'];

const DISPLAY_NUMBER: [(u8, u8); 10] = [
    (S_A + S_B + S_C + S_D + S_E + S_F + S_G, 8),
    (S_A + S_B + S_C + S_D + S_F + S_G, 9),
    (S_A + S_B + S_C + S_E + S_F + S_G, 0),
    (S_A + S_B + S_D + S_E + S_F + S_G, 6),
    (S_A + S_B + S_D + S_F + S_G, 5),
    (S_A + S_C + S_D + S_E + S_G, 2),
    (S_A + S_C + S_D + S_F + S_G, 3),
    (S_A + S_C + S_F, 7),
    (S_B + S_C + S_D + S_F, 4),
    (S_C + S_F, 1)
];

const LETTER_TO_VALUE: [(char, u8); 7] = [('a', S_A), ('b', S_B), ('c', S_C), ('d', S_D), ('e', S_E), ('f', S_F), ('g', S_G)];


struct Display {
    present_allowed: HashMap<usize, Vec<char>>,
    absent_denied: HashMap<usize, Vec<char>>,
    letter_to_value: HashMap<char, u8>,

    cleats_in: HashMap<char, Cleat>,
    cleats_out: HashMap<char, Cleat>,
}

impl Display {
    pub fn new() -> Display {
        let present_allowed = HashMap::from([
            (2, vec!['c', 'f']),
            (3, vec!['a', 'c', 'f']),
            (4, vec!['b', 'c', 'd', 'f']),
        ]);

        let absent_denied = HashMap::from([
            (2, vec!['c', 'f']),
            (3, vec!['a', 'c', 'f']),
            (4, vec!['b', 'c', 'd', 'f']),
            (5, vec!['a', 'd', 'g']),
            (6, vec!['a', 'b', 'f', 'g']),
        ]);

        let letter_to_value = HashMap::from(LETTER_TO_VALUE);

        let mut cleats_in: HashMap<char, Cleat> = HashMap::new();
        let mut cleats_out: HashMap<char, Cleat> = HashMap::new();

        for x in SIGNAL_CHARS {
            cleats_in.insert(x, Cleat::from(x));
            cleats_out.insert(x, Cleat::from(x));
        }

        Display {
            present_allowed,
            absent_denied,
            letter_to_value,

            cleats_in,
            cleats_out,
        }
    }

    pub fn tune(&mut self, txt_signal: &String) {
        let signal = Signal::from(txt_signal);
        let size = txt_signal.len();
        println!("{:?}: {:?}, !{:?}", txt_signal, signal.present, signal.absent);
        if let Some(allowed) = self.present_allowed.get(&size) {
            println!("present allowed {:?}", allowed);
        }

        if let Some(denied) = self.absent_denied.get(&size) {
            self.remove_denied(signal.absent, denied);
        } else {
            panic!("not expected!");
        }
    }

    fn remove_denied(&mut self, keys: HashSet<char>, denied: &[char]) {
        println!("removing {:?} for {:?}", denied, keys);
    }
}

struct Cleat {
    letter: char,
    possible: HashSet<char>,
    solved: Option<char>,
}

impl Cleat {
    pub fn from(letter: char) -> Cleat {
        Cleat {
            letter,
            possible: HashSet::from(SIGNAL_CHARS),
            solved: None,
        }
    }

    pub fn remove_denied(&mut self, d: &char) {
        if self.possible.contains(d) {
            self.possible.remove(d);
        }
    }
}

struct Signal {
    present: HashSet<char>,
    absent: HashSet<char>,
}

impl Signal {
    pub fn from(txt: &String) -> Signal {
        let mut absent = HashSet::from(SIGNAL_CHARS);
        let mut present = HashSet::new();
        for x in txt.chars() {
            present.insert(x);
            absent.remove(&x);
        }
        Signal { present, absent }
    }
}

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
    let mut total = 0;
    // println!("numbers {:?}", DISPLAY_NUMBER);
    for line in raw_input {
        if line.len() == 0 { continue; }
        total += solve_line(line);
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

    let mut display = Display::new();
    for x in signals {
        display.tune(&x);
    }

    // let _map = map_signals(&signals);
    0
}

// fn map_signals(signals: &[String]) -> HashMap<char, char> {
//     let out = HashMap::new();
//     println!("{:?}", signals);
//     let pr_allowed = HashMap::from(PR_ALLOWED);
//     let ab_allowed = HashMap::from(AB_ALLOWED);
//     let mut left = HashMap::from(SIGNAL_CHARS.map(|x| (x, S_Z)));
//     let mut _right = HashMap::from(SIGNAL_CHARS.map(|x| (x, S_Z)));
//     for signal in signals {
//         let n = signal.len();
//         for c in SIGNAL_CHARS {
//             if let Some(allowed) = { if let Some(_) = signal.find(c) { &pr_allowed } else { &ab_allowed } }.get(&n) {
//                 println!("signal {}, size {}, char {}, allowed {:b}", signal, n, c, allowed);
//                 let new_value = left.get(&c).unwrap() & allowed;
//                 left.insert(c, new_value);
//             }
//         }
//     }
//     out
// }

#[cfg(test)]
mod tests {
    use crate::{make_file_name, read_input};

    use super::*;

    #[test]
    fn day_8() {
        let test_data = read_input(make_file_name(true, 8, 1));
        assert_eq!(solve_8_1(&test_data), 26);
        assert_eq!(solve_8_2(&[String::from("acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf")]), 5353);
        // assert_eq!(solve_8_2(&test_data), 61229);
    }
}
