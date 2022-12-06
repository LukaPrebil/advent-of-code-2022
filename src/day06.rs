use std::{collections::{HashSet, VecDeque}, time::Instant};

pub fn solve(input: &str, unique_len: usize) -> u32 {
    let start = Instant::now();
    let mut window: VecDeque<char> = VecDeque::new();
    for (index, char) in input.char_indices() {
        window.push_back(char);
        if window.len() > unique_len {
            window.pop_front();
        }
        if window.len() == unique_len && all_unique(&window) {
            println!("solve with unique_len {}: {}us", unique_len, start.elapsed().as_micros());
            return index as u32 + 1;
        }
    }
    0
}

fn all_unique(deque: &VecDeque<char>) -> bool {
    let mut set = HashSet::new();
    for item in deque {
        if set.contains(item) {
            return false;
        }
        set.insert(item);
    }
    true
}
