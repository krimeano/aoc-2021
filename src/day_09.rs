use std::collections::{HashSet, VecDeque};

const LARGEST_COUNT: usize = 3;

pub fn solve_9_1(raw_input: &[String]) -> u32 {
    let heatmap = find_heatmap(raw_input);
    let risk_points = find_risk_points(&heatmap);

    risk_points.iter().map(|x| heatmap[x.0][x.1] + 1).sum()
}

pub fn solve_9_2(raw_input: &[String], is_verbose: bool) -> usize {
    let heatmap = find_heatmap(raw_input);
    let risk_points = find_risk_points(&heatmap);

    if is_verbose {
        print_heatmap(&heatmap, &risk_points);
    }

    let mut basin_sizes = Vec::new();
    for point in risk_points {
        let basin = find_basin(&heatmap, &point);
        if is_verbose {
            print_heatmap(&heatmap, &basin);
        }
        basin_sizes.push(basin.len());
        basin_sizes.sort_by(|a, b| b.cmp(a));
        if basin_sizes.len() > LARGEST_COUNT {
            basin_sizes.pop();
        }
    }

    let mut out = 1;
    for x in basin_sizes {
        out *= x;
    }
    out
}

fn find_heatmap(raw_input: &[String]) -> Vec<Vec<u32>> {
    raw_input.iter()
        .filter(|x| x.len() > 0)
        .map(|x| x.chars()
            .map(|y| y as u32 - '0' as u32)
            .collect()
        )
        .collect()
}

fn find_risk_points(heatmap: &[Vec<u32>]) -> HashSet<(usize, usize)> {
    let h = heatmap.len();
    let w = heatmap[0].len();

    let mut risk_points = HashSet::new();
    for ix in 0..h {
        for jy in 0..w {
            let cur = heatmap[ix][jy];
            if (ix == 0 || heatmap[ix - 1][jy] > cur)
                && (ix + 1 == h || heatmap[ix + 1][jy] > cur)
                && (jy == 0 || heatmap[ix][jy - 1] > cur)
                && (jy + 1 == w || heatmap[ix][jy + 1] > cur) {
                risk_points.insert((ix, jy));
            }
        }
    }
    risk_points
}

fn print_heatmap(heatmap: &[Vec<u32>], highlight: &HashSet<(usize, usize)>) {
    println!();
    for ix in 0..heatmap.len() {
        let line = &heatmap[ix];
        for jy in 0..line.len() {
            let val = &line[jy];
            if highlight.contains(&(ix, jy)) {
                print!("\x1b[96m\x1b[1m{}\x1b[0m", val);
            } else {
                print!("{}", val);
            }
        }
        println!();
    }
}

fn find_basin(heatmap: &[Vec<u32>], start: &(usize, usize)) -> HashSet<(usize, usize)> {
    let mut out = HashSet::new();
    let mut queue = VecDeque::from([*start]);

    let h = heatmap.len();
    let w = heatmap[0].len();

    while queue.len() > 0 {
        let cur_point = queue.pop_front().unwrap();
        if out.contains(&cur_point) {
            continue;
        }
        let c = heatmap[cur_point.0][cur_point.1];
        out.insert(cur_point);
        let neighbours = &get_neighbours(cur_point.0, cur_point.1, h, w);
        for neighbour in neighbours {
            if out.contains(neighbour) {
                continue;
            }
            let n = heatmap[neighbour.0][neighbour.1];
            if c < n && n < 9 {
                queue.push_back(*neighbour)
            }
        }
        // for neighbor in [(cur.0 - 1, cur.1), (cur.0 + 1, cur.1), (cur.0, cur.1 - 1), (cur.0, cur.1 - 1)] {
        //     if neighbor.0 == -1 || neighbor.0 == h || neighbor.1 == -1 || neighbor.1 == neighbor
        // }
        // println!("cur_point: {:?}, neighbours: {:?}", cur_point, neighbours);
    }
    out
}


fn get_neighbours(ix: usize, jy: usize, h: usize, w: usize) -> Vec<(usize, usize)> {
    let mut out = Vec::new();
    if ix > 0 { out.push((ix - 1, jy)) }
    if ix + 1 < h { out.push((ix + 1, jy)) }

    if jy > 0 { out.push((ix, jy - 1)) }
    if jy + 1 < w { out.push((ix, jy + 1)) }
    out
}

#[cfg(test)]
mod tests {
    use crate::{make_file_name, read_input};

    use super::*;

    #[test]
    fn day_9() {
        let test_data = read_input(make_file_name(true, 9, 1));
        assert_eq!(solve_9_1(&test_data), 15);
        assert_eq!(solve_9_2(&test_data, true), 1134);
    }
}
