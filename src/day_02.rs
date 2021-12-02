pub mod day_2 {
    struct Ort(i32, i32);

    struct Movement(Ort, i32);

    struct Position(i32, i32);

    pub fn solve_2_1(raw_input: &Vec<String>, verbose: bool) -> i32 {
        let input = preprocess(&raw_input);
        let mut position = Position(0, 0);
        for x in input {
            if verbose {
                println!("direction = ({}, {}), value = {}", x.0.0, x.0.1, x.1)
            }
            position.0 += x.0.0 * x.1;
            position.1 += x.0.1 * x.1;
        }

        position.0 * position.1
    }

    fn preprocess(raw_input: &Vec<String>) -> Vec<Movement> {
        raw_input.iter()
            .map(|x| parse_line(String::from(x)))
            .collect()
    }

    fn parse_line(line: String) -> Movement {
        let parts: Vec<String> = line.split(" ")
            .map(|x| String::from(x))
            .collect();

        Movement(
            match &parts[0] as &str {
                "forward" => Ort(1, 0),
                "down" => Ort(0, 1),
                "up" => Ort(0, -1),
                _ => panic!("unexpected input: {}", line)
            },
            parts[1].parse().unwrap())
    }
}

#[cfg(test)]
mod tests {
    use crate::{make_file_name, read_input};
    use crate::day_02::day_2::solve_2_1;

    #[test]
    fn day_2_1() {
        let test_data = read_input(make_file_name(true, 2, 1));
        assert_eq!(solve_2_1(&test_data, true), 150);
    }
}
