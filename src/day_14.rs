use std::collections::HashMap;

struct Solution {
    template: Vec<char>,
    rules: HashMap<(char, char), Vec<(char, char)>>,
    total_pairs: HashMap<(char, char), u64>,
}

impl Solution {
    pub fn from(raw_input: &[String]) -> Solution {
        let mut template = Vec::new();
        let mut rules = HashMap::new();

        for line in raw_input {
            if line.len() == 0 {
                continue;
            }

            if template.len() == 0 {
                template = line.chars().collect();
                continue;
            }

            let mut pair = (' ', ' ');
            let mut prod = ' ';

            for x in line.split(" -> ") {
                if pair.0 != ' ' && prod != ' ' {
                    panic!();
                }

                let mut lc = x.chars();

                if pair.0 == ' ' {
                    if let Some(a) = lc.next() {
                        if let Some(b) = lc.next() {
                            pair = (a, b);
                            if let Some(c) = lc.next() {
                                panic!("found '{}' at '{}'", c, x);
                            }
                        } else {
                            panic!();
                        }
                    } else {
                        panic!();
                    }
                } else {
                    if let Some(a) = lc.next() {
                        prod = a;
                        if let Some(c) = lc.next() {
                            panic!("found '{}' at {}", c, x);
                        }
                    }
                }
            }
            rules.insert(pair, vec![(pair.0, prod), (prod, pair.1)]);
        }

        if template.len() == 0 {
            panic!()
        }

        let mut total_pairs = HashMap::new();
        let mut prev = None;
        for x in &template {
            if let Some(w) = prev {
                *total_pairs.entry((w, *x)).or_insert(0) += 1;
            }
            prev = Some(*x);
        }


        Solution {
            template,
            rules,
            total_pairs,
        }
    }

    pub fn solve(&mut self, steps: u8) -> u64 {
        for _ in 0..steps {
            self.iterate();
        }
        let result = self.result();
        result[0].1 - result[result.len() - 1].1
    }

    fn iterate(&mut self) {
        let (inc_counter, dec_counter) = self.calculate();

        for (x, n) in inc_counter {
            *self.total_pairs.entry(x).or_insert(0) += n;
        }

        for (x, n) in dec_counter {
            *self.total_pairs.entry(x).or_insert(0) -= n;
        }
    }

    fn calculate(&self) -> (HashMap<(char, char), u64>, HashMap<(char, char), u64>) {
        let mut inc_counter = HashMap::new();
        let mut dec_counter = HashMap::new();
        for (pair, n) in &self.total_pairs {
            if let Some(xx) = self.rules.get(pair) {
                for x in xx {
                    *inc_counter.entry(*x).or_insert(0) += n;
                }
                dec_counter.insert(*pair, *n);
            }
        }
        (inc_counter, dec_counter)
    }

    fn result(&self) -> Vec<(char, u64)> {
        let mut out = Vec::new();
        let mut letters = HashMap::from([
            (self.template[0], 1),
            (self.template[self.template.len() - 1], 1),
        ]);
        for (pair, n) in &self.total_pairs {
            *letters.entry(pair.0).or_insert(0) += *n;
            *letters.entry(pair.1).or_insert(0) += *n;
        }
        for (c, n) in letters {
            out.push((c, n / 2));
        }
        out.sort_by(|a, b| b.1.cmp(&a.1));
        out
    }
}

pub fn solve_14_1(raw_input: &[String]) -> u64 { Solution::from(raw_input).solve(10) }

pub fn solve_14_2(raw_input: &[String]) -> u64 { Solution::from(raw_input).solve(40) }

#[cfg(test)]
mod tests {
    use crate::{make_file_name, read_input};

    use super::*;

    #[test]
    fn day_14() {
        let test_data = read_input(make_file_name(true, 14, 1));
        assert_eq!(solve_14_1(&test_data), 1588);
        assert_eq!(solve_14_2(&test_data), 2188189693529);
    }
}
