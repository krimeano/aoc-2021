use crate::my_mod::solve;

mod my_mod {
    use std::fs;

    pub fn make_file_name(is_probe: bool, day: u8, variant: u8) -> String {
        format!(
            "src/{p}input_{d}_{v}.txt",
            p = if is_probe { "probe_" } else { "" },
            d = day,
            v = variant
        )
    }

    pub fn read_input(filename: String) -> Vec<String> {
        println!("read file: {}", filename);
        let contents = fs::read_to_string(filename)
            .expect("Something went wrong reading the file");

        contents.split("\n")
            .filter(|x| !x.is_empty())
            .map(|x| String::from(x))
            .collect()
    }

    pub fn input_to_numbers(vec: Vec<String>) -> Vec<i32> {
        vec.iter()
            .map(|x| x.parse().unwrap())
            .collect()
    }

    pub fn solve(day: u8, variant: u8, expected: i32, solution: fn(Vec<i32>, verbose: bool) -> i32) {
        println!("Probe solution");

        let probe_output = solution(
            input_to_numbers(
                read_input(
                    make_file_name(true, day, variant)
                )
            ),
            true,
        );

        println!("Verifying probed");

        if probe_output == expected {
            println!("Solving");
            let output = solution(
                input_to_numbers(
                    read_input(
                        make_file_name(false, day, variant)
                    )
                ),
                false,
            );
            println!("{}", output);
        } else {
            panic!("\nreceived {}, expected {}", probe_output, expected);
        }
    }
}


fn main() {
    solve(1, 1, 7, solve_1_1);
    solve(1, 1, 5, solve_1_2);
}


fn solve_1_1(input_data: Vec<i32>, verbose: bool) -> i32 {
    // println!("Input data: {}", input_data)
    let mut last_ix = 0;
    let mut increased = 0;
    input_data
        .iter()
        .for_each(|x| {
            if x > &last_ix {
                increased += if last_ix > 0 { 1 } else { 0 };
                if verbose {
                    println!("{}, {}, increased {} times", &last_ix, &x, increased);
                }
            } else {
                if verbose {
                    println!("{}, {}, decreased", &last_ix, &x);
                }
            }
            last_ix = *x;
        });
    increased
}


fn solve_1_2(_input_data: Vec<i32>, _verbose: bool) -> i32 {
    0
}
