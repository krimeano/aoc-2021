use crate::aoc_lib::input_to_numbers;

pub fn solve_1_1(raw_input: &Vec<String>, verbose: bool) -> i32 {
    solve_1(input_to_numbers(raw_input), verbose)
}

pub fn solve_1_2(raw_input: &Vec<String>, verbose: bool) -> i32 {
    let input_data = input_to_numbers(raw_input);
    let mut windows = Vec::new();
    let size = 3;
    // let mut stored_last_x = None;
    for i in 0..input_data.len() - size + 1 {
        let mut x = 0;
        for j in 0..size {
            x += &input_data[i + j];
        }
        windows.push(x);
    }

    return solve_1(windows, verbose);
}


fn solve_1(input_data: Vec<i32>, verbose: bool) -> i32 {
    let mut stored_last_x = None;
    let mut increased = 0;
    input_data
        .iter()
        .for_each(|x| {
            let mut has_increased = 0;
            match stored_last_x {
                Some(last_x) => {
                    has_increased = if x > last_x { 1 } else { 0 };
                    if verbose {
                        if x > &last_x {
                            println!("{}, {}, INCREASED", &last_x, x);
                        } else if x < &last_x {
                            println!("{}, {}, decreased", &last_x, x);
                        } else {
                            println!("{}, {}, no change", &last_x, x);
                        }
                    }
                }
                None => if verbose { println!("N/A, {}, first value", x) }
            }

            increased += has_increased;
            stored_last_x = Some(x);
        });
    increased
}


#[cfg(test)]
mod tests {
    use crate::{make_file_name, read_input, solve_1_1, solve_1_2};

    #[test]
    fn day_1_1() {
        let test_data = read_input(make_file_name(true, 1, 1));
        assert_eq!(solve_1_1(&test_data, false), 7);
    }

    #[test]
    fn day_1_2() {
        let test_data = read_input(make_file_name(true, 1, 1));
        assert_eq!(solve_1_2(&test_data, false), 5);
    }
}
