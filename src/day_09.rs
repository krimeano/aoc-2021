pub fn solve_9_1(raw_input: &[String]) -> u32 {
    let heatmap: Vec<Vec<u32>> = raw_input.iter()
        .filter(|x| x.len() > 0)
        .map(|x| x.chars()
            .map(|y| y as u32 - '0' as u32)
            .collect()
        )
        .collect();

    let h = heatmap.len();
    let w = heatmap[0].len();

    let mut risk_points: Vec<(usize, usize, u32)> = Vec::new();
    for ix in 0..h {
        for jy in 0..w {
            let cur = heatmap[ix][jy];
            if (ix == 0 || heatmap[ix - 1][jy] > cur)
                && (ix + 1 == h || heatmap[ix + 1][jy] > cur)
                && (jy == 0 || heatmap[ix][jy - 1] > cur)
                && (jy + 1 == w || heatmap[ix][jy + 1] > cur) {
                risk_points.push((ix, jy, cur))
            }
        }
    }


    risk_points.iter().map(|x| x.2 + 1).sum()
}

pub fn solve_9_2(raw_input: &[String]) -> u32 { 0 }


#[cfg(test)]
mod tests {
    use crate::{make_file_name, read_input};

    use super::*;

    #[test]
    fn day_9() {
        let test_data = read_input(make_file_name(true, 9, 1));
        assert_eq!(solve_9_1(&test_data), 15);
        assert_eq!(solve_9_2(&test_data), 0);
    }
}
