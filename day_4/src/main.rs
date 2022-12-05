use std::fs;
fn main() {
    let input = fs::read_to_string("day_4/src/input.txt").unwrap();

    let input: Vec<&str> = input.split("\r\n").collect();
    star2(input);
    //star2(input);
}

fn star1(input: Vec<&str>) {
    let mut total: u32 = 0;
    for line in input {
        let (first, second) = line.split_once(',').unwrap();
        let (first_min, first_max) = first.split_once('-').unwrap();
        let (second_min, second_max) = second.split_once('-').unwrap();
        let (first_min, first_max): (u32, u32) =
            (first_min.parse().unwrap(), first_max.parse().unwrap());
        let (second_min, second_max): (u32, u32) =
            (second_min.parse().unwrap(), second_max.parse().unwrap());
        if (first_min <= second_min && first_max >= second_max)
            || (first_min >= second_min && first_max <= second_max)
        {
            total += 1;
        }
    }
    println!("{}", total);
}

fn star2(input: Vec<&str>) {
    let mut total: u32 = 0;
    for line in input {
        let (first, second) = line.split_once(',').unwrap();
        let (first_min, first_max) = first.split_once('-').unwrap();
        let (second_min, second_max) = second.split_once('-').unwrap();
        let (first_min, first_max): (u32, u32) =
            (first_min.parse().unwrap(), first_max.parse().unwrap());
        let (second_min, second_max): (u32, u32) =
            (second_min.parse().unwrap(), second_max.parse().unwrap());
        if first_min >= second_min && first_min <= second_max
            || first_max <= second_max && first_max >= second_min
            || second_max <= first_max && second_max >= first_min
            || second_min >= first_min && second_min <= first_max
        {
            total += 1;
        }
    }
    println!("{}", total);
}
