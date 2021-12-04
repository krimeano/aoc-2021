use crate::aoc_lib::bin_str_to_number;

pub fn solve_3_1(raw_input: &[String]) -> i32 {
    let total = raw_input.len();

    if total == 0 {
        return 0;
    }

    let size = raw_input[0].len();
    let mut result = vec![0; size];

    for line in raw_input {
        line.chars().enumerate().for_each(|(ix, x)| match x {
            '1' => { result[ix] += 2 }
            '0' => {}
            _ => { panic!("unknown character: \"{}\"", x) }
        });
    }

    let mut gamma = 0;
    let mut epsilon = 0;

    for ix in 0..size {
        gamma = gamma << 1;
        epsilon = epsilon << 1;

        gamma += if &result[ix] > &total { 1 } else { 0 };
        epsilon += if &result[ix] < &total { 1 } else { 0 };
    }

    gamma * epsilon
}

pub fn solve_3_2(raw_input: &[String]) -> i32 {
    let total = raw_input.len();

    if total == 0 {
        return 0;
    }

    let mut input = Vec::new();

    for line in raw_input {
        let x = bin_str_to_number(&line);
        input.push(x);
    }
    input.sort();
    // println!("{:?}", input);

    let oxy = gas_generator_rating(&input, 0, 0, true);
    let co2 = gas_generator_rating(&input, 0, 0, false);
    // println!("{}, {}", oxy, co2);
    oxy * co2
}

fn gas_generator_rating(sorted_data: &Vec<i32>, base: i32, cap: i32, is_oxygen: bool) -> i32 {
    match sorted_data.len() {
        0 => 0,
        1 => sorted_data[0],
        2 => sorted_data[if is_oxygen { 1 } else { 0 }],
        _ => {
            let size = sorted_data.len();
            let half_cap = if cap > 0 { cap } else { define_cap(sorted_data[size - 1]) } >> 1;
            let half = base + half_cap;
            let mut cut = 0;
            for ix in 0..size {
                if sorted_data[ix] >= half {
                    cut = ix;
                    break;
                }
            }
            let doubled_cut = 2 * cut;
            // println!("gas_generator_rating {}, {}, {}, {}, {:?}", &half, &size, &cut, &is_oxygen, &sorted_data);
            if is_oxygen ^ (doubled_cut > size) {
                gas_generator_rating(&sorted_data[cut..].to_vec(), half, half_cap, is_oxygen)
            } else {
                gas_generator_rating(&sorted_data[..cut].to_vec(), base, half_cap, is_oxygen)
            }
        }
    }
}

fn define_cap(number: i32) -> i32 {
    let mut cap = 1;
    while cap < number {
        cap = cap << 1;
    }
    cap
}

#[cfg(test)]
mod tests {
    use crate::{make_file_name, read_input, solve_3_1, solve_3_2};

    use super::*;

    #[test]
    fn cap() {
        assert_eq!(define_cap(1), 1);
        assert_eq!(define_cap(2), 2);
        assert_eq!(define_cap(3), 4);
        assert_eq!(define_cap(4), 4);
        assert_eq!(define_cap(5), 8);
        assert_eq!(define_cap(8), 8);
        assert_eq!(define_cap(10), 16);
        assert_eq!(define_cap(111), 128);
    }

    #[test]
    fn oxygen() {
        assert_eq!(gas_generator_rating(&vec![2, 4, 7, 10, 15, 16, 21, 22, 23, 25, 28, 30], 0, 0, true), 23);
    }

    #[test]
    fn co2() {
        assert_eq!(gas_generator_rating(&vec![2, 4, 7, 10, 15, 16, 21, 22, 23, 25, 28, 30], 0, 0, false), 10);
    }

    #[test]
    fn day_3_1() {
        let test_data = read_input(make_file_name(true, 3, 1));
        assert_eq!(solve_3_1(&test_data), 198);
    }

    #[test]
    fn day_3_2() {
        let test_data = read_input(make_file_name(true, 3, 1));
        assert_eq!(solve_3_2(&test_data), 230);
    }
}
