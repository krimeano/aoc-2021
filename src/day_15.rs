use std::cmp::min;
use std::collections::HashSet;

use crate::aoc_lib::get_neighbors;

const MAP_FACTOR: usize = 5;

pub fn solve_15_1(raw_input: &[String]) -> u32 { dijkstra(read_input(raw_input)) }

pub fn solve_15_2(raw_input: &[String]) -> u32 { dijkstra(make_bigger_map(read_input(raw_input))) }

fn read_input(raw_input: &[String]) -> Vec<Vec<u32>> {
    raw_input.iter()
        .filter(|x| x.len() > 0)
        .map(|x| read_line(x))
        .collect()
}

fn read_line(line: &str) -> Vec<u32> {
    line.chars()
        .map(|x| x.to_string().parse().unwrap())
        .collect()
}

fn make_bigger_map(input: Vec<Vec<u32>>) -> Vec<Vec<u32>> {
    let h = input.len();
    if h == 0 {
        panic!();
    }
    let mut out = Vec::new();
    let w = input[0].len();

    for ix in 0..h * MAP_FACTOR {
        let mut row = Vec::new();
        for jy in 0..w * MAP_FACTOR {
            row.push((input[ix % h][jy % w] + (ix / h) as u32 + (jy / w) as u32 - 1) % 9 + 1);
        }
        out.push(row);
    }
    out
}

fn dijkstra(input: Vec<Vec<u32>>) -> u32 {
    let h = input.len();
    if h == 0 {
        panic!();
    }
    let w = input[0].len();
    let hw = (h, w);
    let mut result = Vec::new();
    let mut visited = HashSet::new();
    let mut excluded = Vec::new();


    for _ in 0..h {
        let mut result_row = Vec::new();
        let mut excluded_row = Vec::new();
        for _ in 0..w {
            result_row.push(u32::MAX);
            excluded_row.push(false);
        }
        result.push(result_row);
        excluded.push(excluded_row);
    }

    result[0][0] = 0;
    visited.insert((0, 0));

    while visited.len() > 0 {
        let mut value = u32::MAX;
        let mut cur = (h, w);
        for (ix, jy) in &visited {
            if result[*ix][*jy] < value {
                value = result[*ix][*jy];
                cur = (*ix, *jy);
            }
        }
        // println!("selected {:?} = {} of {:?}", cur, value, visited);

        visited.remove(&cur);
        excluded[cur.0][cur.1] = true;

        let xx = get_neighbors(cur, hw, true);
        for x in xx {
            if excluded[x.0][x.1] {
                continue;
            }
            let movement = input[x.0][x.1];
            let old_x_value = result[x.0][x.1];
            let new_x_value = min(old_x_value, value + movement);
            // println!("\t{} + {} = {} at {:?} {} {}", value, movement, new_x_value, x, if old_x_value == new_x_value { "stays" } else { "was" }, old_x_value);

            result[x.0][x.1] = new_x_value;
            visited.insert(x);
        }
    }
    result[h - 1][w - 1]
}


#[cfg(test)]
mod tests {
    use crate::{make_file_name, read_input};

    use super::*;

    #[test]
    fn day_15() {
        let test_data = read_input(make_file_name(true, 15, 1));
        assert_eq!(solve_15_1(&test_data), 40);
        assert_eq!(solve_15_2(&test_data), 315);
    }
}
