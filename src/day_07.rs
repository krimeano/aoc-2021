use std::cmp::min;

use crate::aoc_lib::csl_to_numbers;

pub fn solve_7_1(raw_input: &[String]) -> u32 {
    let mut swarm = csl_to_numbers(&raw_input[0]);
    swarm.sort();
    let m = (swarm.len() - 1) / 2;
    let pivot = swarm[m];
    let mut total = 0;
    for x in swarm {
        let diff = if x > pivot { x - pivot } else { pivot - x };
        total += diff;
    }
    total
}

pub fn solve_7_2(raw_input: &[String]) -> u32 {
    let swarm = csl_to_numbers(&raw_input[0]);

    let mut sum_xx = 0;
    for x in &swarm {
        sum_xx += x;
    }
    let n = swarm.len() as u32;
    // let rem = sum_xx % n;
    let mean = sum_xx / n; // + if 2 * rem > n { 1 } else { 0 };

    min(calculate_fuel(mean, &swarm), calculate_fuel(mean + 1, &swarm))
}

fn calculate_fuel(pivot: u32, swarm: &[u32]) -> u32 {
    let mut total = 0;
    for x in swarm {
        let diff = if *x > pivot { (x - pivot) * (x - pivot + 1) } else { (pivot - x) * (pivot - x + 1) } / 2;
        total += diff;
    }
    total
}


#[cfg(test)]
mod tests {
    use crate::{make_file_name, read_input};

    use super::*;

    #[test]
    fn day_7() {
        let test_data = read_input(make_file_name(true, 7, 1));
        assert_eq!(solve_7_1(&test_data), 37);
        assert_eq!(solve_7_2(&test_data), 168);
    }
}
