use std::{error::Error, fs};

fn main() -> Result<(), Box<dyn Error>> {
    let mut keys = vec![];
    let mut locks = vec![];
    fs::read("day-25/day25_input.txt")?
        .chunks(43)
        .for_each(|scheme| {
            let mut heights: [u8; 5] = [0, 0, 0, 0, 0];
            for idx in (6..36).step_by(6) {
                for col_offset in 0..5 {
                    if scheme[idx + col_offset] == b'#' {
                        heights[col_offset] += 1;
                    }
                }
            }
            if scheme[0] == b'.' {
                keys.push(heights);
            } else {
                locks.push(heights);
            }
        });

    let mut unique_lock_key_pairs = 0;
    for key in keys.iter() {
        for lock in locks.iter() {
            let mut key_fits = true;
            for idx in 0..5 {
                if key[idx] + lock[idx] > 5 {
                    key_fits = false;
                    break;
                }
            }
            if key_fits {
                unique_lock_key_pairs += 1;
            }
        }
    }

    println!("Number of possible lock/key pairs: {unique_lock_key_pairs}");

    Ok(())
}
