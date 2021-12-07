use crate::aoc_lib::csl_to_numbers;

pub fn solve_6_1(raw_input: &[String]) -> u64 {
    simulate(raw_input, 80)
}

pub fn solve_6_2(raw_input: &[String]) -> u64 {
    simulate(raw_input, 256)
}

fn simulate(raw_input: &[String], days: usize) -> u64 {
    let school = csl_to_numbers(&raw_input[0]);
    let mut timers = [0; 9];
    for x in school {
        timers[x as usize] += 1;
    }
    for _ in 0..days {
        let newcomers = timers[0];
        for ix in 0..8 {
            timers[ix] = timers[ix + 1]
        }
        timers[6] += newcomers;
        timers[8] = newcomers;
    }
    timers.iter().sum()
}


#[cfg(test)]
mod tests {
    use crate::{make_file_name, read_input};

    use super::*;

    #[test]
    fn day_6() {
        let test_data = read_input(make_file_name(true, 6, 1));
        assert_eq!(solve_6_1(&test_data), 5934);
        assert_eq!(solve_6_2(&test_data), 26984457539);
    }
}
