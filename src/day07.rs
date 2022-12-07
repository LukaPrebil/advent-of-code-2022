use std::collections::HashMap;

const TOTAL_SPACE: u32 = 70000000;
const NEEDED_SPACE: u32 = 30000000;

fn parse_input(input: &str) -> HashMap<String, u32> {
    let mut dirs: HashMap<String, u32> = HashMap::from([("/".to_string(), 0)]);
    let mut current_path = "/".to_string();
    for line in input.lines() {
        if line.starts_with("dir") {
            let dirname = line.split_whitespace().nth(1).unwrap();
            let mut full_path = current_path.clone();
            if !full_path.ends_with('/') {
                full_path.push('/');
            }
            full_path.push_str(dirname);
            dirs.entry(full_path).or_insert(0);
        } else if line.starts_with("$ cd ..") {
            current_path.truncate(current_path.rfind('/').unwrap());
            if current_path.is_empty() {
                current_path.push('/');
            }
        } else if line.starts_with("$ cd") {
            let dirname = line.split_whitespace().nth(2).unwrap();
            if !current_path.ends_with('/') {
                current_path.push('/');
            }
            current_path.push_str(dirname);
        } else if line.starts_with("$ ls") {
            continue;
        } else {
            let (size, _filename) = line.split_once(' ').unwrap();
            dirs.entry(current_path.clone())
                .and_modify(|e| *e += size.parse::<u32>().unwrap());
        }
    }
    // println!("{:#?}", dirs);
    dirs
}

fn get_total_size(dirs: &HashMap<String, u32>) -> u32 {
    dirs.values().sum()
}

fn find_all_with<Predicate: Fn(&&String, &&u32) -> bool>(
    dirs: &HashMap<String, u32>,
    predicate: Predicate,
) -> HashMap<String, u32> {
    let candidates: HashMap<_, _> = dirs
        .iter()
        .filter(|(k, v)| predicate(k, v))
        .collect();
    let res = candidates
        .iter()
        .map(|(k, v)| {
            let mut multiplier = 0;
            let mut parent_dir = k.as_str().to_string();
            while candidates.contains_key(&parent_dir) {
                multiplier += 1;
                parent_dir.truncate(parent_dir.rfind('/').unwrap());
            }
            (k.as_str().to_string(), *v * multiplier)
        })
        .collect();
    println!("Candidates: {:#?}", res);
    res
}

pub fn solve(input: &str) -> u32 {
    let dirs = parse_input(input);

    find_all_with(&dirs, |k, _| keys_begin_with_sum(&dirs, k) <= 100000)
        .values()
        .sum()
}

pub fn solve_2(input: &str) -> u32 {
    let dirs = parse_input(input);
    let free_space = TOTAL_SPACE - get_total_size(&dirs);
    println!(
        "Free space: {}, needed: {}",
        free_space,
        NEEDED_SPACE - free_space
    );

    let _candidates = find_all_with(&dirs, |k, _| {
        keys_begin_with_sum(&dirs, k) >= NEEDED_SPACE - free_space
    });

    // TODO: Find the smallest dir that can be deleted
    0
}

fn keys_begin_with_sum(map: &HashMap<String, u32>, prefix: &str) -> u32 {
    map.iter()
        .filter(|(k, _)| k.starts_with(prefix))
        .map(|(_, v)| *v)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one() {
        let input =
            std::fs::read_to_string("./src/input/2022/day07-test.txt").expect("File not found");
        let result = solve(&input);
        assert_eq!(result, 95437);
    }

    #[test]
    fn total_size() {
        let input =
            std::fs::read_to_string("./src/input/2022/day07-test.txt").expect("File not found");
        let result = get_total_size(&parse_input(&input));
        assert_eq!(result, 48381165);
    }

    #[test]
    fn part_two() {
        let input =
            std::fs::read_to_string("./src/input/2022/day07-test.txt").expect("File not found");
        let result = solve_2(&input);
        assert_eq!(result, 24933642);
    }
}
