use aoc_2022::read_input;
use regex::Regex;

const DAY: i8 = 4;

#[derive(Debug, Clone, Copy)]
struct ElfPair {
    start: usize,
    end: usize,
}

fn parse_line_into_elfs(line: &str) -> (ElfPair, ElfPair) {
    let re = Regex::new(
        r"(?P<first_start>\d*)-(?P<first_end>\d*),(?P<second_start>\d*)-(?P<second_end>\d*)",
    )
    .unwrap();

    let groups = re.captures(line).expect("Welp!");
    let first_elf = ElfPair {
        start: groups
            .name("first_start")
            .unwrap()
            .as_str()
            .parse::<usize>()
            .unwrap(),
        end: groups
            .name("first_end")
            .unwrap()
            .as_str()
            .parse::<usize>()
            .unwrap(),
    };
    let second_enf = ElfPair {
        start: groups
            .name("second_start")
            .unwrap()
            .as_str()
            .parse::<usize>()
            .unwrap(),
        end: groups
            .name("second_end")
            .unwrap()
            .as_str()
            .parse::<usize>()
            .unwrap(),
    };
    return (first_elf, second_enf);
}

fn check_if_pair_fully_contains(first: ElfPair, second: ElfPair) -> bool {
    (first.start <= second.start && first.end >= second.end)
        || (second.start <= first.start && second.end >= first.end)
}

fn check_if_number_in_range(start: usize, end: usize, number: usize) -> bool {
    start <= number && end >= number
}

fn check_if_pair_overlaps(first: ElfPair, second: ElfPair) -> bool {
    check_if_number_in_range(first.start, first.end, second.start)
        || check_if_number_in_range(first.start, first.end, second.end)
        || check_if_number_in_range(second.start, second.end, first.start)
        || check_if_number_in_range(second.start, second.end, first.end)
}

fn solution_1(input: String) -> usize {
    input
        .lines()
        .filter(|line| {
            let elves = parse_line_into_elfs(line);
            check_if_pair_fully_contains(elves.0, elves.1)
        })
        .count()
}

fn main() {
    let input = read_input(DAY);
    let overlaps = input
        .lines()
        .filter(|line| {
            let elves = parse_line_into_elfs(line);
            check_if_pair_overlaps(elves.0, elves.1)
        })
        .count();

    println!("{:?}", overlaps);
}
