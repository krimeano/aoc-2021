use std::collections::VecDeque;

use crate::aoc_lib::get_neighbors;

pub fn solve_11_1(raw_input: &[String], is_verbose: bool) -> usize {
    Solution::from(raw_input).solve(is_verbose)
}

pub fn solve_11_2(raw_input: &[String], is_verbose: bool) -> usize {
    Solution::from(raw_input).solve_2(is_verbose)
}

fn print_dumbos(dumbos: &[Vec<u8>]) {
    for line in dumbos {
        println!();
        for d in line {
            if *d > 0 {
                print!("{}", d)
            } else {
                print!("\x1b[93m\x1b[1m{}\x1b[0m", d);
            }
        }
    }
    println!();
}

struct Solution {
    dumbos: Vec<Vec<u8>>,
    size: (usize, usize),
    total: usize,
}

impl Solution {
    pub fn from(raw_input: &[String]) -> Solution {
        let dumbos: Vec<Vec<u8>> = raw_input.iter()
            .filter(|x| x.len() > 0)
            .map(|x| x.chars()
                .map(|y| y as u8 - '0' as u8)
                .collect()
            )
            .collect();

        let size = (dumbos.len(), dumbos[0].len());

        Solution {
            dumbos,
            size,
            total: 0,
        }
    }

    pub fn solve(&mut self, is_verbose: bool) -> usize {
        if is_verbose {
            print_dumbos(&self.dumbos);
        }


        for _ in 0..100 {
            self.total += self.iterate();
            if is_verbose {
                print_dumbos(&self.dumbos);
                println!("total {}", self.total);
            }
        }
        self.total
    }

    pub fn solve_2(&mut self, is_verbose: bool) -> usize {
        if is_verbose {
            print_dumbos(&self.dumbos);
        }
        let expected = self.size.0 * self.size.1;
        let mut total = 0;
        let mut counter = 0;
        while total < expected {
            total = self.iterate();
            counter += 1;
            if is_verbose {
                print_dumbos(&self.dumbos);
                print!("{}", counter);
            }
        }
        counter
    }

    fn iterate(&mut self) -> usize {
        let mut total_flashed = 0;
        let mut just_flashed = VecDeque::new();

        for ix in 0..self.size.0 {
            for jy in 0..self.size.1 {
                let mut d = self.dumbos[ix][jy] + 1;
                if d > 9 {
                    total_flashed += 1;
                    just_flashed.push_back((ix, jy));
                    d = 0;
                }
                self.dumbos[ix][jy] = d;
            }
        }

        while just_flashed.len() > 0 {
            // println!("Just flashed: {:?}", just_flashed);
            for (ix, jy) in get_neighbors(just_flashed.pop_front().unwrap(), self.size, false) {
                if self.dumbos[ix][jy] == 0 {
                    continue;
                }
                let mut d = self.dumbos[ix][jy] + 1;
                if d > 9 {
                    total_flashed += 1;
                    just_flashed.push_back((ix, jy));
                    d = 0;
                }
                self.dumbos[ix][jy] = d;
            }
        }
        total_flashed
    }
}


#[cfg(test)]
mod tests {
    use crate::{make_file_name, read_input};

    use super::*;

    #[test]
    fn day_11() {
        let test_data = read_input(make_file_name(true, 11, 1));
        assert_eq!(solve_11_1(&test_data, false), 1656);
        assert_eq!(solve_11_2(&test_data, false), 195);
    }
}
