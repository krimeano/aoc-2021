
//   a 64 a
// b        c
// 32      16
// b        c
//   d  8 d
// e        f
// 4        2
// e        f
//   g  1 g

pub fn solve_8_1(raw_input: &[String]) -> u32 {
    let mut total = 0;
    for line in raw_input {
        if line.len() == 0 { continue; }

        let mut is_after_pipe = false;
        let mut counter: usize = 0;
        for x in line.chars() {
            total += match x {
                '|' => {
                    is_after_pipe = true;
                    0
                }
                ' ' => {
                    let inc = if is_after_pipe && is_1478(counter) { 1 } else { 0 };
                    counter = 0;
                    inc
                }
                _ => {
                    counter += if is_after_pipe { 1 } else { 0 };
                    0
                }
            }
        }
        if is_1478(counter) {
            total += 1;
        }
    }
    total
}

pub fn solve_8_2(_raw_input: &[String]) -> u32 {
    0
}

fn is_1478(n: usize) -> bool { n > 1 && n != 5 && n != 6 }


#[cfg(test)]
mod tests {
    use crate::{make_file_name, read_input};

    use super::*;

    #[test]
    fn day_8() {
        let test_data = read_input(make_file_name(true, 8, 1));
        assert_eq!(solve_8_1(&test_data), 26);
        assert_eq!(solve_8_2(&[String::from("acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf")]), 0);
        // assert_eq!(solve_8_2(&[String::from("acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf")]), 5353);
        // assert_eq!(solve_8_2(&test_data), 61229);
    }
}
