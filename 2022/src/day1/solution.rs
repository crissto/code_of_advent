use aoc_2022::read_input;

const DAY: i8 = 1;

pub fn main() {
    let input = read_input(DAY);
    let bag_of_calories_by_elf = input.split("\n\n");
    let mut total_calories_by_elf: Vec<i64> = bag_of_calories_by_elf
        .map(|calories| calories.split("\n").flat_map(|x| x.parse::<i64>()).sum())
        .collect();

    total_calories_by_elf.sort();

    let top_three_elves = total_calories_by_elf.iter().rev().take(3).sum::<i64>();
    let top_elf = total_calories_by_elf.iter().rev().take(1).sum::<i64>();

    println!("The top elf has {:?} calories", top_elf);
    println!(
        "The top 3 elves have {:?} calories in total",
        top_three_elves
    );
}
