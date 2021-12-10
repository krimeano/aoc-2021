use crate::aoc_lib::{make_file_name, read_input};
use crate::day_01::{solve_1_1, solve_1_2};
use crate::day_02::{solve_2_1, solve_2_2};
use crate::day_03::{solve_3_1, solve_3_2};
use crate::day_04::{solve_4_1, solve_4_2};
use crate::day_05::{solve_5_1, solve_5_2};
use crate::day_06::{solve_6_1, solve_6_2};
use crate::day_07::{solve_7_1, solve_7_2};
use crate::day_08::{solve_8_1, solve_8_2};
use crate::day_09::{solve_9_1, solve_9_2};
use crate::day_10::{solve_10_1, solve_10_2};

mod aoc_lib;
mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod day_05;
mod day_06;
mod day_07;
mod day_08;
mod day_09;
mod day_10;


fn main() {
    let day = 10;
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
        3 => {
            let raw_input = read_input(make_file_name(false, day, 1));
            println!("Day {} task {}: {}", day, 1, solve_3_1(&raw_input));
            println!("Day {} task {}: {}", day, 2, solve_3_2(&raw_input));
        }
        4 => {
            let raw_input = read_input(make_file_name(false, day, 1));
            println!("Day {} task {}: {}", day, 1, solve_4_1(&raw_input));
            println!("Day {} task {}: {}", day, 2, solve_4_2(&raw_input));
        }
        5 => {
            let raw_input = read_input(make_file_name(false, day, 1));
            println!("Day {} task {}: {}", day, 1, solve_5_1(&raw_input));
            println!("Day {} task {}: {}", day, 2, solve_5_2(&raw_input));
        }
        6 => {
            let raw_input = read_input(make_file_name(false, day, 1));
            println!("Day {} task {}: {}", day, 1, solve_6_1(&raw_input));
            println!("Day {} task {}: {}", day, 2, solve_6_2(&raw_input));
        }
        7 => {
            let raw_input = read_input(make_file_name(false, day, 1));
            println!("Day {} task {}: {}", day, 1, solve_7_1(&raw_input));
            println!("Day {} task {}: {}", day, 2, solve_7_2(&raw_input));
        }
        8 => {
            let raw_input = read_input(make_file_name(false, day, 1));
            println!("Day {} task {}: {}", day, 1, solve_8_1(&raw_input));
            println!("Day {} task {}: {}", day, 2, solve_8_2(&raw_input));
        }
        9 => {
            let raw_input = read_input(make_file_name(false, day, 1));
            println!("Day {} task {}: {}", day, 1, solve_9_1(&raw_input));
            println!("Day {} task {}: {}", day, 2, solve_9_2(&raw_input, false));
        }
        10 => {
            let raw_input = read_input(make_file_name(false, day, 1));
            println!("Day {} task {}: {}", day, 1, solve_10_1(&raw_input));
            println!("Day {} task {}: {}", day, 2, solve_10_2(&raw_input));
        }
        _ => { panic!("Day is not solved yet") }
    }
}

