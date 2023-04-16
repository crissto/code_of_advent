use std::collections::HashSet;

use aoc_2022::read_input;

const DAY: i8 = 9;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct Pos {
    x: i32,
    y: i32,
}

fn move_tail(head: &Pos, tail: &mut Pos) {
    let dx = head.x - tail.x;
    let dy = head.y - tail.y;
    if (dx.abs() + dy.abs() > 1) && (dx.abs() > 1 || dy.abs() > 1) {
        tail.x += dx.signum();
        tail.y += dy.signum()
    }
}
fn main() {
    let input = read_input(DAY);
    let lines: Vec<&str> = input.lines().collect();
    // let mut visits: HashSet<Pos> = HashSet::with_capacity(10_000);
    let mut long_tail_visits: HashSet<Pos> = HashSet::with_capacity(10_000);

    let mut head = Pos { x: 0, y: 0 };
    let mut tails = [Pos { x: 0, y: 0 }; 9];

    for line in lines {
        let (dir, amount) = line.split_once(" ").unwrap();
        for _ in 0..amount.parse().unwrap() {
            match dir {
                "D" => head.y -= 1,
                "U" => head.y += 1,
                "L" => head.x -= 1,
                "R" => head.x += 1,
                _ => panic!("Welp"),
            }
            move_tail(&head, &mut tails[0]);
            // move_tail(&head, &mut tail);
            // visits.insert(tail);

            for i in 1..9 {
                let (left, right) = tails.split_at_mut(i);
                move_tail(&left[i - 1], &mut right[0]);
            }
            long_tail_visits.insert(tails[8]);
        }
    }

    println!("Tail visits {}", long_tail_visits.len());
}
