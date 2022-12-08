use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::PathBuf;

use aoc_2022::read_input;

const DAY: i8 = 7;

#[derive(Debug)]
struct Command {
    command: String,
    output: Option<Vec<String>>,
}

type Commands = Vec<Command>;

fn parse_command(command: &str) -> Command {
    let mut command_with_output = command.split("\n");
    let clean_command = command_with_output.next().unwrap().to_string();
    let output: Vec<String> = command_with_output
        .filter(|s| s.len() > 0)
        .map(|a| a.to_string())
        .collect();

    Command {
        command: clean_command,
        output: if output.len() > 0 { Some(output) } else { None },
    }
}

fn go_through_commands(mut commands: Commands) -> HashMap<String, usize> {
    commands.drain(..1);
    let mut current_path = String::from("");
    let mut paths_to_size: HashMap<String, usize> = HashMap::new();

    for command in commands {
        if command.command == "cd .." {
            let path_parts: Vec<String> = current_path.split("/").map(|a| a.to_string()).collect();
            let new_path = path_parts[..path_parts.len() - 1].join("/").clone();
            current_path = new_path;
            continue;
        }
        if command.command.starts_with("cd ") {
            let split: Vec<&str> = command.command.split(" ").collect();
            current_path = format!("{current_path}/{}", split.last().unwrap());
        }
        if command.command.eq("ls") {
            for output in command.output.expect("Welp") {
                let mut split = output.split(" ");
                let type_or_size = split.next().unwrap();
                let name = split.next().unwrap().to_string();
                if type_or_size == "dir" {
                    continue;
                } else {
                    *paths_to_size.entry(current_path.clone()).or_insert(0) +=
                        type_or_size.parse::<usize>().expect("Failed parsing num");
                }
            }
        }
    }

    paths_to_size
}

fn old_main() {
    let input = read_input(DAY);
    let commands_with_output = input.split("$ ").filter(|x| x.len() > 0);
    let parsed_commands: Commands = commands_with_output.map(parse_command).collect();
    let map = go_through_commands(parsed_commands);

    let sum: Vec<(&String, &usize)> = map
        .iter()
        .filter(|(key, val)| val.to_owned().le(&(100_000 as usize)))
        .collect();
    println!("{}", serde_json::to_string(&sum).unwrap());
    let total = sum.iter().map(|item| item.1).sum::<usize>();
    println!("{:?}", total);
    // println!("{:?}", parsed_commands)
    // // let tree = parse_commands_into_tree(parsed_commands);
    // // println!("{:?}", tree);
}

fn main() {
    let input = read_input(DAY);

    // NOTE: needed capacity measured from **my** inputs; avoids re-allocations during processing
    let mut cwd = String::new();
    let mut map = HashMap::new();

    for line in input.lines().filter(|&l| l != "$ ls" && &l[0..3] != "dir") {
        match line {
            "$ cd /" => {
                // use ".", so we can use "/" as delimiter instead
                cwd.push('.');
            }
            "$ cd .." => {
                let i = cwd.rfind('/').unwrap();
                cwd.truncate(i);
            }
            _ if line.starts_with("$ cd") => {
                cwd.push('/');
                cwd.push_str(&line[5..]);
            }
            _ => {
                let fsize: u32 = line
                    .split_ascii_whitespace()
                    .next()
                    .unwrap()
                    .parse()
                    .unwrap();
                let dsize = map.entry(cwd.clone()).or_insert(0);
                *dsize += fsize;
                cwd.match_indices('/').for_each(|(i, _)| {
                    let dsize = map.entry(cwd[0..i].to_owned()).or_insert(0);
                    *dsize += fsize;
                });
            }
        }
    }

    println!(
        "First {}",
        map.values().filter(|&v| v <= &100_000).sum::<u32>()
    );

    let needed = 30_000_000 - (70_000_000 - map["."]);

    println!(
        "Second {}",
        map.values()
            .filter(|&v| v >= &needed)
            .min()
            .unwrap()
            .to_owned()
    );
}
