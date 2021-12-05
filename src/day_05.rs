use std::collections::HashMap;

#[derive(Debug)]
struct Point(u32, u32);

#[derive(Debug)]
struct Segment(Point, Point);

pub fn solve_5_1(raw_input: &[String]) -> u32 { solve_5(raw_input, true) }

pub fn solve_5_2(raw_input: &[String]) -> u32 { solve_5(raw_input, false) }


fn solve_5(raw_input: &[String], omit_diagonals: bool) -> u32 {
    let mut vents: HashMap<(u32, u32), u32> = HashMap::new();
    for line in raw_input {
        if line.len() == 0 {
            continue;
        }
        let segment = parse_line(line);
        if omit_diagonals && segment.0.0 != segment.1.0 && segment.0.1 != segment.1.1 {
            // println!("omit segment {:?}", segment);
            continue;
        }
        let dx = if segment.0.0 == segment.1.0 { 0 } else { 1 };
        let dy = if segment.0.1 == segment.1.1 { 0 } else { 1 };
        let is_positive_y = segment.0.1 < segment.1.1;

        assert!(dx + dy > 0);

        let mut x = segment.0.0;
        let mut y = segment.0.1;

        while (dx > 0 && x <= segment.1.0) || (is_positive_y && y <= segment.1.1) {
            let key = (x, y);
            let z = match vents.get(&key) {
                Some(z) => *z,
                None => 0,
            } + 1;
            vents.insert(key, z);
            x += dx;
            y = if is_positive_y { y + dy } else if y > 0 { y - dy } else { y };
        }
    }
    let mut out = 0;
    for (_point, overlaps) in vents {
        out += if overlaps >= 2 { 1 } else { 0 };
    }
    out
}

fn parse_line(line: &String) -> Segment {
    let mut pp: Vec<Vec<u32>> =
        line.split("->")
            .map(|x|
                x.split(',')
                    .map(|y| String::from(y.trim()).parse().unwrap())
                    .collect()
            )
            .collect();
    assert_eq!(pp.len(), 2);
    pp.sort();
    Segment(
        Point(pp[0][0], pp[0][1]),
        Point(pp[1][0], pp[1][1]),
    )
}

#[cfg(test)]
mod tests {
    use crate::{make_file_name, read_input};

    use super::*;

    #[test]
    fn day_5() {
        let test_data = read_input(make_file_name(true, 5, 1));
        assert_eq!(solve_5_1(&test_data), 5);
        assert_eq!(solve_5_2(&test_data), 12);
    }
}
