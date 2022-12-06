use crate::utils::file_to_lines;

fn get_repeating_char(s: &str) -> char {
    let middle_idx = s.len() / 2;
    let first = &s[0..middle_idx];
    let second = &s[middle_idx..];

    for c in first.chars() {
        if second.contains(c) {
            return c;
        }
    }

    panic!("Unable to find duplicate between {first} and {second}!");
}

fn char_to_priority(c: char) -> u64 {
    (
        // 1-indexed
        1 + 
        // Char to 0..=25 range
        c.to_lowercase().next().unwrap() as u32 - 'a' as u32 + 
        // Uppercase letters are after
        if c.is_lowercase() { 
            0
        } else { 
            26
        }
    ) as u64
}

pub fn part_1(filepath: &str) -> Result<(), std::io::Error> {
    let mut total_priorities : u64 = 0;
    for line in file_to_lines(filepath)? {
        let txt = line.unwrap();
        total_priorities += char_to_priority(get_repeating_char(&txt));
    }

    println!("Total Priorities: {total_priorities}");

    Ok(())
}

fn get_repeating_chars(first: &str, second: &str) -> Vec<char>  {
    let mut chars = vec![];
    for c in first.chars() {
        if second.contains(c) {
            chars.push(c);
        }
    }

    chars
}

fn get_repeating_char_from_chunk(first: &str, second: &str, third: &str) -> char {
    let first_chars = get_repeating_chars(first, second);
    let second_chars = get_repeating_chars(second, third);

    for c in &first_chars {
        if second_chars.contains(c) {
            return *c;
        }
    }

    panic!("Unable to find duplicate between {first_chars:#?} and {second_chars:#?}!");
}

pub fn part_2(filepath: &str) -> Result<(), std::io::Error> {
    let mut total_priorities : u64 = 0;
    let mut lines = file_to_lines(filepath).unwrap();
    while let (Some(first), Some(second), Some(third)) = (lines.next(), lines.next(), lines.next()) {
        total_priorities += char_to_priority(get_repeating_char_from_chunk(&first.unwrap(), &second.unwrap(), &third.unwrap()));
    }

    println!("Total Priorities: {total_priorities}");

    Ok(())
}
