use std::{collections::HashMap, error::Error, fs};

fn main() -> Result<(), Box<dyn Error>> {
    let update_details = fs::read_to_string("day-5/day5_input.txt")?;
    let (page_order_rules, mut updates) = parse_update_details(&update_details);

    let (valid_update_idxs, corrected_update_idxs) =
        identify_valid_correct_invalid_updates(&page_order_rules, &mut updates);

    let mut valid_middle_page_numbers_sum = 0;
    for idx in valid_update_idxs {
        let current_update = &updates[idx];
        valid_middle_page_numbers_sum += current_update[current_update.len() / 2];
    }

    println!("Sum of middle page numbers from valid updates: {valid_middle_page_numbers_sum}");

    let mut corrected_middle_page_numbers_sum = 0;
    for idx in corrected_update_idxs {
        let current_update = &updates[idx];
        corrected_middle_page_numbers_sum += current_update[current_update.len() / 2];
    }

    println!(
        "Sum of middle page numbers from corrected invalid updates: {corrected_middle_page_numbers_sum}"
    );

    Ok(())
}

fn parse_update_details(update_details: &str) -> (HashMap<i32, Vec<i32>>, Vec<Vec<i32>>) {
    let mut page_order_rules = HashMap::new();
    let mut updates = Vec::new();

    let mut details_iter = update_details.chars().peekable();

    // Parse page ordering rules.
    let mut left_num_str = String::new();
    let mut right_num_str = String::new();
    let mut is_left_num = true;

    while let Some(next_char) = details_iter.next() {
        match next_char {
            '0'..='9' if is_left_num => left_num_str.push(next_char),
            '0'..='9' if !is_left_num => right_num_str.push(next_char),
            '|' => is_left_num = false,
            '\n' => {
                // Parse the page numbers.
                let left_num = left_num_str
                    .parse::<i32>()
                    .expect(&format!("Failed to parse {left_num_str} to an i32."));
                let right_num = right_num_str
                    .parse::<i32>()
                    .expect(&format!("Failed to parse {right_num_str} to an i32."));
                // Add new order rule to the HashMap.
                page_order_rules
                    .entry(left_num)
                    .and_modify(|pages: &mut Vec<i32>| {
                        pages.push(right_num);
                    })
                    .or_insert(vec![right_num]);
                // Cleanup state for next rule.
                left_num_str.clear();
                right_num_str.clear();
                is_left_num = true;
                // Exit page ordering rules section if double new lines are found.
                if details_iter.peek() == Some(&'\n') {
                    break;
                }
            }
            _ => unreachable!(
                "Only expect ascii digits, '|', or new line char in page ordering rules section."
            ),
        }
    }
    // Consume new line delimiter that separates the sections.
    details_iter.next();

    // Parse updates.
    let mut cur_num_str = String::new();
    let mut cur_update = Vec::new();

    while let Some(next_char) = details_iter.next() {
        match next_char {
            '0'..='9' => cur_num_str.push(next_char),
            ',' | '\n' => {
                let cur_num = cur_num_str
                    .parse::<i32>()
                    .expect(&format!("Failed to parse {cur_num_str} to an i32."));
                cur_update.push(cur_num);
                cur_num_str.clear();

                if next_char == '\n' {
                    updates.push(cur_update);
                    cur_update = Vec::new();
                }
            }
            _ => {
                unreachable!("Only expect ascii digits, ',', or new line char in updates section.")
            }
        }
    }

    (page_order_rules, updates)
}

fn identify_valid_correct_invalid_updates(
    page_order_rules: &HashMap<i32, Vec<i32>>,
    updates: &mut Vec<Vec<i32>>,
) -> (Vec<usize>, Vec<usize>) {
    let mut valid_update_idxs = Vec::new();
    let mut update_idxs_to_correct = Vec::new();

    for (update_idx, update) in updates.iter().enumerate() {
        let mut is_valid_update = true;
        // Validate page ordering for update.
        for page_idx in 0..update.len() {
            if let Some(order_rules) = page_order_rules.get(&update[page_idx]) {
                for previous_page_idx in 0..page_idx {
                    if order_rules.contains(&update[previous_page_idx]) {
                        is_valid_update = false;
                        break;
                    }
                }
            }

            if !is_valid_update {
                break;
            }
        }

        match is_valid_update {
            true => valid_update_idxs.push(update_idx),
            false => update_idxs_to_correct.push(update_idx),
        }
    }

    for invalid_update_idx in update_idxs_to_correct.iter() {
        correct_invalid_update(page_order_rules, &mut updates[*invalid_update_idx]);
    }

    (valid_update_idxs, update_idxs_to_correct)
}

fn correct_invalid_update(page_order_rules: &HashMap<i32, Vec<i32>>, update: &mut [i32]) {
    let mut made_swap = true;
    while made_swap {
        made_swap = false;
        for page_idx in 0..update.len() {
            if let Some(order_rules) = page_order_rules.get(&update[page_idx]) {
                for previous_page_idx in 0..page_idx {
                    if order_rules.contains(&update[previous_page_idx]) {
                        // Found Rule violation.
                        update.swap(previous_page_idx, page_idx);
                        made_swap = true;
                        break;
                    }
                }
            }
        }
    }
}
