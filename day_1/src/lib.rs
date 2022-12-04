use advent_utils::file_to_lines;

pub fn part_1(filepath: &str) -> Result<(), std::io::Error> {
    let mut max_calories : u64 = 0;
    let mut curr_calories : u64 = 0;
    for line in file_to_lines(filepath)? {
        let txt = line.unwrap();
        if txt == "" {
            if curr_calories > max_calories {
                max_calories = curr_calories;
            }
            curr_calories = 0;
            continue;
        }
        let calories = txt.parse::<u64>().unwrap();
        curr_calories += calories;
    }

    // Check for last entry
    if curr_calories > max_calories {
        max_calories = curr_calories;
    }

    println!("Max Calories: {max_calories}");

    Ok(())
}

fn shift_vec_at_idx(v: &mut Vec<u64>, idx: usize) {
    for shift_idx in (idx..v.len()).rev() {
        if shift_idx == 0 {
            continue;
        }
        v[shift_idx] = v[shift_idx-1];
    }
}

#[test]
fn test_shift_vec_at_idx() {
    // Shift everything down
    let mut v = vec![500,400,300,200,100];
    shift_vec_at_idx(&mut v, 0);
    assert_eq!(v, vec![500,500,400,300,200]);

    // Only last two elements
    let mut v = vec![500,400,300,200,100];
    shift_vec_at_idx(&mut v, 3);
    assert_eq!(v, vec![500,400,300,300,200]);

    // Only last elements
    let mut v = vec![500,400,300,200,100];
    shift_vec_at_idx(&mut v, 4);
    assert_eq!(v, vec![500,400,300,200,200]);
}

fn update_top_3(max_calories: &mut Vec<u64>, curr_calories: u64) {
    for idx in 0..max_calories.len() {
        let existing_max_calories = max_calories[idx];
        if curr_calories > existing_max_calories {
            // Shift out remaining vals
            shift_vec_at_idx(max_calories, idx);
            max_calories[idx] = curr_calories;

            break;
        }
    }
}

pub fn part_2(filepath: &str) -> Result<(), std::io::Error> {
    let mut max_calories : Vec<u64> = vec![0, 0, 0];
    let mut curr_calories : u64 = 0;
    for line in file_to_lines(filepath)? {
        let txt = line.unwrap();
        if txt == "" {
            update_top_3(&mut max_calories, curr_calories);
            curr_calories = 0;
            continue;
        }
        let calories = txt.parse::<u64>().unwrap();
        curr_calories += calories;
    }

    // Check for last entry
    update_top_3(&mut max_calories, curr_calories);

    println!("Max Calories: {}", max_calories.iter().sum::<u64>());

    Ok(())
}