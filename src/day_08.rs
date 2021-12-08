pub fn solve_8_1(raw_input: &[String]) -> u32 {
    let mut total = 0;
    for line in raw_input {
        if line.len() == 0 { continue; }

        let data: Vec<Vec<String>> = line.split('|')
            .map(|x| x.trim().split(' ')
                .filter(|y| y.len() > 0)
                .map(|y| String::from(y))
                .collect()
            )
            .collect();
        for x in &data[1] {
            if x.len() < 5 || x.len() > 6 {
                total += 1;
            }
        }
    }
    total
}

pub fn solve_8_2(raw_input: &[String]) -> u32 { 0 }


#[cfg(test)]
mod tests {
    use crate::{make_file_name, read_input};

    use super::*;

    #[test]
    fn day_8() {
        let test_data = read_input(make_file_name(true, 8, 1));
        assert_eq!(solve_8_1(&test_data), 26);
        assert_eq!(solve_8_2(&test_data), 0);
    }
}
