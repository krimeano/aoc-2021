use crate::aoc_lib::{make_file_name, read_input};
use crate::day_01::{solve_1_1, solve_1_2};
use crate::day_02::{solve_2_1, solve_2_2};

mod aoc_lib;
mod day_01;
mod day_02;


fn main() {
    let day = 2;
    match day {
        1 => {
            let raw_input = read_input(make_file_name(false, day, 1));
            println!("Day {} task {}: {}", day, 1, solve_1_1(&raw_input, false));
            println!("Day {} task {}: {}", day, 2, solve_1_2(&raw_input, false));
        }
        2 => {
            let raw_input = read_input(make_file_name(false, day, 1));
            println!("Day {} task {}: {}", day, 1, solve_2_1(&raw_input, false));
            println!("Day {} task {}: {}", day, 2, solve_2_2(&raw_input, false));
        }
        _ => { panic!("Day is not solved yet") }
    }
}

