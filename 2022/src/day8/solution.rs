use aoc_2022::read_input;

const DAY: i8 = 8;

fn get_col_as_row(matrix: &Vec<Vec<i32>>, col: usize) -> Vec<i32> {
    return matrix.iter().map(|row| row[col]).collect();
}

fn is_visible(matrix: &Vec<Vec<i32>>, x: usize, y: usize) -> bool {
    let row = &matrix[y];
    let col = get_col_as_row(matrix, x);
    let item_height = matrix[y][x];

    let covered_from_left = row[..x].iter().any(|&item| item >= item_height);
    let covered_from_right = row[x + 1..].iter().any(|&item| item >= item_height);
    let covered_from_top = col[..y].iter().any(|&item| item >= item_height);
    let covered_from_bottom = col[y + 1..].iter().any(|&item| item >= item_height);

    return [
        covered_from_top,
        covered_from_right,
        covered_from_bottom,
        covered_from_left,
    ]
    .iter()
    .all(|&x| x);
}

fn get_scenic_score(matrix: &Vec<Vec<i32>>, x: usize, y: usize) -> usize {
    let row = &matrix[y];
    let col = get_col_as_row(matrix, x);
    let item_height = matrix[y][x];

    if x == 0 || y == 0 || y == col.len() || x == row.len() {
        return 0;
    }

    let index_covered_left = row[..x - 1].iter().rev().position(|&x| x >= item_height);
    let index_covered_right = row[x + 1..].iter().position(|&x| x >= item_height);
    let index_covered_top = col[..y - 1].iter().rev().position(|&x| x >= item_height);
    let index_covered_bottom = col[y + 1..].iter().position(|&x| x >= item_height);

    return (match index_covered_top {
        Some(x) => x + 1,
        None => y,
    } * match index_covered_right {
        Some(x) => row.len() - x - 1,
        None => x,
    } * match index_covered_bottom {
        Some(x) => col.len() - y - 1,
        None => y,
    } * match index_covered_left {
        Some(x) => x + 1,
        None => x,
    });
}

fn main() {
    let input = read_input(DAY);
    let lines = input.lines();
    let matrix: Vec<Vec<i32>> = lines
        .map(|row| {
            row.split("")
                .filter_map(|item| match item {
                    "" => None,
                    _ => Some(item.parse::<i32>().unwrap()),
                })
                .collect()
        })
        .collect();

    let mut total_count: usize = 0;
    let mut scenic_scores: Vec<usize> = Vec::new();

    for row_index in 0..matrix.len() {
        for col_index in 0..matrix[row_index].len() {
            let visible = if is_visible(&matrix, col_index, row_index) {
                0
            } else {
                1
            };
            total_count += visible;

            let scenic_score = get_scenic_score(&matrix, col_index, row_index);
            scenic_scores.push(scenic_score);
        }
    }

    println!("{}", scenic_scores.iter().max().unwrap());
}
