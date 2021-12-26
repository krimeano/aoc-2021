use std::collections::{HashMap, HashSet};
use std::fmt;

pub fn solve_19_1(raw_input: &[String]) -> u32 {
    let scanners = read_data(raw_input);
    let scanner_0 = &scanners[0];
    let vectors_0 = scanner_0.find_vectors();
    let scanner = &scanners[1];
    let vectors = scanner.find_vectors();

    let mut sizes = Vec::new();

    for (&s, vv) in &vectors_0 {
        if vv.len() != 1 {
            continue;
        }
        if let Some(ww) = vectors.get(&s) {
            if ww.len() != 1 {
                continue;
            }
            sizes.push(s);
        }
    }

    sizes.sort_by(|a, b| b.cmp(a));

    println!("{:?}", sizes);
    for s in &sizes {
        println!("|{}| {} vs {}", s, vectors_0.get(s).unwrap()[0], vectors.get(s).unwrap()[0]);
    }
    0
}

pub fn solve_19_2(_raw_input: &[String]) -> u32 { 0 }

fn read_data(raw_input: &[String]) -> Vec<Scanner> {
    let mut out = Vec::new();
    let mut cur = Scanner {
        pos: Some(Dot(0, 0, 0)),
        rotation: Some(Rotation((1, 0, 0), (0, 1, 0), (0, 0, 1))),
        beacons: Vec::new(),
    };
    for line in raw_input {
        if line.len() == 0 {
            out.push(cur);
            cur = Scanner { pos: None, rotation: None, beacons: Vec::new() };
            continue;
        }
        if line.chars().nth(1).unwrap() == '-' {
            continue;
        }
        let pos: Vec<i32> = line.split(',').map(|x| x.parse().unwrap()).collect();
        cur.beacons.push(Dot(pos[0], pos[1], pos[2]));
    }
    out
}

#[derive(Debug)]
struct Scanner {
    pos: Option<Dot>,
    rotation: Option<Rotation>,
    beacons: Vec<Dot>,
}

#[derive(Debug)]
struct Rotation((i32, i32, i32), (i32, i32, i32), (i32, i32, i32));

#[derive(Debug, Clone)]
struct Vector {
    beg: Dot,
    end: Dot,
}

#[derive(Debug, Clone)]
struct Dot(i32, i32, i32);


impl Scanner {
    pub fn find_vectors(&self) -> HashMap<i32, Vec<Vector>> {
        let size = self.beacons.len();
        let mut out = HashMap::new();
        for ix in 0..size - 1 {
            for jy in ix + 1..size {
                let v = Vector::new(&self.beacons[ix], &self.beacons[jy]);
                let s = v.sqr();
                out.entry(s).or_insert(Vec::new()).push(v.clone());
            }
        }
        out
    }
}


impl fmt::Display for Vector {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}\t|{}|\t{}->{}", self.val(), self.sqr(), self.beg, self.end)
    }
}

impl Vector {
    pub fn new(beg: &Dot, end: &Dot) -> Self {
        Self {
            beg: beg.clone(),
            end: end.clone(),
        }
    }

    pub fn neg(&self) -> Self {
        Self::new(&self.end, &self.beg)
    }

    pub fn val(&self) -> Dot {
        self.end.rem(&self.beg)
    }

    pub fn sqr(&self) -> i32 {
        let val = self.val();
        val.0 * val.0 + val.1 * val.1 + val.2 * val.2
    }
}


impl fmt::Display for Dot {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {}, {})", self.0, self.1, self.2)
    }
}


impl Dot {
    pub fn neg(&self) -> Self {
        Self(-self.0, -self.1, -self.2)
    }

    pub fn add(&self, other: &Self) -> Self {
        Self(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }

    pub fn rem(&self, other: &Self) -> Self {
        self.add(&other.neg())
    }
}


#[cfg(test)]
mod tests {
    use crate::{make_file_name, read_input};

    use super::*;

    #[test]
    fn day_19() {
        let test_data = read_input(make_file_name(true, 19, 1));
        let test_data_2 = read_input(make_file_name(true, 19, 2));
        // assert_eq!(solve_19_1(&test_data), 12);
        // assert_eq!(solve_19_1(&test_data_2), 79);
        assert_eq!(solve_19_2(&test_data), 0);
    }
}
