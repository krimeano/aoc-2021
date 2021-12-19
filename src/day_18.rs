use std::fmt;

#[derive(Clone)]
enum SnailFish {
    N(u64),
    S(Box<Self>, Box<Self>),
}


impl fmt::Display for SnailFish {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::N(n) => write!(f, "{}", n),
            Self::S(a, b) => {
                write!(f, "[{},{}]", *a, *b)
            }
        }
    }
}

impl SnailFish {
    pub fn from(line: &str) -> Self {
        if &line[0..1] != "[" {
            return Self::N(line.parse().unwrap());
        }

        // let parse_item = |x: &str| Box::new(if &x[0..1] == "[" {
        //     Item::S(Self::from(x))
        // } else {
        //     Item::N(x.parse().unwrap())
        // });
        //
        let top = line.len() - 1;
        let mut iter = line.chars();
        let mut ix = 0;
        let mut opened = 0;

        while let &Some(c) = &iter.next() {
            match c {
                '[' => opened += 1,
                ']' => opened -= 1,
                ',' => if opened == 1 { break; },
                _ => {}
            }
            ix += 1;
        }
        Self::S(
            Box::new(SnailFish::from(&line[1..ix])),
            Box::new(SnailFish::from(&line[ix + 1..top])),
        )
    }

    pub fn add(self, other: Self) -> Self {
        Self::S(
            Box::new(self),
            Box::new(other),
        )
    }
}


pub fn solve_18_1(_raw_input: &[String]) -> u32 {
    0
}

pub fn solve_18_2(_raw_input: &[String]) -> u32 { 0 }


#[cfg(test)]
mod tests {
    use crate::{make_file_name, read_input};

    use super::*;

    #[test]
    fn day_18() {
        let test_data = read_input(make_file_name(true, 18, 1));

        let fish = "[[[[1,3],[5,3]],[[1,3],[8,7]]],[[[4,9],[6,9]],[[8,2],[7,3]]]]";
        assert_eq!(format!("{}", SnailFish::from(fish)), fish);
        assert_eq!(format!("{}", SnailFish::from("[1,2]").add(SnailFish::from("[[3,4],5]"))), "[[1,2],[[3,4],5]]");
        assert_eq!(solve_18_1(&test_data), 4140);
        assert_eq!(solve_18_2(&test_data), 0);
    }
}
