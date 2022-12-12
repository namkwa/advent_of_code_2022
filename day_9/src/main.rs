use std::{collections::HashSet, fs};
fn main() {
    let input = fs::read_to_string("day_9/src/input.txt").unwrap();

    let input: Vec<&str> = input.lines().collect();
    star1(input);
}

fn star1(input: Vec<&str>) {
    let mut visited_positions: HashSet<(i32, i32)> = HashSet::new();
    let mut current_x = 0;
    let mut current_y = 0;
    let mut current_direction: char = input.get(0).unwrap().chars().nth(0).unwrap();
    let mut current_orientation: Orientation = get_orientation(current_direction);
    for line in input {
        println!("{} {:?}", line, (current_x, current_y));
        if current_orientation != get_orientation(line.chars().nth(0).unwrap()) {
            current_orientation = get_orientation(line.chars().nth(0).unwrap());
            if current_orientation == Orientation::Horizontal {
                match current_direction {
                    'U' => {
                        current_x -= 1;
                        current_direction = 'U';
                    }
                    'D' => {
                        current_x += 1;
                        current_direction = 'D';
                    }
                    _ => panic!(),
                }
                continue;
            } else {
                match current_direction {
                    'R' => {
                        current_x += 1;
                        current_direction = 'R';
                    }
                    'L' => {
                        current_x -= 1;
                        current_direction = 'L';
                    }
                    _ => panic!(),
                }
                continue;
            }
        }
        match line.chars().nth(0).unwrap() {
            'R' => current_x += line.chars().nth(2).unwrap().to_digit(10).unwrap() as i32,
            'L' => current_x -= line.chars().nth(2).unwrap().to_digit(10).unwrap() as i32,
            'U' => current_y += line.chars().nth(2).unwrap().to_digit(10).unwrap() as i32,
            'D' => current_y -= line.chars().nth(2).unwrap().to_digit(10).unwrap() as i32,
            _ => panic!(),
        }
        visited_positions.insert((current_x, current_y));
    }
    println!("{}", visited_positions.len());
}

fn get_orientation(orientation: char) -> Orientation {
    match orientation {
        'R' | 'L' => return Orientation::Horizontal,
        'U' | 'D' => return Orientation::Vertical,
        _ => return Orientation::Vertical,
    }
}

#[derive(PartialEq)]
enum Orientation {
    Horizontal,
    Vertical,
}
