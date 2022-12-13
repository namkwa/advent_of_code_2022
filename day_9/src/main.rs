use std::{collections::HashSet, fs};
fn main() {
    let input = fs::read_to_string("day_9/src/input.txt").unwrap();

    let input: Vec<&str> = input.lines().collect();
    star1(input);
}

fn star1(input: Vec<&str>) {
    let mut visited_positions: HashSet<(i32, i32)> = HashSet::new();
    let mut head_position: (i32, i32) = (0, 0);
    let mut tail_position: (i32, i32) = (0, 0);
    visited_positions.insert(tail_position);
    for line in input {
        let (direction, amount) = line.split_once(' ').unwrap();
        for _ in 0..amount.parse::<usize>().unwrap() {
            match direction.chars().nth(0).unwrap() {
                'R' => head_position.0 += 1,
                'L' => head_position.0 -= 1,
                'U' => head_position.1 += 1,
                'D' => head_position.1 -= 1,
                _ => panic!(),
            }
            let mut xdiff = head_position.0 - tail_position.0;
            let mut ydiff = head_position.1 - tail_position.1;
            while xdiff.abs() >= 2 || ydiff.abs() >= 2 {
                if xdiff != 0 {
                    tail_position.0 += xdiff / (xdiff.abs());
                }
                if ydiff != 0 {
                    tail_position.1 += ydiff / (ydiff.abs());
                }
                visited_positions.insert(tail_position);
                xdiff = head_position.0 - tail_position.0;
                ydiff = head_position.1 - tail_position.1;
            }
        }
    }
    println!("{}", visited_positions.len());
}
