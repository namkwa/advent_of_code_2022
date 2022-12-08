use std::fs;
fn main() {
    let input = fs::read_to_string("day_7/src/input.txt").unwrap();

    let input: Vec<&str> = input.lines().collect();
    star1(input);
}

fn star1(input: Vec<&str>) {}
