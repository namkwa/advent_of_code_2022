use std::{collections::HashSet, fs};
fn main() {
    let input = fs::read_to_string("day_9/src/input.txt").unwrap();

    let input: Vec<&str> = input.lines().collect();
    star1(input);
}

fn star1(input: Vec<&str>) {
    let mut visited_positions: HashSet<(i32, i32)> = HashSet::new();
    let mut rope: Vec<(i32, i32)> = vec![(0, 0); 10];
    visited_positions.insert((0, 0));
    for line in input {
        let (direction, amount) = line.split_once(' ').unwrap();
        for _ in 0..amount.parse::<usize>().unwrap() {
            match direction.chars().nth(0).unwrap() {
                'R' => rope[0].0 += 1,
                'L' => rope[0].0 -= 1,
                'U' => rope[0].1 += 1,
                'D' => rope[0].1 -= 1,
                _ => unreachable!(),
            }
            for idx in 1..10 {
                let mut xdiff = rope[idx - 1].0 - rope[idx].0;
                let mut ydiff = rope[idx - 1].1 - rope[idx].1;
                while xdiff.abs() >= 2 || ydiff.abs() >= 2 {
                    if xdiff != 0 {
                        rope[idx].0 += xdiff / (xdiff.abs());
                    }
                    if ydiff != 0 {
                        rope[idx].1 += ydiff / (ydiff.abs());
                    }
                    if idx == 9 {
                        visited_positions.insert(rope[idx]);
                    }
                    xdiff = rope[idx].0 - rope[idx - 1].0;
                    ydiff = rope[idx].1 - rope[idx - 1].1;
                }
            }
        }
    }
    println!("{}", visited_positions.len());
}
