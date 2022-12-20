use std::fs;
fn main() {
    let input = fs::read_to_string("day_20/src/input.txt").unwrap();

    let input: Vec<i32> = input.lines().map(|x| x.parse::<i32>().unwrap()).collect();
    star1(input);
}

fn star1(input: Vec<i32>) {
    let mut input_copy = input.clone();
    let input_len = input.len();
    for line in input {
        let start = input_copy.iter().position(|x| *x == line).unwrap();
        let end = if line > 0 {
            (start + line as usize) % input_len
        } else {
            ((start as i32 + line) % input_len as i32) as usize
        };
    }
}
