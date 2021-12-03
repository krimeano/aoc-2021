use std::fs;

use regex::Regex;

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
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    contents.split("\n")
        .filter(|x| !x.is_empty())
        .map(|x| String::from(x.trim()))
        .collect()
}

pub fn input_to_numbers(vec: &Vec<String>) -> Vec<i32> {
    vec.iter()
        .map(|x| x.parse().unwrap())
        .collect()
}


pub fn bin_str_to_number(bin: &str) -> i32 {
    let re = Regex::new(r"^[01]{1,32}$").unwrap();

    assert!(re.is_match(bin));

    let mut out = 0;
    for x in bin.chars() {
        out = out << 1;
        match x {
            '1' => { out += 1 }
            '0' => {}
            _ => { panic!("unknown character: \"{}\"", x) }
        };
    }
    out
}


#[cfg(test)]
mod tests {
    use crate::aoc_lib::bin_str_to_number;

    #[test]
    fn tes_bin_str_to_number() {
        assert_eq!(bin_str_to_number("0"), 0);
        assert_eq!(bin_str_to_number("1"), 1);
        assert_eq!(bin_str_to_number("100"), 4);
        assert_eq!(bin_str_to_number("0000100"), 4);
        assert_eq!(bin_str_to_number("1111"), 15);
        assert_eq!(bin_str_to_number("1010101"), 85);
    }
}
