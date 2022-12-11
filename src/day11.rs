#[derive(Debug)]
struct Monkey {
    inspected_times: u64,
    items: Vec<u64>,
    operation: (char, u64),
    divisible_by: u64,
    true_recipient: u64,
    false_recipient: u64,
}

fn parse_input(input: &str) -> Vec<Monkey> {
    input
        .split("\n\n")
        .map(|monkey| {
            let mut lines = monkey.lines();
            let items = lines
                .nth(1)
                .unwrap()
                .split([' ', ','])
                .filter(|s| !s.is_empty())
                .skip(2)
                .map(|item| item.parse().unwrap())
                .collect();

            let ops = lines.next().unwrap().split(' ').collect::<Vec<_>>();
            let op_raw: Vec<_> = ops.iter().rev().take(2).collect();
            let operation = (
                op_raw[1].chars().next().unwrap(),
                op_raw[0].parse().unwrap_or(0), // Zero as a sentinel for squaring
            );

            let divisible_by = lines
                .next()
                .unwrap()
                .split(' ')
                .last()
                .unwrap()
                .parse()
                .unwrap();
            let true_recipient = lines
                .next()
                .unwrap()
                .split(' ')
                .last()
                .unwrap()
                .parse()
                .unwrap();
            let false_recipient = lines
                .next()
                .unwrap()
                .split(' ')
                .last()
                .unwrap()
                .parse()
                .unwrap();
            Monkey {
                items,
                operation,
                divisible_by,
                true_recipient,
                false_recipient,
                inspected_times: 0,
            }
        })
        .collect()
}

// fn gcd(a: u64, b: u64) -> u64 {
//     if b == 0 {
//         a
//     } else {
//         gcd(b, a % b)
//     }
// }

// fn lcm(a: u64, b: u64) -> u64 {
//     a * b / gcd(a, b)
// }
// fn lcm_of_slice(nums: &[u64]) -> u64 {
//     nums.iter().fold(1, |acc, &x| lcm(acc, x))
// }

pub fn solve(input: &str, is_part_two: bool) -> u64 {
    let mut monkeys = parse_input(input);

    let all_divisors = monkeys
        .iter()
        .map(|monkey| monkey.divisible_by)
        .collect::<Vec<_>>();
    let lcm_of_divisors: u64 = all_divisors.iter().product();

    let number_of_rounds = if is_part_two { 10000 } else { 20 };

    for _round in 0..number_of_rounds {
        let mut items_for_monkeys: Vec<Vec<u64>> = vec![Vec::new(); monkeys.len()];
        for (monkey_index, monkey) in monkeys.iter_mut().enumerate() {
            monkey
                .items
                .extend(items_for_monkeys[monkey_index].drain(0..));
            if monkey.items.is_empty() {
                continue;
            }
            monkey.inspected_times += monkey.items.len() as u64;
            let mut indices_to_throw: Vec<(usize, u64)> = Vec::new(); // (index, recipient)
            for (index, item) in monkey.items.iter_mut().enumerate() {
                match monkey.operation {
                    ('*', 0) => *item *= *item,
                    ('+', n) => *item += n,
                    ('*', n) => *item *= n,
                    _ => panic!("Unknown operation"),
                };
                if is_part_two {
                    *item %= lcm_of_divisors;
                } else {
                    *item /= 3;
                }
                if *item % monkey.divisible_by == 0 {
                    indices_to_throw.push((index, monkey.true_recipient));
                } else {
                    indices_to_throw.push((index, monkey.false_recipient));
                }
            }
            for (index, recipient) in indices_to_throw.iter().rev() {
                items_for_monkeys[*recipient as usize].push(monkey.items.remove(*index));
            }
        }
        for (monkey, items) in monkeys.iter_mut().zip(items_for_monkeys) {
            monkey.items.extend(items.iter().rev());
        }
    }
    println!("{:#?}", monkeys);
    monkeys.sort_by_key(|monkey| monkey.inspected_times);
    monkeys
        .iter()
        .rev()
        .take(2)
        .map(|monkey| monkey.inspected_times)
        .product()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn log_parsed() {
        let input =
            std::fs::read_to_string("./src/input/2022/day11-test.txt").expect("File not found");
        let result = parse_input(&input);
        println!("{:#?}", result);
        assert_eq!(result.len(), 4);
    }

    #[test]
    fn part_one() {
        let input =
            std::fs::read_to_string("./src/input/2022/day11-test.txt").expect("File not found");
        let result = solve(&input, false);
        assert_eq!(result, 10605);
    }

    #[test]
    fn part_two() {
        let input =
            std::fs::read_to_string("./src/input/2022/day11-test.txt").expect("File not found");
        let result = solve(&input, true);
        assert_eq!(result, 2713310158);
    }
}
