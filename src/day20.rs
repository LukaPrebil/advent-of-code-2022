fn parse_input(input: &str) -> Vec<i64> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

pub fn solve(input: &str, is_part_2: bool) -> i64 {
    let coordinates = parse_input(input);
    let decryption_key = if is_part_2 { 811589153 } else { 1 };
    let iterations = if is_part_2 { 10 } else { 1 };
    mix(&coordinates, iterations, decryption_key)
}

fn mix(coordinates: &[i64], iterations: usize, decryption_key: i64) -> i64 {
    let decrypted_coords = coordinates
        .iter()
        .map(|x| x * decryption_key)
        .collect::<Vec<_>>();
    let mut working = (0..decrypted_coords.len()).collect::<Vec<_>>();
    for _ in 0..iterations {
        for (index, &decrypted_coordinate) in decrypted_coords.iter().enumerate() {
            let pos = working.iter().position(|&x| x == index).unwrap();
            working.remove(pos);
            let new_i =
                (pos as i64 + decrypted_coordinate).rem_euclid(working.len() as i64) as usize;
            working.insert(new_i, index);
        }
    }
    let orig_zero_i = decrypted_coords.iter().position(|&i| i == 0).unwrap();
    let zero_i = working.iter().position(|&i| i == orig_zero_i).unwrap();
    [1000, 2000, 3000]
        .iter()
        .map(|i| decrypted_coords[working[(zero_i + i) % working.len()]])
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        let input =
            std::fs::read_to_string("./src/input/2022/day19-test.txt").expect("File not found");
        let result = parse_input(&input);
        assert_eq!(result.len(), 7);
    }

    #[test]
    fn test_solve() {
        let input =
            std::fs::read_to_string("./src/input/2022/day20-test.txt").expect("File not found");
        let result = solve(&input, false);
        assert_eq!(result, 3);
    }

    #[test]
    fn test_solve_2() {
        let input =
            std::fs::read_to_string("./src/input/2022/day20-test.txt").expect("File not found");
        let result = solve(&input, true);
        assert_eq!(result, 1623178306);
    }

}
