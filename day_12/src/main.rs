use std::fs;
fn main() {
    let input = fs::read_to_string("day_12/src/input.txt").unwrap();

    let input: Vec<Vec<i32>> = input
        .lines()
        .map(|line| line.chars().map(|x| x as i32 - 96).collect::<Vec<i32>>())
        .collect::<Vec<Vec<i32>>>();
    star1(input);
}

fn star1(input: Vec<Vec<i32>>) {}

fn get_next_step() {}
