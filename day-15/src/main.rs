use std::{error::Error, fs};

fn main() -> Result<(), Box<dyn Error>> {
    let input_source = fs::read_to_string("day-15/day15_input.txt")?;
    let mut split_source = input_source.split("\n\n");

    let mut warehouse_1: Vec<Vec<char>> = split_source
        .next()
        .unwrap()
        .split('\n')
        .map(|row| row.chars().collect())
        .collect();
    let mut warehouse_2 = construct_warehouse_2(&warehouse_1);

    let moves = split_source.next().unwrap();

    let warehouse_1_sum_of_coords = warehouse_1_sum_box_gps_coords(&mut warehouse_1, moves);
    println!("Sum of all boxes' GPS coordinates in warehouse 1: {warehouse_1_sum_of_coords}");

    let warehouse_2_sum_of_coords = warehouse_2_sum_box_gps_coords(&mut warehouse_2, moves);
    println!("Sum of all boxes' GPS coordinates in warehouse 2: {warehouse_2_sum_of_coords}");

    Ok(())
}

fn warehouse_1_sum_box_gps_coords(warehouse: &mut Vec<Vec<char>>, moves: &str) -> usize {
    let mut robot_pos = (0, 0);
    let mut found_robot = false;
    for row in 0..warehouse.len() {
        for col in 0..warehouse[row].len() {
            if warehouse[row][col] == '@' {
                robot_pos = (row, col);
                found_robot = true;
                break;
            }
        }
        if found_robot {
            break;
        }
    }

    for c in moves.chars() {
        match c {
            '^' => warehouse_1_move_up(warehouse, &mut robot_pos),
            '>' => warehouse_1_move_right(warehouse, &mut robot_pos),
            'v' => warehouse_1_move_down(warehouse, &mut robot_pos),
            '<' => warehouse_1_move_left(warehouse, &mut robot_pos),
            _ => (),
        }
    }
    let mut gps_sum = 0;

    for row in 0..warehouse.len() {
        for col in 0..warehouse[row].len() {
            if warehouse[row][col] == 'O' {
                gps_sum += 100 * row + col;
            }
        }
    }

    gps_sum
}

fn warehouse_1_move_up(warehouse: &mut Vec<Vec<char>>, pos: &mut (usize, usize)) {
    match warehouse[pos.0 - 1][pos.1] {
        '.' => {
            warehouse[pos.0][pos.1] = '.';
            warehouse[pos.0 - 1][pos.1] = '@';
            pos.0 = pos.0 - 1;
        }
        'O' => {
            let mut offset = 2;
            loop {
                match warehouse[pos.0 - offset][pos.1] {
                    '#' => break,
                    '.' => {
                        warehouse[pos.0][pos.1] = '.';
                        warehouse[pos.0 - 1][pos.1] = '@';
                        for box_offset in 2..=offset {
                            warehouse[pos.0 - box_offset][pos.1] = 'O';
                        }
                        pos.0 = pos.0 - 1;
                        break;
                    }
                    _ => (),
                }
                offset += 1;
            }
        }
        _ => (),
    }
}

fn warehouse_1_move_right(warehouse: &mut Vec<Vec<char>>, pos: &mut (usize, usize)) {
    match warehouse[pos.0][pos.1 + 1] {
        '.' => {
            warehouse[pos.0][pos.1] = '.';
            warehouse[pos.0][pos.1 + 1] = '@';
            pos.1 = pos.1 + 1;
        }
        'O' => {
            let mut offset = 2;
            loop {
                match warehouse[pos.0][pos.1 + offset] {
                    '#' => break,
                    '.' => {
                        warehouse[pos.0][pos.1] = '.';
                        warehouse[pos.0][pos.1 + 1] = '@';
                        for box_offset in 2..=offset {
                            warehouse[pos.0][pos.1 + box_offset] = 'O';
                        }
                        pos.1 = pos.1 + 1;
                        break;
                    }
                    _ => (),
                }
                offset += 1;
            }
        }
        _ => (),
    }
}

fn warehouse_1_move_down(warehouse: &mut Vec<Vec<char>>, pos: &mut (usize, usize)) {
    match warehouse[pos.0 + 1][pos.1] {
        '.' => {
            warehouse[pos.0][pos.1] = '.';
            warehouse[pos.0 + 1][pos.1] = '@';
            pos.0 = pos.0 + 1;
        }
        'O' => {
            let mut offset = 2;
            loop {
                match warehouse[pos.0 + offset][pos.1] {
                    '#' => break,
                    '.' => {
                        warehouse[pos.0][pos.1] = '.';
                        warehouse[pos.0 + 1][pos.1] = '@';
                        for box_offset in 2..=offset {
                            warehouse[pos.0 + box_offset][pos.1] = 'O';
                        }
                        pos.0 = pos.0 + 1;
                        break;
                    }
                    _ => (),
                }
                offset += 1;
            }
        }
        _ => (),
    }
}

