pub fn solve_6_1(raw_input: &[String]) -> u64 {
    simulate(raw_input, 80)
}

pub fn solve_6_2(raw_input: &[String]) -> u64 {
    simulate(raw_input, 256)
}

fn simulate(raw_input: &[String], days: usize) -> u64 {
    let school = parse(&raw_input[0]);
    let mut timers = [0; 9];
    for x in school {
        timers[x as usize] += 1;
    }
    for _ in 0..days {
        timers = [
            timers[1],
            timers[2],
            timers[3],
            timers[4],
            timers[5],
            timers[6],
            timers[7] + timers[0],
            timers[8],
            timers[0]
        ];
    }
    timers.iter().sum()
}

fn parse(line: &String) -> Vec<u64> {
    line.split(',')
        .map(|x| x.trim().parse().unwrap())
        .collect()
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
