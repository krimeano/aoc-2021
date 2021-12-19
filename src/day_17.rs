use std::collections::{HashMap, HashSet};

use regex::Regex;

pub fn solve_17_1(raw_input: &[String]) -> i32 {
    let input = read_input(&raw_input[0]);
    input.1.0 * (input.1.0 + 1) / 2
}

pub fn solve_17_2(raw_input: &[String]) -> usize {
    let input = read_input(&raw_input[0]);
    let n_x = find_possible_n_x(input.0);
    let mut speed_x_per_n: HashMap<i32, Vec<i32>> = HashMap::new();
    for n in n_x.0 {
        let mut speed_x = Vec::new();
        for v_x in find_v_x(input.0.0, n).ceil() as i32..find_v_x(input.0.1, n).floor() as i32 + 1 {
            if v_x >= n {
                speed_x.push(v_x);
            }
        }
        speed_x_per_n.insert(n, speed_x);
    }
    let y_speed_limit = -input.1.0 * 2 + 1;
    let mut speed_combinations = HashSet::new();
    for n in 1..y_speed_limit {
        let mut speed_y: Vec<i32> = Vec::new();
        let min_v_y = find_v_x(input.1.0, n);
        let max_v_y = find_v_x(input.1.1, n);
        for v_y in min_v_y.ceil() as i32..max_v_y.floor() as i32 + 1 {
            speed_y.push(v_y);
        }
        let speed_x = if let Some(speed_x) = speed_x_per_n.get(&n) { speed_x.clone() } else { n_x.1.clone() };
        for &x in &speed_x {
            for &y in &speed_y {
                speed_combinations.insert((x, y));
            }
        }
    }
    speed_combinations.len()
}

fn read_input(line: &str) -> ((i32, i32), (i32, i32)) {
    let re = Regex::new(r"^.*: x=([^.]+)\.\.([^.]+), y=([^.]+)\.\.([^.]+)$").unwrap();
    let cap = re.captures(line).unwrap();

    ((cap[1].parse().unwrap(), cap[2].parse().unwrap()), (cap[3].parse().unwrap(), cap[4].parse().unwrap()))
}

fn find_possible_n_x(xx: (i32, i32)) -> (Vec<i32>, Vec<i32>) {
    let mut out_exact = Vec::new();
    let mut out_gt = Vec::new();
    let mut total = 1;
    let mut ix: i32 = 1;
    while total <= xx.1 {
        if total >= xx.0 {
            out_gt.push(ix)
        } else {
            out_exact.push(ix);
        }
        ix += 1;
        total += ix;
    }
    (out_exact, out_gt)
}

fn find_v_x(x: i32, n: i32) -> f32 {
    (2 * x + n * (n - 1)) as f32 / (2 * n) as f32
}

#[cfg(test)]
mod tests {
    use crate::{make_file_name, read_input};

    use super::*;

    #[test]
    fn day_17() {
        let test_data = read_input(make_file_name(true, 17, 1));
        assert_eq!(solve_17_1(&test_data), 45);
        assert_eq!(solve_17_2(&test_data), 112);
    }
}
