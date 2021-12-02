pub mod aoc_lib {
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
        println!("read file: {}", filename);
        let contents = fs::read_to_string(filename)
            .expect("Something went wrong reading the file");

        contents.split("\n")
            .filter(|x| !x.is_empty())
            .map(|x| String::from(x))
            .collect()
    }

    pub fn input_to_numbers(vec: &Vec<String>) -> Vec<i32> {
        vec.iter()
            .map(|x| x.parse().unwrap())
            .collect()
    }
}
