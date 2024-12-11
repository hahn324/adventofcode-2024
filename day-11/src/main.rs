use std::{collections::HashMap, error::Error, fs};

fn main() -> Result<(), Box<dyn Error>> {
    let stone_line = fs::read_to_string("day-11/day11_input.txt")?;
    let stones = parse_stone_line(&stone_line);

    let mut blink_cache = HashMap::new();

    let mut total_after_25_blinks = 0;
    for stone in stones.iter() {
        total_after_25_blinks += blink(*stone, 25, &mut blink_cache);
    }
    println!("{total_after_25_blinks} stones after 25 blinks.");

    let mut total_after_75_blinks = 0;
    for stone in stones.iter() {
        total_after_75_blinks += blink(*stone, 75, &mut blink_cache);
    }
    println!("{total_after_75_blinks} stones after 75 blinks.");

    Ok(())
}

fn parse_stone_line(stone_line: &str) -> Vec<usize> {
    let mut stones = Vec::new();

    let mut current_idx = 0;
    let mut start_idx = 0;

    for c in stone_line.chars() {
        match c {
            ' ' | '\n' => {
                stones.push(stone_line[start_idx..current_idx].parse().unwrap());
                start_idx = current_idx + 1;
            }
            _ => (),
        }
        current_idx += 1;
    }

    stones
}

fn blink(
    stone_num: usize,
    num_blinks: usize,
    blink_cache: &mut HashMap<(usize, usize), usize>,
) -> usize {
    let cache_key = (stone_num, num_blinks);
    if let Some(&num_stones) = blink_cache.get(&cache_key) {
        return num_stones;
    }

    let num_stones = match num_blinks {
        1 => {
            if stone_num != 0 && (stone_num.ilog10() + 1) & 1 == 0 {
                2
            } else {
                1
            }
        }
        _ => match stone_num {
            0 => blink(1, num_blinks - 1, blink_cache),
            _ if (stone_num.ilog10() + 1) & 1 == 0 => {
                let num_digits = stone_num.ilog10() + 1;
                let left_half = stone_num / 10usize.pow(num_digits / 2);
                let left_res = blink(left_half, num_blinks - 1, blink_cache);

                let right_half = stone_num % 10usize.pow(num_digits / 2);
                let right_res = blink(right_half, num_blinks - 1, blink_cache);

                left_res + right_res
            }
            _ => blink(stone_num * 2024, num_blinks - 1, blink_cache),
        },
    };

    blink_cache.insert(cache_key, num_stones);

    num_stones
}
