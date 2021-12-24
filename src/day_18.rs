use std::cmp::max;
use std::fmt;

pub fn solve_18_1(raw_input: &[String]) -> u32 {
    find_sum(raw_input).magnutude()
}

pub fn solve_18_2(raw_input: &[String]) -> u32 {
    let mut fishes = Vec::new();
    for line in raw_input {
        if line.len() == 0 {
            continue;
        }
        fishes.push(SnailFish::from(line));
    }
    let mut out = 0;
    let size = fishes.len();
    for ix in 0..size - 1 {
        for jy in ix + 1..size {
            out = max(
                out,
                max(
                    fishes[ix].clone().add(fishes[jy].clone()).reduce().magnutude(),
                    fishes[jy].clone().add(fishes[ix].clone()).reduce().magnutude(),
                ),
            )
        }
    }
    out
}

fn find_sum(raw_input: &[String]) -> SnailFish {
    let mut sum = SnailFish::N(0);
    for line in raw_input {
        if line.len() == 0 {
            continue;
        }
        let fish = SnailFish::from(line);
        sum = sum.add(fish).reduce();
    }
    sum
}

#[derive(Clone)]
enum SnailFish {
    N(u32),
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
    pub fn new(a: Self, b: Self) -> Self {
        Self::S(Box::new(a), Box::new(b))
    }

    pub fn from(line: &str) -> Self {
        if &line[0..1] != "[" {
            return Self::N(line.parse().unwrap());
        }

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
        Self::new(
            Self::from(&line[1..ix]),
            Self::from(&line[ix + 1..top]),
        )
    }


    pub fn add(self, other: Self) -> Self {
        match self {
            Self::N(x) => match other {
                Self::N(y) => Self::N(x + y),
                Self::S(c, d) => Self::new(Self::N(x).add(*c), *d)
            }
            Self::S(a, b) => match other {
                Self::N(y) => Self::new(*a, b.add(Self::N(y))),
                Self::S(c, d) => Self::new(Self::S(a, b), Self::S(c, d))
            }
        }
    }

    pub fn explode(self, depth: usize) -> (Self, (Option<Self>, Option<Self>)) {
        match self {
            Self::N(x) => (Self::N(x), (None, None)),
            Self::S(a, b) => match depth < 4 {
                true => {
                    let (m, (p, q)) = a.explode(depth + 1);
                    if let Some(q) = q {
                        (Self::new(m, q.add(*b)), (p, Some(Self::N(0))))
                    } else {
                        let (n, (r, s)) = b.explode(depth + 1);
                        if let Some(r) = r {
                            (Self::new(m.add(r), n), (Some(Self::N(0)), s))
                        } else {
                            (Self::new(m, n), (p, s))
                        }
                    }
                }
                false => (Self::N(0), (Some(*a), Some(*b)))
            }
        }
    }

    pub fn split(self) -> (Self, Option<u32>) {
        match self {
            Self::N(x) => match x < 10 {
                true => (Self::N(x), None),
                false => (Self::new(Self::N(x / 2), Self::N(x / 2 + x % 2)), Some(x)),
            }
            Self::S(a, b) => {
                let (m, was_a) = a.split();
                if let Some(was_a) = was_a {
                    (Self::new(m, *b), Some(was_a))
                } else {
                    let (n, was_b) = b.split();
                    (Self::new(m, n), was_b)
                }
            }
        }
    }

    pub fn reduce(self) -> Self {
        let (fish, (p, q)) = self.explode(0);
        if let Some(_) = p {
            fish.reduce()
        } else if let Some(_) = q {
            fish.reduce()
        } else {
            let (fish, old) = fish.split();
            if let Some(_) = old {
                fish.reduce()
            } else {
                fish
            }
        }
    }

    pub fn magnutude(&self) -> u32 {
        match self {
            Self::N(x) => *x,
            Self::S(a, b) => 3 * a.magnutude() + 2 * b.magnutude()
        }
    }
}

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
        assert_eq!(format!("{}", SnailFish::from("2").add(SnailFish::from("3"))), "5");
        assert_eq!(format!("{}", SnailFish::from("[1,2]").add(SnailFish::from("3"))), "[1,5]");
        assert_eq!(format!("{}", SnailFish::from("2").add(SnailFish::from("[3,4]"))), "[5,4]");

        let (fish, _) = SnailFish::from("[[[[[9,8],1],2],3],4]").explode(0);
        assert_eq!(format!("{}", fish), "[[[[0,9],2],3],4]");

        let (fish, _) = SnailFish::from("[7,[6,[5,[4,[3,2]]]]]").explode(0);
        assert_eq!(format!("{}", fish), "[7,[6,[5,[7,0]]]]");

        let (fish, _) = SnailFish::from("[[6,[5,[4,[3,2]]]],1]").explode(0);
        assert_eq!(format!("{}", fish), "[[6,[5,[7,0]]],3]");

        let (fish, _) = SnailFish::from("[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]").explode(0);
        assert_eq!(format!("{}", fish), "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]");

        let (fish, _) = SnailFish::from("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]").explode(0);
        assert_eq!(format!("{}", fish), "[[3,[2,[8,0]]],[9,[5,[7,0]]]]");

        let (fish, _) = SnailFish::from("[[[[0,7],4],[15,[0,13]]],[1,1]]").split();
        assert_eq!(format!("{}", fish), "[[[[0,7],4],[[7,8],[0,13]]],[1,1]]");

        let fish = SnailFish::from("[[[[4,3],4],4],[7,[[8,4],9]]]").add(SnailFish::from("[1,1]"));
        assert_eq!(format!("{}", fish.reduce()), "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]");

        assert_eq!(format!("{}", find_sum(&["[1,1]".to_string(), "[2,2]".to_string(), "[3,3]".to_string(), "[4,4]".to_string()])), "[[[[1,1],[2,2]],[3,3]],[4,4]]");
        assert_eq!(format!("{}", find_sum(&["[1,1]".to_string(), "[2,2]".to_string(), "[3,3]".to_string(), "[4,4]".to_string(), "[5,5]".to_string()])), "[[[[3,0],[5,3]],[4,4]],[5,5]]");
        assert_eq!(format!("{}", find_sum(&["[1,1]".to_string(), "[2,2]".to_string(), "[3,3]".to_string(), "[4,4]".to_string(), "[5,5]".to_string(), "[6,6]".to_string()])), "[[[[5,0],[7,4]],[5,5]],[6,6]]");

        assert_eq!(format!("{}", find_sum(&read_input(make_file_name(true, 18, 2)))), "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]");
        assert_eq!(format!("{}", find_sum(&test_data)), "[[[[6,6],[7,6]],[[7,7],[7,0]]],[[[7,7],[7,7]],[[7,8],[9,9]]]]");

        assert_eq!(SnailFish::from("[[1,2],[[3,4],5]]").magnutude(), 143);
        assert_eq!(SnailFish::from("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]").magnutude(), 1384);
        assert_eq!(SnailFish::from("[[[[1,1],[2,2]],[3,3]],[4,4]]").magnutude(), 445);
        assert_eq!(SnailFish::from("[[[[3,0],[5,3]],[4,4]],[5,5]]").magnutude(), 791);
        assert_eq!(SnailFish::from("[[[[5,0],[7,4]],[5,5]],[6,6]]").magnutude(), 1137);
        assert_eq!(SnailFish::from("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]").magnutude(), 3488);

        assert_eq!(solve_18_1(&test_data), 4140);
        assert_eq!(solve_18_2(&test_data), 3993);
    }
}
