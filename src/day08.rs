pub fn solve(input: &str) -> i32 {
    let matrix: Vec<&str> = input.lines().collect();
    let mut result = 0;
    let mut seen = vec![vec![false; matrix[0].len()]; matrix.len()];
    for (line_index, line) in matrix.iter().enumerate() {
        let visible_trees = find_visible_trees_in_line(line, line_index, &mut seen, false);
        result += visible_trees;
    }
    for column_index in 0..matrix[0].len() {
        let column = matrix
            .iter()
            .map(|line| line.chars().nth(column_index).unwrap())
            .collect::<String>();
        let visible_trees =
            find_visible_trees_in_line(&column.as_str(), column_index, &mut seen, true);
        result += visible_trees;
    }
    result
}

fn find_visible_trees_in_line(
    line: &&str,
    line_index: usize,
    seen: &mut [Vec<bool>],
    transpose: bool,
) -> i32 {
    let mut visible_trees = 0;
    let mut tallest_tree = -1;
    let mut tallest_tree_rev = -1;
    for char_index in 0..line.len() {
        let tree_height: i32 = line
            .chars()
            .nth(char_index)
            .unwrap()
            .to_digit(10)
            .unwrap()
            .try_into()
            .unwrap();
        let tree_height_rev: i32 = line
            .chars()
            .nth(line.len() - (char_index + 1))
            .unwrap()
            .to_digit(10)
            .unwrap()
            .try_into()
            .unwrap();

        if tree_height > tallest_tree {
            tallest_tree = tree_height;
            let row = if transpose { char_index } else { line_index };
            let col = if transpose { line_index } else { char_index };
            if !seen[row][col] {
                seen[row][col] = true;
                visible_trees += 1;
            }
        }
        if tree_height_rev > tallest_tree_rev {
            tallest_tree_rev = tree_height_rev;
            let row = if transpose {
                line.len() - (char_index + 1)
            } else {
                line_index
            };
            let col = if transpose {
                line_index
            } else {
                line.len() - (char_index + 1)
            };
            if !seen[row][col] {
                seen[row][col] = true;
                visible_trees += 1;
            }
        }
    }
    visible_trees
}

pub fn solve_2(input: &str) -> u32 {
    let matrix: Vec<&str> = input.lines().collect();
    let mut scores = vec![vec![0; matrix[0].len()]; matrix.len()];
    for (row, line) in matrix.iter().enumerate() {
        for (col, tree) in line.char_indices() {
            let tree_height = tree.to_digit(10).unwrap();
            println!("row: {}, col: {}, tree_height: {}", row, col, tree_height);
            let left = get_left_score(col, &matrix, row, tree_height);
            println!("left: {}", left);
            let right = get_right_score(col, &matrix, row, tree_height);
            println!("right: {}", right);
            let top = get_top_score(row, &matrix, col, tree_height);
            println!("top: {}", top);
            let bottom = get_bottom_score(row, &matrix, col, tree_height);
            println!("bottom: {}", bottom);
            scores[row][col] = left * right * top * bottom;
        }
    }
    scores.iter().for_each(|s| println!("{:?}", s));
    *scores.iter().flatten().max().unwrap()
}

fn get_left_score(col: usize, matrix: &[&str], row: usize, tree_height: u32) -> u32 {
    if col == 0 {
        0
    } else if matrix[row]
        .chars()
        .nth(col - 1)
        .unwrap()
        .to_digit(10)
        .unwrap()
        == tree_height
    {
        1
    } else {
        let mut index = col - 1;
        let mut trees = 1;
        while matrix[row]
            .chars()
            .nth(index)
            .unwrap()
            .to_digit(10)
            .unwrap()
            <= tree_height
        {
            trees += 1;
            if matrix[row]
                .chars()
                .nth(index)
                .unwrap()
                .to_digit(10)
                .unwrap()
                == tree_height
                || index == 0
            {
                break;
            }
            if index > 0 {
                index -= 1;
            }
        }
        trees
    }
}

fn get_right_score(col: usize, matrix: &[&str], row: usize, tree_height: u32) -> u32 {
    if col == matrix[row].len() - 1 {
        0
    } else if matrix[row]
        .chars()
        .nth(col + 1)
        .unwrap()
        .to_digit(10)
        .unwrap()
        == tree_height
    {
        1
    } else {
        let mut index = col + 1;
        let mut trees = 1;
        while index < matrix[0].len()
            && matrix[row]
                .chars()
                .nth(index)
                .unwrap()
                .to_digit(10)
                .unwrap()
                <= tree_height
        {
            trees += 1;
            if index == matrix[row].len()
                || matrix[row]
                    .chars()
                    .nth(index)
                    .unwrap()
                    .to_digit(10)
                    .unwrap()
                    == tree_height
            {
                break;
            }
            index += 1;
        }
        trees
    }
}

fn get_top_score(row: usize, matrix: &[&str], col: usize, tree_height: u32) -> u32 {
    if row == 0 {
        0
    } else if matrix[row - 1]
        .chars()
        .nth(col)
        .unwrap()
        .to_digit(10)
        .unwrap()
        == tree_height
    {
        1
    } else {
        let mut index = row - 1;
        let mut trees = 1;
        while matrix[index]
            .chars()
            .nth(col)
            .unwrap()
            .to_digit(10)
            .unwrap()
            <= tree_height
        {
            trees += 1;
            if matrix[index]
                .chars()
                .nth(col)
                .unwrap()
                .to_digit(10)
                .unwrap()
                == tree_height
                || index == 0
            {
                break;
            }
            if index > 0 {
                index -= 1;
            }
        }
        trees
    }
}

fn get_bottom_score(row: usize, matrix: &[&str], col: usize, tree_height: u32) -> u32 {
    if row == matrix.len() - 1 {
        0
    } else if matrix[row + 1]
        .chars()
        .nth(col)
        .unwrap()
        .to_digit(10)
        .unwrap()
        == tree_height
    {
        1
    } else {
        let mut index = row + 1;
        let mut trees = 0;
        while index < matrix.len()
            && matrix[index]
                .chars()
                .nth(col)
                .unwrap()
                .to_digit(10)
                .unwrap()
                <= tree_height
        {
            trees += 1;
            if index == matrix.len()
                || matrix[index]
                    .chars()
                    .nth(col)
                    .unwrap()
                    .to_digit(10)
                    .unwrap()
                    == tree_height
            {
                break;
            }
            index += 1;
        }
        trees
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one() {
        let input =
            std::fs::read_to_string("./src/input/2022/day08-test.txt").expect("File not found");
        let result = solve(&input);
        assert_eq!(result, 21);
    }

    #[test]
    fn part_two() {
        let input =
            std::fs::read_to_string("./src/input/2022/day08-test.txt").expect("File not found");
        let result = solve_2(&input);
        assert_eq!(result, 8);
    }
}
