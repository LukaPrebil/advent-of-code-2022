pub fn parse_input_1(input: &str) -> Vec<Vec<u32>> {
    input
        .split("\n\n")
        .map(|s| s.split('\n').map(|n| n.parse().unwrap()).collect())
        .collect()
}

pub fn solve_1(input: &[Vec<u32>]) -> u32 {
    input
        .iter()
        .map(|cals| cals.iter().sum::<u32>())
        .max()
        .unwrap()
}

pub fn parse_input_2(input: &str) -> Vec<u32> {
    input
        .split("\n\n")
        .map(|s| s.split('\n').map(|n| n.parse().unwrap()).collect())
        .collect::<Vec<Vec<u32>>>()
        .iter()
        .map(|cals| cals.iter().sum())
        .collect()
}

pub fn solve_2(input: &mut [u32]) -> u32 {
    input.sort();
    input.iter().rev().take(3).sum()
}
