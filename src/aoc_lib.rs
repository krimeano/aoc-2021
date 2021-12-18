use std::fs;

pub fn make_file_name(is_probe: bool, day: u8, variant: u8) -> String {
    format!(
        "input/{p}day_{d}_{v}.txt",
        p = if is_probe { "probe_" } else { "" },
        d = day,
        v = variant
    )
}

pub fn read_input(filename: String) -> Vec<String> {
    // println!("read file: {}", filename);
    let contents = fs::read_to_string(&filename)
        .expect(&format!("Something went wrong reading the file {}", &filename));

    contents.split("\n")
        // .filter(|x| !x.is_empty())
        .map(|x| String::from(x.trim()))
        .collect()
}

pub fn input_to_numbers(vec: &Vec<String>) -> Vec<i32> {
    vec.iter()
        .filter(|x| !x.is_empty())
        .map(|x| x.parse().unwrap())
        .collect()
}


pub fn bin_str_to_number(bin: &str) -> u32 {
    if bin.len() == 0 {
        panic!("empty string!");
    }
    bin_str_to_number_128(bin, 32) as u32
}

pub fn bin_str_to_number_128(bin: &str, max_size: usize) -> u128 {
    if bin.len() == 0 {
        panic!("empty string!");
    }
    let mut out = 0;
    let mut counter = 0;

    for x in bin.chars() {
        counter += 1;

        if counter > max_size {
            panic!("too large string!");
        }

        out = out << 1;

        match x {
            '1' => out = out | 1,
            '0' => {}
            _ => panic!("expected 0 or 1, received {}", x)
        }
    }
    out
}

pub fn csl_to_numbers(line: &str) -> Vec<u32> {
    line.split(',')
        .map(|x| x.trim().parse().unwrap())
        .collect()
}

pub fn get_neighbors(
    (ix, jy): (usize, usize),
    (h, w): (usize, usize),
    orthogonal_only: bool,
) -> Vec<(usize, usize)> {
    let mut out = Vec::new();

    if ix > 0 {
        out.push((ix - 1, jy));
    }

    if ix + 1 < h {
        out.push((ix + 1, jy));
    }

    if jy > 0 {
        out.push((ix, jy - 1));
        if !orthogonal_only {
            if ix > 0 {
                out.push((ix - 1, jy - 1));
            }
            if ix + 1 < h {
                out.push((ix + 1, jy - 1));
            }
        }
    }

    if jy + 1 < w {
        out.push((ix, jy + 1));
        if !orthogonal_only {
            if ix > 0 {
                out.push((ix - 1, jy + 1));
            }
            if ix + 1 < h {
                out.push((ix + 1, jy + 1));
            }
        }
    }

    out
}

pub fn find_not_less_than(sorted_haystack: &[u32], needle: u32) -> usize {
    let size = sorted_haystack.len();
    let mut bottom = 0;
    let mut top = size;

    while bottom < top {
        let middle = (bottom + top) / 2;
        if sorted_haystack[middle] < needle {
            bottom = middle + 1
        } else {
            top = middle
        }
    }
    bottom
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bin_str_to_number() {
        assert_eq!(bin_str_to_number("0"), 0);
        assert_eq!(bin_str_to_number("1"), 1);
        assert_eq!(bin_str_to_number("100"), 4);
        assert_eq!(bin_str_to_number("0000100"), 4);
        assert_eq!(bin_str_to_number("1111"), 15);
        assert_eq!(bin_str_to_number("1010101"), 85);
    }

    #[test]
    fn test_find() {
        let empty_vec: Vec<u32> = Vec::new();
        assert_eq!(find_not_less_than(&empty_vec, 123), 0);
        assert_eq!(find_not_less_than(&vec![1], 0), 0);
        assert_eq!(find_not_less_than(&vec![1], 1), 0);
        assert_eq!(find_not_less_than(&vec![1], 2), 1);

        assert_eq!(find_not_less_than(&vec![2, 4], 1), 0);
        assert_eq!(find_not_less_than(&vec![2, 4], 2), 0);
        assert_eq!(find_not_less_than(&vec![2, 4], 3), 1);
        assert_eq!(find_not_less_than(&vec![2, 4], 4), 1);
        assert_eq!(find_not_less_than(&vec![2, 4], 5), 2);

        assert_eq!(find_not_less_than(&vec![1, 5, 10], 0), 0);
        assert_eq!(find_not_less_than(&vec![1, 5, 10], 1), 0);
        assert_eq!(find_not_less_than(&vec![1, 5, 10], 2), 1);
        assert_eq!(find_not_less_than(&vec![1, 5, 10], 5), 1);
        assert_eq!(find_not_less_than(&vec![1, 5, 10], 7), 2);
        assert_eq!(find_not_less_than(&vec![1, 5, 10], 10), 2);
        assert_eq!(find_not_less_than(&vec![1, 5, 10], 100), 3);

        assert_eq!(find_not_less_than(&vec![1, 3, 5, 7, 9, 11], 4), 2);
        assert_eq!(find_not_less_than(&vec![1, 3, 5, 7, 9, 11], 5), 2);
        assert_eq!(find_not_less_than(&vec![1, 3, 5, 7, 9, 11], 6), 3);
        assert_eq!(find_not_less_than(&vec![1, 3, 5, 7, 9, 11], 7), 3);
        assert_eq!(find_not_less_than(&vec![1, 3, 5, 7, 9, 11], 8), 4);


        assert_eq!(find_not_less_than(&vec![1, 2, 5, 5, 5, 8], 2), 1);
        assert_eq!(find_not_less_than(&vec![1, 2, 5, 5, 5, 8], 4), 2);
        assert_eq!(find_not_less_than(&vec![1, 2, 5, 5, 5, 8], 5), 2);
        assert_eq!(find_not_less_than(&vec![1, 2, 5, 5, 5, 8], 6), 5);
    }
}
