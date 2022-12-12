use std::fs;
fn main() {
    let input = fs::read_to_string("day_11/src/input.txt").unwrap();

    let input: Vec<&str> = input.split("\r\n\r\n").collect();
    star1(input);
    //star2(&input);
}

fn star1(input: Vec<&str>) {
    let mut monkeys: Vec<Monkey> = Vec::new();
    let mut totals: Vec<u64> = Vec::new();
    for monkey_information in input {
        monkeys.push(Monkey::new(monkey_information));
        totals.push(0);
    }
    let mut total_modulo: u64 = 1;
    for monkey in 0..monkeys.len() {
        total_modulo *= monkeys.get(monkey).unwrap().divisibility;
    }
    for _ in 0..10000 {
        for index_monkey in 0..monkeys.len() {
            let current_monkey: Monkey = monkeys.get(index_monkey).unwrap().clone();
            for index_item in 0..current_monkey.items.len() {
                *totals.get_mut(index_monkey).unwrap() += 1;
                let new_item = apply_operation(
                    monkeys
                        .get(index_monkey)
                        .unwrap()
                        .items
                        .get(index_item)
                        .unwrap(),
                    &monkeys.get(index_monkey).unwrap().operation,
                );
                if new_item % current_monkey.divisibility == 0 {
                    monkeys
                        .get_mut(current_monkey.monkey_to_throw_true)
                        .unwrap()
                        .items
                        .push(new_item % total_modulo);
                } else {
                    monkeys
                        .get_mut(current_monkey.monkey_to_throw_false)
                        .unwrap()
                        .items
                        .push(new_item % total_modulo);
                }
            }
            monkeys.get_mut(index_monkey).unwrap().items.clear();
        }
    }
    totals.sort();
    totals.reverse();
    println!("{:?}", totals.get(0).unwrap() * totals.get(1).unwrap());
}
#[derive(Debug, Clone)]
struct Monkey {
    items: Vec<u64>,
    operation: Operation,
    divisibility: u64,
    monkey_to_throw_true: usize,
    monkey_to_throw_false: usize,
}

#[derive(Debug, Copy, Clone)]
enum Operation {
    Add { value: u64 },
    Mutiply { value: u64 },
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
            .map(|x| x.parse::<u64>().unwrap())
            .collect();
        let divisibility = lines
            .get(3)
            .unwrap()
            .split_once("by ")
            .unwrap()
            .1
            .parse::<u64>()
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
        let mut operation: Operation = Operation::Square;
        if lines.get(2).unwrap().contains("+") {
            operation = Operation::Add {
                value: lines
                    .get(2)
                    .unwrap()
                    .split_once("+ ")
                    .unwrap()
                    .1
                    .parse::<u64>()
                    .unwrap(),
            };
        } else if !lines
            .get(2)
            .unwrap()
            .split_once("* ")
            .unwrap()
            .1
            .contains("old")
        {
            operation = Operation::Mutiply {
                value: lines
                    .get(2)
                    .unwrap()
                    .split_once("* ")
                    .unwrap()
                    .1
                    .parse::<u64>()
                    .unwrap(),
            }
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

fn apply_operation(item: &u64, operation: &Operation) -> u64 {
    match *operation {
        Operation::Add { value } => return *item + value,
        Operation::Mutiply { value } => return *item * value,
        Operation::Square => return *item * *item,
    }
}
