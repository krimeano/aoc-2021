pub fn solve_20_1(raw_input: &[String]) -> usize {
    let mut solution = Solution::from(raw_input);
    // solution.print_img();
    solution.process();
    solution.process();
    // solution.print_img();

    solution.count()
}

pub fn solve_20_2(raw_input: &[String]) -> usize {
    let mut solution = Solution::from(raw_input);
    for _ in 0..50 {
        solution.process()
    }
    solution.count()
}


struct Solution {
    background: usize,
    enhancement: Vec<usize>,
    img: Vec<Vec<usize>>,
}

impl Solution {
    pub fn from(raw_input: &[String]) -> Self {
        let background = 0;
        let mut enhancement = Vec::new();
        let mut img = Vec::new();
        for line in raw_input {
            if line.len() == 0 {
                continue;
            }
            if enhancement.len() == 0 {
                enhancement = line.chars().map(|x| if x == '#' { 1 } else { 0 }).collect();
                continue;
            }
            img.push(line.chars().map(|x| if x == '#' { 1 } else { 0 }).collect())
        };
        Self {
            background,
            enhancement,
            img,
        }
    }

    // pub fn print_img(&self) {
    //     println!();
    //     for row in &self.img {
    //         for &col in row {
    //             print!("{}", if col == 1 { '#' } else { '.' })
    //         }
    //         println!();
    //     }
    //     println!()
    // }

    pub fn count(&self) -> usize {
        let mut out = 0;
        for row in &self.img {
            for &col in row {
                out += col;
            }
        }
        out
    }

    pub fn process(&mut self) {
        let h = self.img.len();
        let w = self.img[0].len();
        let mut new_img = Vec::new();
        for ix in 0..h + 2 {
            let mut row = Vec::new();
            for jy in 0..w + 2 {
                let mut enh_index: usize = 0;
                for kx in 0..3 {
                    for ly in 0..3 {
                        let ax = ix + kx;
                        let by = jy + ly;
                        let val = if ax < 2 || ax >= h + 2 || by < 2 || by >= w + 2 { self.background } else { self.img[ax - 2][by - 2] };
                        enh_index = enh_index << 1;
                        enh_index += val;
                    }
                }
                row.push(self.enhancement[enh_index])
            }
            new_img.push(row);
        }
        self.img = new_img;
        self.background = self.new_bg();
    }

    fn new_bg(&self) -> usize {
        if self.background == 1 { self.enhancement[511] } else { self.enhancement[0] }
    }
}


#[cfg(test)]
mod tests {
    use crate::{make_file_name, read_input};

    use super::*;

    #[test]
    fn day_20() {
        let test_data = read_input(make_file_name(true, 20, 1));
        assert_eq!(solve_20_1(&test_data), 35);
        assert_eq!(solve_20_2(&test_data), 3351);
    }
}
