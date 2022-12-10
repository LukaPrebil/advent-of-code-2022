pub fn solve(input: &str) -> i32 {
    let mut screen = [' '; 240];
    let end_state = input.lines().fold((0, 1, 0), |data, instruction| {
        let (mut cycle, mut register, mut sum) = data;
        if instruction == "noop" {
            write_to_screen(&mut screen, cycle, register);
            increment_cycle(&mut cycle, &mut sum, register);
        } else {
            write_to_screen(&mut screen, cycle, register);
            increment_cycle(&mut cycle, &mut sum, register);
            write_to_screen(&mut screen, cycle, register);
            increment_cycle(&mut cycle, &mut sum, register);
            register += instruction[5..].parse::<i32>().unwrap();
        }
        (cycle, register, sum)
    });
    print_screen(&screen);
    end_state.2
}

fn increment_cycle(cycle: &mut i32, sum: &mut i32, register: i32) {
    *cycle += 1;
    if [20, 60, 100, 140, 180, 220].contains(cycle) {
        *sum += register * *cycle;
    }
}

fn write_to_screen(screen: &mut [char; 240], cycle: i32, register: i32) {
    if register.abs_diff(cycle % 40) <= 1 {
        screen[cycle as usize] = '\u{2588}';
    }
}

fn print_screen(screen: &[char; 240]) {
    for (i, pixel) in screen.iter().enumerate() {
        if [40, 80, 120, 160, 200, 240].contains(&i) {
            println!();
        }
        print!("{}", pixel);
    }
    println!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one() {
        let input =
            std::fs::read_to_string("./src/input/2022/day10-test.txt").expect("File not found");
        let result = solve(&input);
        assert_eq!(result, 13140);
    }
}
