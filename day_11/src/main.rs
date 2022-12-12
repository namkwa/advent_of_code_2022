use std::fs;
fn main() {
    let input = fs::read_to_string("day_11/src/input.txt").unwrap();

    let input: Vec<&str> = input.split("\r\n\r\n").collect();
    //star1(&input);
    //star2(&input);
}

fn star1(input: Vec<&str>) {
    let monkeys: Vec<Monkey> = Vec::new();
    for monkey_information in input {
        monkeys.push(Monkey::new(monkey_information));
    }
}

struct Monkey {
    items: Vec<u32>,
    operation: Operation,
    divisibility: u32,
    monkey_to_throw_true: usize,
    monkey_to_throw_false: usize,
}

enum Operation {
    Add { value: u32 },
    Mutiply { value: u32 },
    Square,
}

impl Monkey {
    fn new(input: &str) -> Monkey {
        let lines: Vec<&str> = input.lines().collect();
        let items = lines
            .get(1)
            .unwrap()
            .split_once("items: ")
            .unwrap()
            .1
            .split(", ")
            .map(|x| x.parse::<u32>().unwrap())
            .collect();
        let divisibility = lines
            .get(3)
            .unwrap()
            .split_once("by ")
            .unwrap()
            .1
            .parse::<u32>()
            .unwrap();
        let monkey_to_throw_true = lines
            .get(4)
            .unwrap()
            .split_once("monkey ")
            .unwrap()
            .1
            .parse::<usize>()
            .unwrap();
        let monkey_to_throw_false = lines
            .get(5)
            .unwrap()
            .split_once("monkey ")
            .unwrap()
            .1
            .parse::<usize>()
            .unwrap();
        let operation: Operation;
        if lines.get(1).unwrap().contains("+") {
            let operation = lines
                .get(1)
                .unwrap()
                .split_once("+ ")
                .unwrap()
                .1
                .parse::<u32>()
                .unwrap();
        }
        Monkey {
            items: items,
            operation: operation,
            divisibility: divisibility,
            monkey_to_throw_true: monkey_to_throw_true,
            monkey_to_throw_false: monkey_to_throw_false,
        }
    }
}
