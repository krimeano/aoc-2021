use std::cmp::{max, min};
use std::collections::HashSet;

const LIMIT: i32 = 50;

pub fn solve_22_1(raw_input: &[String]) -> usize {
    let mut lights: HashSet<(i32, i32, i32)> = HashSet::new();

    for line in raw_input {
        if line.len() == 0 {
            continue;
        }
        // println!("{:?}", line);
        let cmd = parse_line(line);
        // println!("{:?}", cmd);
        if let Some((is_on, (xx, yy, zz))) = cmd {
            for ix in xx.0..xx.1 + 1 {
                for jy in yy.0..yy.1 + 1 {
                    for kz in zz.0..zz.1 + 1 {
                        if is_on {
                            lights.insert((ix, jy, kz));
                        } else {
                            lights.remove(&(ix, jy, kz));
                        }
                    }
                }
            }
        }
    }

    lights.len()
}

pub fn solve_22_2(_raw_input: &[String]) -> u32 { 0 }


fn parse_line(line: &str) -> Option<(bool, ((i32, i32), (i32, i32), (i32, i32)))> {
    let is_on = line.chars().nth(1).unwrap() == 'n';
    let coords: Vec<Option<(i32, i32)>> = line.split(' ').nth(1).unwrap().split(',')
        .map(|x| parse_coords(x))
        .collect();

    if let Some(xx) = coords[0] {
        if let Some(yy) = coords[1] {
            if let Some(zz) = coords[2] {
                return Some((is_on, (xx, yy, zz)));
            }
        }
    }

    None
}

fn parse_coords(raw: &str) -> Option<(i32, i32)> {
    let numbers: Vec<i32> = raw.split('=').nth(1).unwrap().split("..")
        .map(|x| x.parse().unwrap())
        .collect();
    let x0 = min(numbers[0], numbers[1]);
    let x1 = max(numbers[0], numbers[1]);
    if x0 > LIMIT || x1 < -LIMIT {
        None
    } else {
        Some((x0, x1))
    }
}


#[cfg(test)]
mod tests {
    use crate::{make_file_name, read_input};

    use super::*;

    #[test]
    fn day_22() {
        let test_data = read_input(make_file_name(true, 22, 1));
        let test_data_2 = read_input(make_file_name(true, 22, 2));
        assert_eq!(solve_22_1(&test_data), 39);
        assert_eq!(solve_22_1(&test_data_2), 590784);
        assert_eq!(solve_22_2(&test_data), 0);
    }
}
