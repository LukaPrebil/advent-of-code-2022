use std::{collections::VecDeque, time::Instant};

pub fn solve(input: &str, unique_len: usize) -> u32 {
    let start = Instant::now();
    let mut window: VecDeque<char> = VecDeque::new();
    for (index, char) in input.char_indices() {
        window.push_back(char);
        if window.len() > unique_len {
            window.pop_front();
        }
        if window.len() == unique_len && all_unique_without_set(&window) {
            println!("solve with unique_len {}: {}us", unique_len, start.elapsed().as_micros());
            return index as u32 + 1;
        }
    }
    0
}

// fn all_unique(deque: &VecDeque<char>) -> bool {
//     let mut set = HashSet::new();
//     for item in deque {
//         if set.contains(item) {
//             return false;
//         }
//         set.insert(item);
//     }
//     true
// }

// This is an order of magnitude faster than the above
// Mainly because it is not allocating a new set, and hashing each value to check
fn all_unique_without_set(deque: &VecDeque<char>) -> bool {
    for (index, item) in deque.iter().enumerate() {
        for (index2, item2) in deque.iter().enumerate() {
            if index != index2 && item == item2 {
                return false;
            }
        }
    }
    true
}
