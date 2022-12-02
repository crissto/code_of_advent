use aoc_2022::read_input;

const DAY: i8 = 2;

const ROCK: i32 = 1;
const PAPER: i32 = 2;
const SCISSORS: i32 = 3;
const LOOSING: i32 = 0;
const DRAW: i32 = 3;
const WINNING: i32 = 6;

fn get_scores(elf_hand: &str, our_hand: &str) -> [i32; 2] {
    match (elf_hand, our_hand) {
        ("A", "X") => [ROCK + DRAW, SCISSORS + LOOSING],
        ("A", "Y") => [PAPER + WINNING, ROCK + DRAW],
        ("A", "Z") => [SCISSORS + LOOSING, PAPER + WINNING],
        ("B", "X") => [ROCK + LOOSING, ROCK + LOOSING],
        ("B", "Y") => [PAPER + DRAW, PAPER + DRAW],
        ("B", "Z") => [SCISSORS + WINNING, SCISSORS + WINNING],
        ("C", "X") => [ROCK + WINNING, PAPER + LOOSING],
        ("C", "Y") => [PAPER + LOOSING, SCISSORS + DRAW],
        ("C", "Z") => [SCISSORS + DRAW, ROCK + WINNING],
        (_, _) => panic!(
            "This shouldn't have happened. Hands {:?}, {:?}",
            elf_hand, our_hand
        ),
    }
}

fn main() {
    let input = read_input(DAY);

    let scores = input.lines().map(|line| {
        let mut hands = line.split(" ");
        get_scores(hands.next().unwrap(), hands.next().unwrap())
    });

    let part1_scores = scores.clone().map(|score| score[0]).sum::<i32>();
    let part2_scores = scores.clone().map(|score| score[1]).sum::<i32>();

    println!("The score for part one is {:?};", part1_scores);
    println!("The score for part two is {:?};", part2_scores);
}
