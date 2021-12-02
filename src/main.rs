use crate::aoc_lib::aoc_lib::{make_file_name, read_input};
use crate::day_01::day_1::{solve_1_1, solve_1_2};
use crate::day_02::day_2::solve_2_1;

mod aoc_lib;
mod day_01;
mod day_02;


fn main() {
    let raw_input = read_input(make_file_name(false, 1, 1));
    println!("Day1 task1: {}", solve_1_1(&raw_input, false));
    println!("Day1 task2: {}", solve_1_2(&raw_input, false));

    println!("Day2 task2: {}", solve_2_1(&read_input(make_file_name(false, 2, 1)), false));
}

