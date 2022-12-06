use std::{collections::VecDeque, fs};
fn main() {
    let input = fs::read_to_string("day_6/src/input.txt").unwrap();
    star1(input.chars().collect(), 13);
}

fn star1(input: Vec<char>, marker_size: usize) {
    let mut marker: VecDeque<char> = VecDeque::new();
    for i in 0..marker_size {
        marker.push_back(*input.get(i).unwrap());
    }
    let mut counter = marker_size.clone();
    while counter < input.len() {
        let new_char: char = *input.get(counter).unwrap();
        while marker.contains(&new_char) {
            marker.pop_front();
            counter += 1;
        }
        if marker.len() == marker_size {
            println!("{}", counter + 1);
            return;
        }
        while marker.len() != marker_size {
            if marker.contains(&input.get(counter - (marker_size - marker.len())).unwrap()) {
                marker.pop_front();
                counter += 1
            } else {
                marker.push_back(*input.get(counter - (marker_size - marker.len())).unwrap());
            }
        }
    }
}
