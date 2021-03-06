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
use crate::day_11::{solve_11_1, solve_11_2};
use crate::day_12::{solve_12_1, solve_12_2};
use crate::day_13::{solve_13_1, solve_13_2};
use crate::day_14::{solve_14_1, solve_14_2};
use crate::day_15::{solve_15_1, solve_15_2};
use crate::day_16::{solve_16_1, solve_16_2};
use crate::day_17::{solve_17_1, solve_17_2};
use crate::day_18::{solve_18_1, solve_18_2};
use crate::day_19::{solve_19_1, solve_19_2};
use crate::day_20::{solve_20_1, solve_20_2};
use crate::day_21::{solve_21_1, solve_21_2};
use crate::day_22::{solve_22_1, solve_22_2};

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
mod day_11;
mod day_12;
mod day_13;
mod day_14;
mod day_15;
mod day_16;
mod day_17;
mod day_18;
mod day_19;
mod day_20;
mod day_21;
mod day_22;


fn main() {
    let day = 21;
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
        11 => {
            let raw_input = read_input(make_file_name(false, day, 1));
            println!("Day {} task {}: {}", day, 1, solve_11_1(&raw_input, false));
            println!("Day {} task {}: {}", day, 2, solve_11_2(&raw_input, false));
        }
        12 => {
            let raw_input = read_input(make_file_name(false, day, 1));
            println!("Day {} task {}: {}", day, 1, solve_12_1(&raw_input));
            println!("Day {} task {}: {}", day, 2, solve_12_2(&raw_input));
        }
        13 => {
            let raw_input = read_input(make_file_name(false, day, 1));
            println!("Day {} task {}: {}", day, 1, solve_13_1(&raw_input));
            println!("Day {} task {}: {}", day, 2, solve_13_2(&raw_input, true));
        }
        14 => {
            let raw_input = read_input(make_file_name(false, day, 1));
            println!("Day {} task {}: {}", day, 1, solve_14_1(&raw_input));
            println!("Day {} task {}: {}", day, 2, solve_14_2(&raw_input));
        }
        15 => {
            let raw_input = read_input(make_file_name(false, day, 1));
            println!("Day {} task {}: {}", day, 1, solve_15_1(&raw_input));
            println!("Day {} task {}: {}", day, 2, solve_15_2(&raw_input));
        }
        16 => {
            let raw_input = read_input(make_file_name(false, day, 1));
            println!("Day {} task {}: {}", day, 1, solve_16_1(&raw_input));
            println!("Day {} task {}: {}", day, 2, solve_16_2(&raw_input));
        }
        17 => {
            let raw_input = read_input(make_file_name(false, day, 1));
            println!("Day {} task {}: {}", day, 1, solve_17_1(&raw_input));
            println!("Day {} task {}: {}", day, 2, solve_17_2(&raw_input));
        }
        18 => {
            let raw_input = read_input(make_file_name(false, day, 1));
            println!("Day {} task {}: {}", day, 1, solve_18_1(&raw_input));
            println!("Day {} task {}: {}", day, 2, solve_18_2(&raw_input));
        }
        19 => {
            let raw_input = read_input(make_file_name(false, day, 1));
            println!("Day {} task {}: {}", day, 1, solve_19_1(&raw_input));
            println!("Day {} task {}: {}", day, 2, solve_19_2(&raw_input));
        }
        20 => {
            let raw_input = read_input(make_file_name(false, day, 1));
            println!("Day {} task {}: {}", day, 1, solve_20_1(&raw_input));
            println!("Day {} task {}: {}", day, 2, solve_20_2(&raw_input));
        }
        21 => {
            println!("Day {} task {}: {}", day, 1, solve_21_1([10, 7]));
            println!("Day {} task {}: {}", day, 2, solve_21_2([10, 7]));
        }
        22 => {
            let raw_input = read_input(make_file_name(false, day, 1));
            println!("Day {} task {}: {}", day, 1, solve_22_1(&raw_input));
            println!("Day {} task {}: {}", day, 2, solve_22_2(&raw_input));
        }
        _ => { panic!("Day is not solved yet") }
    }
}

