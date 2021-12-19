use std::fmt;

enum Item {
    N(u64),
    S(SnailFish),
}

struct SnailFish(Box<Item>, Box<Item>);


impl fmt::Display for SnailFish {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let a = match &*self.0 {
            Item::N(n) => format!("{}", n),
            Item::S(s) => format!("{}", s),
        };
        let z = match &*self.1 {
            Item::N(n) => format!("{}", n),
            Item::S(s) => format!("{}", s),
        };
        write!(f, "[{},{}]", a, z)
    }
}

impl SnailFish {
    pub fn from(line: &str) -> Self {
        let parse_item = |x: &str| Box::new(if &x[0..1] == "[" {
            Item::S(Self::from(x))
        } else {
            Item::N(x.parse().unwrap())
        });

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
            // println!("ix: {}, char: '{}', opened: {}", ix, c, opened);
            ix += 1;
        }

        Self(parse_item(&line[1..ix]), parse_item(&line[ix + 1..top]))
    }

    pub fn add(a: Self, b: Self) -> Self {
        Self(Box::new(Item::S(a)), Box::new(Item::S(b)))
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
        assert_eq!(format!("{}", SnailFish::add(SnailFish::from("[1,2]"), SnailFish::from("[[3,4],5]"))), "[[1,2],[[3,4],5]]");
        assert_eq!(solve_18_1(&test_data), 4140);
        assert_eq!(solve_18_2(&test_data), 0);
    }
}
