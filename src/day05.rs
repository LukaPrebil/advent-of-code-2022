use std::time::Instant;

fn parse_input(input: &str) -> (Vec<Vec<char>>, Vec<(u32, u32, u32)>) {
    let start = Instant::now();
    let mut split = input.split("\n\n");
    let stacks = split.next().unwrap();
    let instructions = split.next().unwrap();
    let stacks_vec = build_stacks(stacks);
    let instructions_vec: Vec<(u32, u32, u32)> = instructions
        .lines()
        .map(|line| {
            let formatted_line = line
                .replace("from ", "")
                .replace("move ", "")
                .replace(" to", "");

            let split_line: Vec<&str> = formatted_line.split(" ").collect();

            (
                split_line[0].parse().unwrap(),
                split_line[1].parse().unwrap(),
                split_line[2].parse().unwrap(),
            )
        })
        .collect();
    println!("parse_input: {:?}us", start.elapsed().as_micros());
    (stacks_vec, instructions_vec)
}

fn build_stacks(stacks: &str) -> Vec<Vec<char>> {
    let mut stack_vec: Vec<Vec<char>> = vec![
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
    ];
    let split_stacks: Vec<&str> = stacks.lines().rev().collect();
    for line in split_stacks {
        for (index, element) in line.char_indices() {
            if element != ' ' {
                stack_vec[index].push(element);
            }
        }
    }
    stack_vec
}

pub fn solve(input: &str, should_reverse: bool) -> String {
    let (stacks, instructions) = parse_input(input);
    let mut result = "".to_string();
    instructions.iter().fold(stacks, |mut stack, (how_many, from, to)| {
        let stack_len = stack[*from as usize - 1].len();
        let mut moved_items = stack[*from as usize - 1].split_off(stack_len - *how_many as usize);
        if should_reverse { moved_items.reverse(); }
        stack[*to as usize - 1].append(&mut moved_items);
        stack
    }).iter().for_each(|stack| {
        result.push(*stack.last().unwrap());
    });
    result
}