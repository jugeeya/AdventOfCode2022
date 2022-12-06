use crate::utils::file_to_lines;

fn line_to_ranges(line: &str) -> ((u64, u64), (u64, u64)) {
    let mut comma_split = line.split(',');
    let mut first_range = comma_split.next().unwrap().split('-');
    let mut second_range = comma_split.next().unwrap().split('-');

    let first_min = first_range.next().unwrap().parse::<u64>().unwrap();
    let first_max = first_range.next().unwrap().parse::<u64>().unwrap();
    let second_min = second_range.next().unwrap().parse::<u64>().unwrap();
    let second_max = second_range.next().unwrap().parse::<u64>().unwrap();
    ((first_min, first_max), (second_min, second_max))
}

fn line_contains_included_range(line: &str) -> bool {
    let ((first_min, first_max), (second_min, second_max)) = line_to_ranges(line);

    (first_min >= second_min && first_max <= second_max)
        || (second_min >= first_min && second_max <= first_max)
}

fn line_contains_any_included_range(line: &str) -> bool {
    let ((first_min, first_max), (second_min, second_max)) = line_to_ranges(line);

    (first_min >= second_min && first_min <= second_max)
        || (second_min >= first_min && second_min <= first_max)
}

pub fn part_1(filepath: &str) -> Result<(), std::io::Error> {
    let mut fully_included_ranges: u64 = 0;
    for line in file_to_lines(filepath)? {
        let txt = line.unwrap();
        if line_contains_included_range(&txt) {
            fully_included_ranges += 1;
        }
    }

    println!("Total Included Ranges: {fully_included_ranges}");

    Ok(())
}

pub fn part_2(filepath: &str) -> Result<(), std::io::Error> {
    let mut fully_included_ranges: u64 = 0;
    for line in file_to_lines(filepath)? {
        let txt = line.unwrap();
        if line_contains_any_included_range(&txt) {
            fully_included_ranges += 1;
        }
    }

    println!("Total Included Ranges: {fully_included_ranges}");

    Ok(())
}
