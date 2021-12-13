use std::collections::{HashMap, HashSet, VecDeque};

const START: &str = "start";
const END: &str = "end";

pub fn solve_12_1(raw_input: &[String]) -> usize {
    solve_12(raw_input, false)
}

pub fn solve_12_2(raw_input: &[String]) -> usize {
    solve_12(raw_input, true)
}

pub fn solve_12(raw_input: &[String], can_visit_twice: bool) -> usize {
    let mut all_caves: HashSet<String> = HashSet::new();
    let mut cave_paths: HashMap<String, HashSet<String>> = HashMap::new();
    let visited_small_caves = HashSet::from([START.to_string()]);
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
    let mut paths = find_paths(START, visited_small_caves, all_caves.clone(), &cave_paths, can_visit_twice, 0);
    paths.sort();
    // for x in &paths {
    //     println!("{:?}", x);
    // }
    //
    // println!("{}", paths.len());
    paths.len()
}


fn is_big_cave(cave: &str) -> bool {
    if let Some(x) = cave.chars().nth(0) { x.is_uppercase() } else { false }
}

fn find_paths(cur: &str, visited_small_caves: HashSet<String>, all_caves: HashSet<String>, cave_paths: &HashMap<String, HashSet<String>>, can_visit_twice: bool, depth: usize) -> Vec<VecDeque<String>> {
    let mut paths = Vec::new();
    if cur == END {
        let end = VecDeque::from([cur.to_string()]);
        paths.push(end);
        return paths;
    }
    if let Some(neighbors) = cave_paths.get(cur) {
        // for _ in 0..depth {
        //     print!("\t");
        // }
        // println!("{}, {}, visited: {:?}, neighbors: {:?}, {:?}", &cur, can_visit_twice, &visited_small_caves, &neighbors, &all_caves);
        for x in neighbors {
            if all_caves.contains(x) {
                let mut available_caves = all_caves.clone();
                let mut just_visited_small_caves = visited_small_caves.clone();
                let mut allow_next_visit = can_visit_twice;
                if !is_big_cave(x) {
                    just_visited_small_caves.insert(x.to_string());
                    if can_visit_twice && visited_small_caves.contains(x) {
                        for y in &just_visited_small_caves {
                            available_caves.remove(y);
                        }
                        allow_next_visit = false;
                    } else if !can_visit_twice {
                        available_caves.remove(x);
                    }
                }
                let sub_paths = find_paths(x, just_visited_small_caves, available_caves, &cave_paths, allow_next_visit, depth + 1);
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
        assert_eq!(solve_12_2(&test_data_1), 36);
        assert_eq!(solve_12_2(&test_data_2), 103);
        assert_eq!(solve_12_2(&test_data_3), 3509);
    }
}
