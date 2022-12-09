use crate::utils::file_to_lines;
use itertools::Itertools;

fn num_visible(trees: Vec<Vec<u32>>) {
    let row_len = trees.len();
    let num_visible = (0..row_len)
        .map(|row| {
            let col_len = trees[row].len();
            (0..col_len)
                .map(|col| {
                    if row == 0 || row == col_len - 1 || col == 0 || col == col_len - 1 {
                        return 1;
                    }

                    let curr_height = trees[row][col];
                    if [
                        (0..row).all(|row_idx| trees[row_idx][col] < curr_height),
                        (row + 1..row_len).all(|row_idx| trees[row_idx][col] < curr_height),
                        (0..col).all(|col_idx| trees[row][col_idx] < curr_height),
                        (col + 1..col_len).all(|col_idx| trees[row][col_idx] < curr_height),
                    ]
                    .iter()
                    .any(|vis| *vis)
                    {
                        return 1;
                    }

                    0
                })
                .sum::<u64>()
        })
        .sum::<u64>();

    println!("Number Visible: {num_visible}");
}

fn scenic_score(trees: Vec<Vec<u32>>) {
    let row_len = trees.len();
    let max_scenic_score = (0..row_len)
        .map(|row| {
            let col_len = trees[row].len();
            (0..col_len)
                .map(|col| {
                    if row == 0 || row == col_len - 1 || col == 0 || col == col_len - 1 {
                        return 0;
                    }

                    let curr_height = trees[row][col];
                    let mut all_left = (0..row).rev();
                    let mut all_up = row + 1..row_len;
                    let mut all_right = (0..col).rev();
                    let mut all_down = col + 1..col_len;

                    let left =
                        all_left.take_while_ref(|row_idx| trees[*row_idx][col] < curr_height);
                    let up = all_up.take_while_ref(|row_idx| trees[*row_idx][col] < curr_height);
                    let right =
                        all_right.take_while_ref(|col_idx| trees[row][*col_idx] < curr_height);
                    let down =
                        all_down.take_while_ref(|col_idx| trees[row][*col_idx] < curr_height);
                    let ret = [
                        left.count() + all_left.next().map(|_| 1).unwrap_or(0),
                        up.count() + all_up.next().map(|_| 1).unwrap_or(0),
                        right.count() + all_right.next().map(|_| 1).unwrap_or(0),
                        down.count() + all_down.next().map(|_| 1).unwrap_or(0),
                    ]
                    .iter()
                    .product::<usize>();
                    ret
                })
                .max()
                .unwrap()
        })
        .max()
        .unwrap();

    println!("Scenic Score: {max_scenic_score}");
}

pub fn general_parser(filepath: &str, part: usize) -> Result<(), std::io::Error> {
    let mut trees: Vec<Vec<u32>> = vec![];
    let mut row = 0;
    for line in file_to_lines(filepath)? {
        let txt = line.unwrap();
        for c in txt.chars() {
            let height = c.to_digit(10).unwrap();
            let tree_row = trees.get_mut(row);
            if tree_row.is_some() {
                tree_row.unwrap().push(height);
            } else {
                trees.push(vec![height]);
            }
        }
        row += 1;
    }

    if part == 1 {
        num_visible(trees);
    } else {
        scenic_score(trees);
    }

    Ok(())
}

pub fn part_1(filepath: &str) -> Result<(), std::io::Error> {
    general_parser(filepath, 1)
}

pub fn part_2(filepath: &str) -> Result<(), std::io::Error> {
    general_parser(filepath, 2)
}
