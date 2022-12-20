struct Parsed {
    tensor: Vec<Vec<Vec<bool>>>,
    coordinates: Vec<(usize, usize, usize)>,
}

fn parse_input(input: &str) -> Parsed {
    let max_num = 2 + input
        .lines()
        .map(|line| {
            line.splitn(3, ',')
                .map(|n| n.parse::<usize>().unwrap())
                .max()
                .unwrap()
        })
        .max()
        .unwrap();
    let mut tensor = vec![vec![vec![false; max_num]; max_num]; max_num];
    let coordinates = input
        .lines()
        .map(|line| {
            let coordinates: Vec<usize> = line.splitn(3, ',').map(|n| n.parse().unwrap()).collect();
            tensor[coordinates[0]][coordinates[1]][coordinates[2]] = true; // set position as occupied
            (coordinates[0], coordinates[1], coordinates[2])
        })
        .collect();

    Parsed {
        tensor,
        coordinates,
    }
}

pub fn solve(input: &str) -> usize {
  let parsed = parse_input(input);
  let mut exposed_sides = 0;
  for (x, y, z) in parsed.coordinates {
    exposed_sides += 6 - count_neighbours(&parsed.tensor, x, y, z);
  }
  exposed_sides
}

fn count_neighbours(tensor: &[Vec<Vec<bool>>], x: usize, y: usize, z: usize) -> usize {
    let mut count = 0;
    for x_offset in [-1, 1] {
        if x as i32 + x_offset >= 0 && x as i32 + x_offset < tensor.len() as i32 && tensor[(x as i32 + x_offset) as usize][y][z] {
            count += 1;
        }
        
    }
    for y_offset in [-1, 1] {
        if y as i32 + y_offset >= 0 && y as i32 + y_offset < tensor.len() as i32 && tensor[x][(y as i32 + y_offset) as usize][z] {
            count += 1;
        }
    }
    for z_offset in [-1, 1] {
        if z as i32 + z_offset >= 0 && z as i32 + z_offset < tensor.len() as i32 && tensor[x][y][(z as i32 + z_offset) as usize] {
            count += 1;
        }
    }  
    count
}

fn solve_2(input: &str) -> usize {
    let parsed = parse_input(input);
    let border_cubes: Vec<_> = parsed.coordinates.iter().filter(|(x,y,z)| {
        let neighbours = count_neighbours(&parsed.tensor, *x, *y, *z);
        if neighbours < 6 && neighbours > 0  {
            // true if can reach edge, else false
            

        } else {
            false
        }
    }).collect();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        let input =
            std::fs::read_to_string("./src/input/2022/day18-test.txt").expect("File not found");
        let result = parse_input(&input);
        assert_eq!(result.tensor.len(), 8);
    }

    #[test]
    fn test_solve() {
        let input =
            std::fs::read_to_string("./src/input/2022/day18-test.txt").expect("File not found");
        let result = solve(&input);
        assert_eq!(result, 64);
    }

    #[test]
    fn test_solve_easy() {
        let input = "1,1,1\n2,1,1";
        let result = solve(input);
        assert_eq!(result, 10);
    }
}
