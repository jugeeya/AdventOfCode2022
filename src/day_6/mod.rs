use std::collections::HashSet;
use crate::utils::file_to_lines;

pub fn general_parser(filepath: &str, num_chars: usize) -> Result<(), std::io::Error> {
    let mut chars_seen : Vec<char> = vec![];

    for line in file_to_lines(filepath)? {
        let txt = line.unwrap();
        for (idx, c) in txt.chars().enumerate() {
            if chars_seen.len() == num_chars {
                chars_seen.remove(0);
            }
            chars_seen.push(c);
            if chars_seen.len() == num_chars {
                let mut unique_set = HashSet::new();
                chars_seen.iter().for_each(|c| { unique_set.insert(c); } );
                if unique_set.len() == num_chars {
                    println!("{}", idx+1);
                    return Ok(());
                }
            }
        }
    }

    Ok(())
}

pub fn part_1(filepath: &str) -> Result<(), std::io::Error> {
    general_parser(filepath, 4)
}


pub fn part_2(filepath: &str) -> Result<(), std::io::Error> {
    general_parser(filepath, 14)
}
