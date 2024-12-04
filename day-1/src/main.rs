use std::collections::HashMap;
use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let input_lists = fs::read_to_string("day-1/day1_input.txt")?;

    let mut left_list = Vec::new();

    let mut right_list = Vec::new();
    let mut right_hash_map = HashMap::new();

    // Starting index of first number.
    let mut num_idx_start = 0;
    // Starts reading from first number in left list.
    let mut is_left_num = true;

    for (cur_idx, cur_char) in input_lists.chars().enumerate() {
        if cur_char == '\n' || (cur_char == ' ' && is_left_num) {
            let current_num = input_lists[num_idx_start..cur_idx].parse::<i32>()?;
            match is_left_num {
                true => {
                    left_list.push(current_num);
                    // Delimiter is 3 space chars.
                    num_idx_start = cur_idx + 3;
                }
                false => {
                    right_list.push(current_num);
                    right_hash_map
                        .entry(current_num)
                        .and_modify(|counter| *counter += 1)
                        .or_insert(1);
                    // Delimiter is just newline char.
                    num_idx_start = cur_idx + 1;
                }
            }
            is_left_num = !is_left_num;
        }
    }

    left_list.sort();
    right_list.sort();

    let mut total_distance = 0;
    let mut similarity_score = 0;

    for idx in 0..left_list.len() {
        let left_value = left_list[idx];

        total_distance += (left_value - right_list[idx]).abs();

        if let Some(&num_appearances) = right_hash_map.get(&left_value) {
            similarity_score += left_value * num_appearances;
        }
    }

    println!("Total Distance: {total_distance}");
    println!("Similarity Score: {similarity_score}");

    Ok(())
}
