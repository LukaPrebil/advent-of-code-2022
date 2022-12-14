use std::time::Instant;

struct Instruction(u32, u32, u32);

struct Parsed {
    stacks: Vec<Vec<char>>,
    instructions: Vec<Instruction>,
}

fn parse_input(input: &str) -> Parsed {
    let start = Instant::now();
    let mut split = input.split("\n\n");
    let stacks = split.next().unwrap();
    let stacks_vec = build_stacks(stacks);

    let instructions_vec: Vec<Instruction> = split
        .next()
        .unwrap()
        .lines()
        .map(|line| {
            let mut formatted_line = line.to_string();
            formatted_line.retain(|c| c.is_numeric() || c == ' ');

            let split_line: Vec<&str> = formatted_line.split_whitespace().collect();

            Instruction(
                split_line[0].parse().unwrap(),
                split_line[1].parse().unwrap(),
                split_line[2].parse().unwrap(),
            )
        })
        .collect();
    println!("parse_input: {}us", start.elapsed().as_micros());
    Parsed {
        stacks: stacks_vec,
        instructions: instructions_vec,
    }
}

fn build_stacks(stacks: &str) -> Vec<Vec<char>> {
    let split_stacks: Vec<&str> = stacks.lines().rev().collect();
    let len = split_stacks.last().unwrap().len();
    let mut stack_vec: Vec<Vec<char>> = vec![Vec::new(); len];

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
    let Parsed {
        stacks,
        instructions,
    } = parse_input(input);
    let mut result = "".to_string();
    let start = Instant::now();
    instructions
        .iter()
        .fold(stacks, |mut stack, Instruction(how_many, from, to)| {
            let stack_len = stack[*from as usize - 1].len();
            let mut moved_items =
                stack[*from as usize - 1].split_off(stack_len - *how_many as usize);
            if should_reverse {
                moved_items.reverse();
            }
            stack[*to as usize - 1].append(&mut moved_items);
            stack
        })
        .iter()
        .for_each(|stack| {
            result.push(*stack.last().unwrap());
        });
    println!(
        "solve with should_reverse={} took {}us",
        should_reverse,
        start.elapsed().as_micros()
    );
    result
}
