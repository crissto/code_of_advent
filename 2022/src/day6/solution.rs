use aoc_2022::read_input;

const DAY: i8 = 6;

fn main() {
    let input = read_input(DAY);

    let wsize = 14;
    let solution = input
        .as_bytes()
        .windows(wsize)
        .position(|s| !(1..s.len()).any(|i| s[i..].contains(&s[i - 1])))
        .unwrap()
        + wsize;

    println!("{:?}", solution);
}
