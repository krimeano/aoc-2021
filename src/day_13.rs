use std::collections::{HashSet, VecDeque};

#[derive(Debug)]
enum ReadState {
    Dots,
    Instructions,
}


#[derive(PartialEq, Eq, Hash, Debug)]
struct Dot {
    x: u16,
    y: u16,
}

#[derive(PartialEq, Eq, Hash, Debug)]
struct Instruction {
    axis: char,
    value: u16,
}

pub fn solve_13_1(raw_input: &[String]) -> usize {
    let (mut dots, mut instructions) = read_input(raw_input);

    if let Some(instruction) = instructions.pop_front() {
        dots = fold(dots, instruction);
    }

    dots.len()
}

pub fn solve_13_2(raw_input: &[String], to_print: bool) -> usize {
    let (mut dots, mut instructions) = read_input(raw_input);

    let mut x_max = 0;
    let mut y_max = 0;
    while let Some(instruction) = instructions.pop_front() {
        match instruction.axis {
            'x' => x_max = instruction.value,
            'y' => y_max = instruction.value,
            _ => panic!("unexpected axis!")
        }
        dots = fold(dots, instruction);
    }
    if to_print {
        for y in 0..y_max {
            for x in 0..x_max {
                if let Some(_) = dots.get(&Dot { x, y }) {
                    print!("#");
                } else {
                    print!(" ");
                }
            }
            println!();
        }
    }
    0
}

fn read_input(raw_input: &[String]) -> (HashSet<Dot>, VecDeque<Instruction>) {
    let mut read_state = ReadState::Dots;
    let mut dots = HashSet::new();
    let mut instructions = VecDeque::new();

    for line in raw_input {
        match read_state {
            ReadState::Dots => {
                if line.len() > 0 {
                    let coords: Vec<u16> = line.split(",")
                        .map(|x| x.parse().unwrap())
                        .collect();
                    let dot = Dot { x: coords[0], y: coords[1] };
                    dots.insert(dot);
                } else {
                    read_state = ReadState::Instructions;
                }
            }
            ReadState::Instructions => {
                if line.len() > 0 {
                    let chunks: Vec<String> = line.split('=').map(|x| String::from(x)).collect();
                    if let Some(axis) = chunks[0].chars().last() {
                        let value = chunks[1].parse().unwrap();
                        let instruction = Instruction { axis, value };
                        instructions.push_back(instruction);
                    } else {
                        panic!()
                    }
                }
            }
        }
    }
    (dots, instructions)
}

fn fold(dots: HashSet<Dot>, instruction: Instruction) -> HashSet<Dot> {
    let mut out = HashSet::new();
    // println!("{:?}", &instruction);
    // println!("{:?}", &dots);

    for dot in dots {
        match instruction.axis {
            'x' => {
                let x = if dot.x < instruction.value { dot.x } else { 2 * instruction.value - dot.x };
                let y = dot.y;
                out.insert(Dot { x, y });
            }
            'y' => {
                let x = dot.x;
                let y = if dot.y < instruction.value { dot.y } else { 2 * instruction.value - dot.y };
                out.insert(Dot { x, y });
            }
            _ => panic!("unexpected axis!")
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use crate::{make_file_name, read_input};

    use super::*;

    #[test]
    fn day_13() {
        let test_data = read_input(make_file_name(true, 13, 1));
        assert_eq!(solve_13_1(&test_data), 17);
        assert_eq!(solve_13_2(&test_data, false), 0);
    }
}