fn warehouse_1_move_left(warehouse: &mut Vec<Vec<char>>, pos: &mut (usize, usize)) {
    match warehouse[pos.0][pos.1 - 1] {
        '.' => {
            warehouse[pos.0][pos.1] = '.';
            warehouse[pos.0][pos.1 - 1] = '@';
            pos.1 = pos.1 - 1;
        }
        'O' => {
            let mut offset = 2;
            loop {
                match warehouse[pos.0][pos.1 - offset] {
                    '#' => break,
                    '.' => {
                        warehouse[pos.0][pos.1] = '.';
                        warehouse[pos.0][pos.1 - 1] = '@';
                        for box_offset in 2..=offset {
                            warehouse[pos.0][pos.1 - box_offset] = 'O';
                        }
                        pos.1 = pos.1 - 1;
                        break;
                    }
                    _ => (),
                }
                offset += 1;
            }
        }
        _ => (),
    }
}

fn construct_warehouse_2(warehouse: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut warehouse_2 = Vec::with_capacity(warehouse.len());
    for row in 0..warehouse.len() {
        let mut current_row = Vec::with_capacity(warehouse[row].len() * 2);
        for col in 0..warehouse[row].len() {
            match warehouse[row][col] {
                '@' => current_row.append(&mut vec!['@', '.']),
                '#' => current_row.append(&mut vec!['#', '#']),
                '.' => current_row.append(&mut vec!['.', '.']),
                'O' => current_row.append(&mut vec!['[', ']']),
                _ => (),
            }
        }
        warehouse_2.push(current_row);
    }

    warehouse_2
}

fn warehouse_2_sum_box_gps_coords(warehouse: &mut Vec<Vec<char>>, moves: &str) -> usize {
    let mut robot_pos = (0, 0);
    let mut found_robot = false;
    for row in 0..warehouse.len() {
        for col in 0..warehouse[row].len() {
            if warehouse[row][col] == '@' {
                robot_pos = (row, col);
                found_robot = true;
                break;
            }
        }
        if found_robot {
            break;
        }
    }

    for c in moves.chars() {
        match c {
            '^' => warehouse_2_move_up(warehouse, &mut robot_pos),
            '>' => warehouse_2_move_right(warehouse, &mut robot_pos),
            'v' => warehouse_2_move_down(warehouse, &mut robot_pos),
            '<' => warehouse_2_move_left(warehouse, &mut robot_pos),
            _ => (),
        }
    }
    let mut gps_sum = 0;

    for row in 0..warehouse.len() {
        for col in 0..warehouse[row].len() {
            if warehouse[row][col] == '[' {
                gps_sum += 100 * row + col;
            }
        }
    }

    gps_sum
}

fn warehouse_2_move_up(warehouse: &mut Vec<Vec<char>>, pos: &mut (usize, usize)) {
    match warehouse[pos.0 - 1][pos.1] {
        '.' => {
            warehouse[pos.0][pos.1] = '.';
            warehouse[pos.0 - 1][pos.1] = '@';
            pos.0 = pos.0 - 1;
        }
        '[' | ']' => {
            let mut col_offset_range = if warehouse[pos.0 - 1][pos.1] == '[' {
                (pos.1, pos.1 + 1)
            } else {
                (pos.1 - 1, pos.1)
            };
            let mut col_range_history = vec![col_offset_range];
            let mut row_offset = 2;
            loop {
                let next_area =
                    &warehouse[pos.0 - row_offset][col_offset_range.0..=col_offset_range.1];
                // Break if obstacle is found.
                if next_area.contains(&'#') {
                    break;
                }
                if next_area.iter().all(|&c| c == '.') {
                    for box_offset in (1..row_offset).rev() {
                        let col_range = col_range_history.pop().unwrap();
                        for col_idx in col_range.0..=col_range.1 {
                            warehouse[pos.0 - box_offset - 1][col_idx] =
                                warehouse[pos.0 - box_offset][col_idx];
                            warehouse[pos.0 - box_offset][col_idx] = '.';
                        }
                    }
                    warehouse[pos.0][pos.1] = '.';
                    warehouse[pos.0 - 1][pos.1] = '@';

                    pos.0 = pos.0 - 1;
                    break;
                }
                // Update col_offset_range.
                let mut box_idxs = Vec::new();
                let mut area_idx = 0;
                for idx in col_offset_range.0..=col_offset_range.1 {
                    match next_area[area_idx] {
                        '[' | ']' => box_idxs.push(idx),
                        _ => (),
                    }
                    area_idx += 1;
                }
                col_offset_range.0 = *box_idxs.iter().min().unwrap();
                col_offset_range.1 = *box_idxs.iter().max().unwrap();

                if next_area[0] == ']' {
                    col_offset_range.0 = col_offset_range.0 - 1;
                }
                if next_area[next_area.len() - 1] == '[' {
                    col_offset_range.1 = col_offset_range.1 + 1;
                }
                col_range_history.push(col_offset_range);
                row_offset += 1;
            }
        }
        _ => (),
    }
}

