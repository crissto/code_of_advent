use std::usize;

use aoc_2022::read_input;
use regex::Regex;

const DAY: i8 = 5;

fn parse_moves(m: &str) -> (usize, usize, usize) {
    let re = Regex::new(r"move (?P<quantity>\d*) from (?P<from>\d) to (?P<to>\d)").unwrap();

    let groups = re.captures(m).expect("Welp!");

    (
        groups
            .name("quantity")
            .unwrap()
            .as_str()
            .parse::<usize>()
            .unwrap(),
        groups
            .name("from")
            .unwrap()
            .as_str()
            .parse::<usize>()
            .unwrap(),
        groups
            .name("to")
            .unwrap()
            .as_str()
            .parse::<usize>()
            .unwrap(),
    )
}

fn move_item_in_data(data: Vec<String>, from: usize, to: usize, quantity: usize) -> Vec<String> {
    let mut original_data = data.clone();

    let from_line: Vec<String> = data
        .get(from - 1)
        .unwrap()
        .split(" ")
        .map(|i| i.to_string())
        .collect();
    let mut to_line: Vec<String> = data
        .get(to - 1)
        .unwrap()
        .split(" ")
        .map(|i| i.to_string())
        .collect();

    let items: Vec<String> = from_line[from_line.len() - quantity..].to_vec();
    let final_from_line = from_line[0..from_line.len() - quantity].to_vec();

    original_data[from - 1] = final_from_line.join(" ");
    to_line.append(&mut items.clone());
    original_data[to - 1] = to_line.join(" ");
    original_data
}

fn main() {
    let input = read_input(DAY);
    let sections: Vec<&str> = input.split("\n\n").collect();
    let mut data: Vec<String> = sections[0].split("\n").map(|i| i.to_owned()).collect();
    let moves: Vec<&str> = sections[1].split("\n").collect();

    let parsed_moves: Vec<(usize, usize, usize)> = moves.iter().map(|m| parse_moves(m)).collect();
    // println!("data {:?} {quantity}", data);
    // data = move_item_in_data(data, from, to, quantity);
    for m in parsed_moves {
        let (quantity, from, to) = m;
        data = move_item_in_data(data, from, to, quantity);
    }
    println!("data {:?}", data);
}
