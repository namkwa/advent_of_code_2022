use std::{
    collections::{binary_heap::Iter, HashMap},
    fs,
};
fn main() {
    let input = fs::read_to_string("day_7/src/input.txt").unwrap();

    let input: Vec<String> = input.split("\r\n").map(|line| line.to_owned()).collect();
    star1(input);
}

fn star1(input: Vec<String>) {
    let mut output = 0;
    for line in input {
        if line.contains("$ cd") && line != String::from("$ cd ..") {
        } else {
        }
    }
}
