use std::{error::Error, fs};

fn main() -> Result<(), Box<dyn Error>> {
    // NOTE: last char in disk_map is a new line char.
    let disk_map = fs::read_to_string("day-9/day9_input.txt")?;

    let mut block_layout_v1 = construct_block_layout(&disk_map[..disk_map.len() - 1]);
    compact_files_v1(&mut block_layout_v1);
    let checksum_v1 = calculate_checksum(&block_layout_v1);

    println!("Resulting filesystem checksum after v1 compaction: {checksum_v1}");

    let mut block_layout_v2 = construct_block_layout(&disk_map[..disk_map.len() - 1]);
    compact_files_v2(&mut block_layout_v2);
    let checksum_v2 = calculate_checksum(&block_layout_v2);

    println!("Resulting filesystem checksum after v2 compaction: {checksum_v2}");

    Ok(())
}

fn construct_block_layout(disk_map: &str) -> Vec<i64> {
    let mut block_layout = Vec::new();
    let mut current_file_id = 0;
    let mut is_file_length = true;

    for c in disk_map.chars() {
        let block_length =
            c.to_digit(10)
                .expect(&format!("Failed to parse {c} to a decimal digit.")) as usize;
        match is_file_length {
            true => {
                block_layout.append(&mut vec![current_file_id; block_length]);
                current_file_id += 1;
            }
            // Uses -1 to represent an empty block.
            false => block_layout.append(&mut vec![-1; block_length]),
        }

        is_file_length = !is_file_length;
    }

    block_layout
}

fn compact_files_v1(block_layout: &mut [i64]) {
    let mut left_idx = 0;
    let mut right_idx = block_layout.len() - 1;

    while left_idx < right_idx {
        if block_layout[right_idx] != -1 {
            // Move left_idx to leftmost empty block (represented by -1).
            while block_layout[left_idx] != -1 && left_idx < right_idx {
                left_idx += 1;
            }
            block_layout.swap(left_idx, right_idx);
        }
        right_idx -= 1;
    }
}

fn compact_files_v2(block_layout: &mut [i64]) {
    let mut file_idx = block_layout.len() - 1;

    loop {
        if block_layout[file_idx] != -1 {
            // Calculate block size of current file.
            let cur_file_id = block_layout[file_idx];
            let mut file_block_size = 0;
            while block_layout[file_idx] == cur_file_id && file_idx > 0 {
                file_block_size += 1;
                file_idx -= 1;
            }
            if file_idx == 0 {
                break;
            }
            file_idx += 1;
            // Check if a large enough free space exists to its left.
            let mut free_block_idx = 0;
            let mut free_block_size = 0;
            while free_block_idx <= file_idx {
                if free_block_size == file_block_size {
                    let free_block_start = free_block_idx - free_block_size as usize;
                    for offset in 0..file_block_size {
                        block_layout.swap(free_block_start + offset, file_idx + offset);
                    }
                    break;
                }
                match block_layout[free_block_idx] {
                    -1 => free_block_size += 1,
                    _ => free_block_size = 0,
                }
                free_block_idx += 1;
            }
        }
        if file_idx == 0 {
            break;
        }
        file_idx -= 1;
    }
}

fn calculate_checksum(block_layout: &[i64]) -> usize {
    let mut checksum = 0;
    for idx in 0..block_layout.len() {
        if block_layout[idx] != -1 {
            checksum += idx * block_layout[idx] as usize;
        }
    }

    checksum
}
