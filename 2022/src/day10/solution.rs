use aoc_2022::read_input;

const DAY: i8 = 10;

fn print(signals: &[i32]) {
    for (i, x) in signals[1..].iter().enumerate() {
        let i = (i as i32) % 40;
        if i == 0 {
            print!("\n")
        }
        if i - 2 < *x && *x < i + 2 {
            print!("#");
        } else {
            print!(".");
        }
    }
}
fn main() {
    let input = read_input(DAY);
    let lines: Vec<&str> = input.lines().collect();
    let mut signals = vec![0, 1];
    let mut val = 1;

    for line in lines {
        if line == "noop" {
            // println!("noop {:?}", pending_op);
            signals.extend(vec![val])
        } else if line.starts_with("addx") {
            let (_, op_val) = line.split_once(" ").unwrap();
            let old = val;
            val += op_val.parse::<i32>().unwrap();
            signals.extend(vec![old, val]);
        }
    }

    let part1: i32 = signals
        .iter()
        .enumerate()
        .skip(20)
        .step_by(40)
        .map(|(cycle, x)| {
            let parsed = cycle as i32;
            parsed * x
        })
        .sum();

    println!("{part1}");

    print(&signals);
}
