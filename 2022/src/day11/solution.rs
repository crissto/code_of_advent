use std::collections::HashMap;

use aoc_2022::read_input;

const DAY: i8 = 11;
const ROUNDS: i8 = 20;

fn eval_expression(exp: String) -> u128 {
    let split: Vec<&str> = exp.split(" ").collect();
    if split.len() != 3 {
        panic!("No no");
    }

    let first = split[0].parse::<u128>().unwrap();
    let second = split[2].parse::<u128>().unwrap();

    match split[1] {
        "+" => first + second,
        "*" => first * second,
        "-" => first - second,
        "/" => first / second,
        _ => panic!("WAT"),
    }
}

#[derive(Debug, Clone)]
struct MonkeyData {
    id: i8,
    initial_items: Vec<u128>,
    operation: String,
    divisible_by: i8,
    to_if_true: i8,
    to_if_false: i8,
}

fn parse_monkey(monkey: &str) -> MonkeyData {
    let monkey_lines = monkey.split("\n").collect::<Vec<&str>>();
    let id = monkey_lines[0]
        .split_once(" ")
        .unwrap()
        .1
        .strip_suffix(":")
        .unwrap()
        .parse::<i8>()
        .unwrap();
    let initial_items: Vec<u128> = monkey_lines[1]
        .split_once(": ")
        .unwrap()
        .1
        .split(", ")
        .map(|item| item.parse::<u128>().unwrap())
        .collect();
    let operation = monkey_lines[2].split_once(" = ").unwrap().1.to_string();

    let divisible_by = monkey_lines[3]
        .split_once("Test: divisible by ")
        .unwrap()
        .1
        .parse::<i8>()
        .unwrap();
    let to_if_true = monkey_lines[4]
        .split_once("If true: throw to monkey ")
        .unwrap()
        .1
        .parse::<i8>()
        .unwrap();

    let to_if_false = monkey_lines[5]
        .split_once("If false: throw to monkey ")
        .unwrap()
        .1
        .parse::<i8>()
        .unwrap();
    // let operated = eval_expression(operation);
    // if (operated as i32).rem_euclid(divisible_by.try_into().unwrap()) == 0 {
    //     monkeys.entry(if_true).or_insert(Vec::new()).push(operated);
    //     println!("Monkey {id} passed {operated} to {if_true}");
    // } else {
    //     let floored = (operated as f64 / 3.0).floor() as usize;
    //     monkeys.entry(if_false).or_insert(Vec::new()).push(floored);
    //     println!("Monkey {id} passed {operated} to {if_false}");
    // }

    MonkeyData {
        id,
        initial_items,
        operation,
        divisible_by,
        to_if_true,
        to_if_false,
    }
}

fn main() {
    let input = read_input(DAY);
    let mut monkeys: Vec<MonkeyData> = input.split("\n\n").map(parse_monkey).collect();
    let mut monkey_inspections: HashMap<i8, usize> = HashMap::with_capacity(10);
    let mut monkey_items: Vec<Vec<u128>> =
        monkeys.iter().map(|m| m.initial_items.clone()).collect();

    for round in 0..2 {
        for monkey in monkeys {
            for item in monkey_items[monkey.id as usize] {
                let operation = monkey.operation.clone().replace("old", &item.to_string());
                let operated = eval_expression(operation);
                if (operated as i32).rem_euclid(monkey.divisible_by.try_into().unwrap()) == 0 {
                    monkeys[monkey.to_if_true as usize]
                        .initial_items
                        .push(operated);
                } else {
                    let floored = (operated as f64 / 3.0).floor() as usize;
                    monkeys[monkey.to_if_false as usize]
                        .initial_items
                        .push(floored as u128);
                }
            }
            println!("{:?}", monkeys)
        }
        println!("{round} {:?}", monkey_inspections)
    }
}
