use std::collections::{HashMap, HashSet, VecDeque};

const START: &str = "start";
const END: &str = "end";

pub fn solve_12_1(raw_input: &[String]) -> usize {
    let mut all_caves: HashSet<String> = HashSet::new();
    let mut cave_paths: HashMap<String, HashSet<String>> = HashMap::new();
    for line in raw_input {
        if line.len() == 0 {
            continue;
        }
        let caves: Vec<String> = line.split('-').map(|x| String::from(x)).collect();
        for cave in &caves {
            all_caves.insert(cave.clone());
        }
        if caves[1] != START {
            cave_paths.entry(caves[0].clone()).or_insert(HashSet::new()).insert(caves[1].clone());
        }
        if caves[0] != START {
            cave_paths.entry(caves[1].clone()).or_insert(HashSet::new()).insert(caves[0].clone());
        }
    }
    // println!("all caves: {:?}", all_caves);
    // println!("big caves: {:?}", big_caves);
    // println!("paths: {:?}", cave_paths);
    all_caves.remove(START);
    let paths = find_paths(START, all_caves.clone(), &cave_paths, 0);
    // println!("paths {:?}", paths);
    println!("{}", paths.len());
    paths.len()
}

pub fn solve_12_2(_raw_input: &[String]) -> u32 { 0 }

fn is_big_cave(cave: &str) -> bool {
    if let Some(x) = cave.chars().nth(0) { x.is_uppercase() } else { false }
}

fn find_paths(cur: &str, all_caves: HashSet<String>, cave_paths: &HashMap<String, HashSet<String>>, depth: usize) -> Vec<VecDeque<String>> {
    let mut paths = Vec::new();
    if cur == END {
        let end = VecDeque::from([cur.to_string()]);
        paths.push(end);
        return paths;
    }
    if let Some(neighbors) = cave_paths.get(cur) {
        for _ in 0..depth {
            print!("\t");
        }
        println!("{}, {:?}, {:?}", &cur, neighbors, all_caves);
        for x in neighbors {
            if all_caves.contains(x) {
                let mut available_caves = all_caves.clone();
                if !is_big_cave(x) {
                    available_caves.remove(x);
                }
                let sub_paths = find_paths(x, available_caves, &cave_paths, depth + 1);
                for mut y in sub_paths {
                    y.push_front(cur.to_string());
                    paths.push(y)
                }
            }
        }
    } else {
        panic!("no paths from {}", cur)
    }
    paths
}

#[cfg(test)]
mod tests {
    use crate::{make_file_name, read_input};

    use super::*;

    #[test]
    fn day_12() {
        let test_data_1 = read_input(make_file_name(true, 12, 1));
        let test_data_2 = read_input(make_file_name(true, 12, 2));
        let test_data_3 = read_input(make_file_name(true, 12, 3));
        assert_eq!(solve_12_1(&test_data_1), 10);
        assert_eq!(solve_12_1(&test_data_2), 19);
        assert_eq!(solve_12_1(&test_data_3), 226);
        assert_eq!(solve_12_2(&test_data_1), 0);
    }
}
