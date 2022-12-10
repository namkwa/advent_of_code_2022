use std::fs;
fn main() {
    let input = fs::read_to_string("day_10/src/input.txt").unwrap();

    let input: Vec<&str> = input.lines().collect();
    star1(&input);
    star2(&input);
}

fn star1(input: &Vec<&str>) {
    let mut clock = 0;
    let mut x = 1;
    let mut total = 0;
    let values: Vec<i32> = (20..221).step_by(40).collect();
    for line in input {
        let parsed_value = line.split_once(" ");
        if parsed_value == None {
            clock += 1;
            if values.contains(&clock) {
                println!("{} {}", &clock, &x);
                total += &clock * x;
            }
        } else {
            for _ in 1..3 {
                clock += 1;
                if values.contains(&clock) {
                    println!("{} {}", &clock, &x);
                    total += &clock * x;
                }
            }
            x += parsed_value.unwrap().1.parse::<i32>().unwrap();
        }
    }
    println!("{}", total);
}

fn star2(input: &Vec<&str>) {
    let mut clock = 0;
    let mut x: i32 = 1;
    let mut crt_screen: Vec<Vec<char>> = construct_crt(6, 40);
    for line in input {
        let parsed_value = line.split_once(" ");
        if parsed_value == None {
            if clock % 40 <= x + 1 && clock % 40 >= x - 1 {
                crt_screen[(clock / 40) as usize][(clock % 40) as usize] = '#';
            }
            clock += 1;
        } else {
            for _ in 1..3 {
                if clock % 40 <= x + 1 && clock % 40 >= x - 1 {
                    crt_screen[(clock / 40) as usize][(clock % 40) as usize] = '#';
                }
                clock += 1;
            }
            x += parsed_value.unwrap().1.parse::<i32>().unwrap();
        }
    }
    for line in crt_screen {
        println!("{:?}", line);
    }
}

fn construct_crt(x: usize, y: usize) -> Vec<Vec<char>> {
    let mut output: Vec<Vec<char>> = Vec::new();
    for i in 0..x {
        output.push(Vec::new());
        for _ in 0..y {
            output[i].push('.');
        }
    }
    return output;
}
