struct Ort(i32, i32);

struct Movement(Ort, i32);


pub fn solve_2_1(raw_input: &Vec<String>, verbose: bool) -> i32 {
    let mut x = 0;
    let mut y = 0;

    for line in raw_input {
        if line.len() == 0 {
            continue;
        }

        let Movement(direction, distance) = parse_line(line);
        x += direction.0 * distance;
        y += direction.1 * distance;

        if verbose {
            println!("direction = ({}, {}), distance = {}, position = {}, {}", direction.0, direction.1, distance, x, y);
        }
    }

    x * y
}

pub fn solve_2_2(raw_input: &Vec<String>, verbose: bool) -> i32 {
    let mut aim_depth = 0;
    let mut x = 0;

    let mut y = 0;
    for line in raw_input {
        if line.len() == 0 {
            continue;
        }
        let Movement(direction, distance) = parse_line(line);
        if verbose {
            println!("direction = ({}, {}), value = {}", direction.0, direction.1, distance)
        }
        if direction.0 != 0 {
            x += direction.0 * distance;
            y += direction.0 * distance * aim_depth;
        } else if direction.1 != 0 {
            aim_depth += direction.1 * distance;
        }

        if verbose {
            println!("direction = ({}, {}), value = {}, aim = {}, position = {}, {}", direction.0, direction.1, distance, aim_depth, x, y);
        }
    }
    x * y
}

fn parse_line(line: &String) -> Movement {
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


#[cfg(test)]
mod tests {
    use crate::{make_file_name, read_input, solve_2_1, solve_2_2};

    #[test]
    fn day_2_1() {
        let test_data = read_input(make_file_name(true, 2, 1));
        assert_eq!(solve_2_1(&test_data, false), 150);
    }

    #[test]
    fn day_2_2() {
        let test_data = read_input(make_file_name(true, 2, 1));
        assert_eq!(solve_2_2(&test_data, false), 900);
    }
}
