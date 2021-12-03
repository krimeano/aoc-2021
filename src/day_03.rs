pub fn solve_3_1(raw_input: &Vec<String>) -> i32 {
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
        println!("{}, {} of {}", ix, &result[ix], &total);

        gamma = gamma << 1;
        epsilon = epsilon << 1;

        gamma += if &result[ix] > &total { 1 } else { 0 };
        epsilon += if &result[ix] < &total { 1 } else { 0 };
    }
    println!("gamma = {}, epsilon = {}", &gamma, &epsilon);

    gamma * epsilon
}




#[cfg(test)]
mod tests {
    use crate::{make_file_name, read_input};

    use super::*;

    #[test]
    fn day_3_1() {
        let test_data = read_input(make_file_name(true, 3, 1));
        assert_eq!(solve_3_1(&test_data), 198);
    }
}
