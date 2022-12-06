use std::fs;
fn main() {
    let input = fs::read_to_string("day_5/src/input.txt").unwrap();

    let input: Vec<&str> = input.lines().collect();
    star2(input);
}

fn star2(input: Vec<&str>) {
    let mut stacks: Vec<Vec<char>> = Vec::new();
    stacks.push(vec!['B', 'W', 'N']);
    stacks.push(vec!['L', 'Z', 'S', 'P', 'T', 'D', 'M', 'B']);
    stacks.push(vec!['Q', 'H', 'Z', 'W', 'R']);
    stacks.push(vec!['W', 'D', 'V', 'J', 'Z', 'R']);
    stacks.push(vec!['S', 'H', 'M', 'B']);
    stacks.push(vec!['L', 'G', 'N', 'J', 'H', 'V', 'P', 'B']);
    stacks.push(vec!['J', 'Q', 'Z', 'F', 'H', 'D', 'L', 'S']);
    stacks.push(vec!['W', 'S', 'F', 'J', 'G', 'Q', 'B']);
    stacks.push(vec!['Z', 'W', 'M', 'S', 'C', 'D', 'J']);
    for line in input {
        let (_, temp) = line.split_once("move ").unwrap();
        let (number_of_moves, temp) = temp.split_once(" from ").unwrap();
        let (origin, destination) = temp.split_once(" to ").unwrap();
        let (number_of_moves, origin, destination): (usize, usize, usize) = (
            number_of_moves.parse().unwrap(),
            origin.parse().unwrap(),
            destination.parse().unwrap(),
        );
        let len_origin = stacks.get(origin - 1).expect("REASON").len();
        for _ in 0..number_of_moves {
            let char_to_move = *stacks[origin - 1]
                .get(len_origin - number_of_moves)
                .unwrap();
            stacks[origin - 1].remove(len_origin - number_of_moves);
            stacks[destination - 1].push(char_to_move);
        }
    }
    let mut output = String::new();
    for i in 0..9 {
        output.push(*stacks[i].last().unwrap());
    }
    println!("{}", output);
}
