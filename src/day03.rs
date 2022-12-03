fn parse_input(input: &str) -> Vec<&str> {
    input.lines().collect()
}

fn get_priority(char: Option<char>) -> u32 {
    match char {
        Some(c) => {
            if c.is_ascii_uppercase() {
                c as u32 - 38
            } else {
                c as u32 - 96
            }
        }
        None => 0,
    }
}

fn find_common_letter(strs: &[&str]) -> Option<char> {
    for char in strs[0].chars() {
        if strs[1..].iter().all(|str| str.contains(char)) {
            return Some(char);
        }
    }
    None
}

pub fn solve_1(input: &str) -> u32 {
    parse_input(input)
        .iter()
        .map(|line| {
            let (first_half, second_half) = line.split_at(line.len() / 2);
            get_priority(find_common_letter(&[first_half, second_half]))
        })
        .sum()
}

pub fn solve_2(input: &str) -> u32 {
    parse_input(input)
        .chunks(3)
        .into_iter()
        .map(|chunk| get_priority(find_common_letter(chunk)))
        .sum()
}
