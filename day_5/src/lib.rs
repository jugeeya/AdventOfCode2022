use std::collections::HashMap;
use utils::file_to_lines;

pub fn general_parser(filepath: &str, part_1: bool) -> Result<(), std::io::Error> {
    let mut stack_desc = true;
    let mut stacks: HashMap<usize, Vec<char>> = HashMap::new();
    let mut instructions: Vec<(usize, usize, usize)> = vec![];

    for line in file_to_lines(filepath)? {
        let txt = line.unwrap();
        if txt == "" {
            stack_desc = false;
            continue;
        }
        if stack_desc {
            for item_idx in txt
                .chars()
                .enumerate()
                .filter(|(_idx, c)| *c == '[')
                .map(|(idx, _c)| idx)
            {
                let item_idx = item_idx + 1;
                // Number of characters between stacks = 4
                let stack_num = ((item_idx - 1) / 4) + 1;
                let c = txt.chars().nth(item_idx).unwrap();
                let stack = stacks.get_mut(&stack_num);
                if stack.is_none() {
                    stacks.insert(stack_num, vec![c]);
                } else {
                    stack.unwrap().insert(0, c);
                }
            }
        } else {
            let mut space_split = txt.split(' ');
            let (_, num_to_move, _, stack_from, _, stack_to) = (
                space_split.next(),
                space_split.next(),
                space_split.next(),
                space_split.next(),
                space_split.next(),
                space_split.next(),
            );
            let num_to_move = num_to_move.unwrap().parse::<usize>().unwrap();
            let stack_from = stack_from.unwrap().parse::<usize>().unwrap();
            let stack_to = stack_to.unwrap().parse::<usize>().unwrap();
            instructions.push((num_to_move, stack_from, stack_to));
        }
    }

    for (num_to_move, stack_from, stack_to) in instructions {
        let mut temp_rev_stack: Vec<char> = vec![];
        if !part_1 {
            for _ in 0..num_to_move {
                temp_rev_stack.push(stacks.get_mut(&stack_from).unwrap().pop().unwrap());
            }
        }
        for _ in 0..num_to_move {
            let from = if part_1 {
                stacks.get_mut(&stack_from).unwrap().pop().unwrap()
            } else {
                temp_rev_stack.pop().unwrap()
            };
            stacks.get_mut(&stack_to).unwrap().push(from);
        }
    }

    let mut stack_keys = stacks.keys().collect::<Vec<&usize>>();
    stack_keys.sort();
    for stack_num in stack_keys {
        print!("{}", stacks.get(stack_num).unwrap().last().unwrap());
    }

    println!();

    Ok(())
}

pub fn part_1(filepath: &str) -> Result<(), std::io::Error> {
    general_parser(filepath, true)
}

pub fn part_2(filepath: &str) -> Result<(), std::io::Error> {
    general_parser(filepath, false)
}
