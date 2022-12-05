use std::{collections::HashSet, fs};

fn main() {
    let input = fs::read_to_string("day_3/src/input.txt").unwrap();

    let rucksacks: Vec<&str> = input.split("\r\n").collect();
    star1(&rucksacks);
    star2(rucksacks);
}

fn star1(input: &Vec<&str>) {
    let mut total: i32 = 0;
    for rucksack in input {
        let (first, second) = rucksack.split_at(rucksack.len() / 2);
        let first_set = str_to_hashset(first);
        let second_set = str_to_hashset(second);
        for character in first_set.iter() {
            if second_set.contains(character) {
                if character.is_uppercase() {
                    total += *character as i32 - 38;
                    break;
                } else if character.is_lowercase() {
                    total += *character as i32 - 96;
                    break;
                }
            }
        }
    }
    println!("{}", total);
}

fn star2(input: Vec<&str>) {
    let mut total: i32 = 0;
    let mut counter: usize = 0;
    let input_size: usize = input.len();
    while counter < input_size {
        if counter % 3 != 0 {
            continue;
        }
        let first_set = str_to_hashset(input.get(counter).unwrap());
        let second_set = str_to_hashset(input.get(counter + 1).unwrap());
        let third_set = str_to_hashset(input.get(counter + 2).unwrap());
        for character in first_set.iter() {
            if second_set.contains(character) && third_set.contains(character) {
                if character.is_uppercase() {
                    total += *character as i32 - 38;
                    break;
                } else if character.is_lowercase() {
                    total += *character as i32 - 96;
                    break;
                }
            }
        }
        counter += 1;
    }
    println!("{}", total);
}

fn str_to_hashset(input: &str) -> HashSet<char> {
    let mut output: HashSet<char> = HashSet::new();
    for character in input.chars() {
        output.insert(character);
    }
    return output;
}