fn warehouse_2_move_right(warehouse: &mut Vec<Vec<char>>, pos: &mut (usize, usize)) {
    match warehouse[pos.0][pos.1 + 1] {
        '.' => {
            warehouse[pos.0][pos.1] = '.';
            warehouse[pos.0][pos.1 + 1] = '@';
            pos.1 = pos.1 + 1;
        }
        '[' => {
            let mut col_offset = 3;
            loop {
                match warehouse[pos.0][pos.1 + col_offset] {
                    '#' => break,
                    '.' => {
                        for offset in (0..col_offset).rev() {
                            warehouse[pos.0][pos.1 + offset + 1] = warehouse[pos.0][pos.1 + offset];
                        }
                        warehouse[pos.0][pos.1] = '.';
                        pos.1 = pos.1 + 1;
                        break;
                    }
                    _ => (),
                }
                col_offset += 2;
            }
        }
        _ => (),
    }
}

fn warehouse_2_move_down(warehouse: &mut Vec<Vec<char>>, pos: &mut (usize, usize)) {
    match warehouse[pos.0 + 1][pos.1] {
        '.' => {
            warehouse[pos.0][pos.1] = '.';
            warehouse[pos.0 + 1][pos.1] = '@';
            pos.0 = pos.0 + 1;
        }
        '[' | ']' => {
            let mut col_offset_range = if warehouse[pos.0 + 1][pos.1] == '[' {
                (pos.1, pos.1 + 1)
            } else {
                (pos.1 - 1, pos.1)
            };
            let mut col_range_history = vec![col_offset_range];
            let mut row_offset = 2;
            loop {
                let next_area =
                    &warehouse[pos.0 + row_offset][col_offset_range.0..=col_offset_range.1];
                // Break if obstacle is found.
                if next_area.contains(&'#') {
                    break;
                }
                if next_area.iter().all(|&c| c == '.') {
                    for box_offset in (1..row_offset).rev() {
                        let col_range = col_range_history.pop().unwrap();
                        for col_idx in col_range.0..=col_range.1 {
                            warehouse[pos.0 + box_offset + 1][col_idx] =
                                warehouse[pos.0 + box_offset][col_idx];
                            warehouse[pos.0 + box_offset][col_idx] = '.';
                        }
                    }
                    warehouse[pos.0][pos.1] = '.';
                    warehouse[pos.0 + 1][pos.1] = '@';

                    pos.0 = pos.0 + 1;
                    break;
                }
                // Update col_offset_range.
                let mut box_idxs = Vec::new();
                let mut area_idx = 0;
                for idx in col_offset_range.0..=col_offset_range.1 {
                    match next_area[area_idx] {
                        '[' | ']' => box_idxs.push(idx),
                        _ => (),
                    }
                    area_idx += 1;
                }
                col_offset_range.0 = *box_idxs.iter().min().unwrap();
                col_offset_range.1 = *box_idxs.iter().max().unwrap();

                if next_area[0] == ']' {
                    col_offset_range.0 = col_offset_range.0 - 1;
                }
                if next_area[next_area.len() - 1] == '[' {
                    col_offset_range.1 = col_offset_range.1 + 1;
                }
                col_range_history.push(col_offset_range);
                row_offset += 1;
            }
        }
        _ => (),
    }
}

fn warehouse_2_move_left(warehouse: &mut Vec<Vec<char>>, pos: &mut (usize, usize)) {
    match warehouse[pos.0][pos.1 - 1] {
        '.' => {
            warehouse[pos.0][pos.1] = '.';
            warehouse[pos.0][pos.1 - 1] = '@';
            pos.1 = pos.1 - 1;
        }
        ']' => {
            let mut col_offset = 3;
            loop {
                match warehouse[pos.0][pos.1 - col_offset] {
                    '#' => break,
                    '.' => {
                        for offset in (0..col_offset).rev() {
                            warehouse[pos.0][pos.1 - offset - 1] = warehouse[pos.0][pos.1 - offset];
                        }
                        warehouse[pos.0][pos.1] = '.';
                        pos.1 = pos.1 - 1;
                        break;
                    }
                    _ => (),
                }
                col_offset += 2;
            }
        }
        _ => (),
    }
}
