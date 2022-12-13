use std::fs;
fn main() {
    let input = fs::read_to_string("day_12/src/input.txt").unwrap();

    let input: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    println!("{:?}", input);
    star1(input);
    //star2(&input);
}

fn star1(input: Vec<Vec<char>>) {}
