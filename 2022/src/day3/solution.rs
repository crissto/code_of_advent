use std::collections::HashMap;

use aoc_2022::read_input;

const DAY: i8 = 3;

fn find_common_in_two(first: &str, second: &str) -> char {
    let vec: Vec<char> = first
        .chars()
        .filter(|char| second.contains(*char))
        .collect();

    *vec.first().unwrap()
}

fn char_to_ascii_number(character: char) -> u32 {
    if character.is_lowercase() {
        character as u32 - 96
    } else {
        character as u32 - 65 + 27
    }
}

fn first_solution() -> u32 {
    let input = read_input(DAY);
    let chars: Vec<u32> = input
        .lines()
        .map(|line| {
            let common = find_common_in_two(&line[..line.len() / 2], &line[line.len() / 2..]);
            char_to_ascii_number(common)
        })
        .collect();

    chars.iter().sum::<u32>()
}

fn main() {
    let input = read_input(DAY);
    let lines: Vec<&str> = input.lines().collect();
    let chunks = lines.chunks(3);

    let similar_by_chunks = chunks.map(|chunk| {
        let first = chunk.first().unwrap();
        let second = chunk.get(1).unwrap();
        let third = chunk.get(2).unwrap();

        let similar: Vec<char> = first
            .chars()
            .filter(|item| second.contains(*item) && third.contains(*item))
            .collect();

        char_to_ascii_number(*similar.first().expect("Welp"))
    });

    println!("Sum is {:?}", similar_by_chunks.sum::<u32>())
}
