use std::collections::HashMap;

const OPENING: [(char, char); 4] = [(')', '('), (']', '['), ('}', '{'), ('>', '<')];
const POINTS: [(char, u32); 4] = [(')', 3), (']', 57), ('}', 1197), ('>', 25137)];
const SCORE: [(char, u64); 4] = [('(', 1), ('[', 2), ('{', 3), ('<', 4)];

struct Validator {
    opening: HashMap<char, char>,
    points: HashMap<char, u32>,
    score: HashMap<char, u64>,
    last_points: u32,
    last_stack: Vec<char>,
}


impl Validator {
    pub fn new() -> Validator {
        Validator {
            opening: HashMap::from(OPENING),
            points: HashMap::from(POINTS),
            score: HashMap::from(SCORE),
            last_points: 0,
            last_stack: vec![],
        }
    }

    pub fn validate(&mut self, line: &String) -> bool {
        let mut stack = Vec::new();
        for cur in line.chars() {
            if let Some(pair) = self.opening.get(&cur) {
                if let Some(prev) = stack.pop() {
                    if prev != *pair {
                        self.last_points = *self.points.get(&cur).unwrap();
                        self.last_stack = stack;
                        return false;
                    }
                } else {
                    self.last_points = *self.points.get(&cur).unwrap();
                    self.last_stack = stack;
                    return false;
                }
            } else {
                stack.push(cur);
            }
        }
        self.last_points = 0;
        self.last_stack = stack;
        true
    }

    pub fn score_autocomplete(&mut self) -> u64 {
        let mut out = 0;
        while self.last_stack.len() > 0 {
            if let Some(pair) = self.last_stack.pop() {
                out = out * 5 + *self.score.get(&pair).unwrap();
            }
        }
        out
    }
}

pub fn solve_10_1(raw_input: &[String]) -> u32 {
    let mut out = 0;
    let mut validator = Validator::new();
    for line in raw_input {
        if line.len() == 0 {
            continue;
        }
        validator.validate(line);
        out += validator.last_points;
    }
    out
}

pub fn solve_10_2(raw_input: &[String]) -> u32 {
    let mut validator = Validator::new();

    let mut scores = vec![];
    for line in raw_input {
        if line.len() == 0 || !validator.validate(line) {
            continue;
        }
        scores.push(validator.score_autocomplete());
    }
    scores.sort();
    scores[scores.len() / 2] as u32
}


#[cfg(test)]
mod tests {
    use crate::{make_file_name, read_input};

    use super::*;

    #[test]
    fn day_10() {
        let test_data = read_input(make_file_name(true, 10, 1));
        assert_eq!(solve_10_1(&test_data), 26397);
        assert_eq!(solve_10_2(&test_data), 288957);
    }
}
