use std::{collections::HashSet, error::Error, fs};

#[derive(Copy, PartialEq, Debug, Clone, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Copy, PartialEq, Debug, Clone)]
struct MapPoint {
    row: usize,
    col: usize,
}

fn main() -> Result<(), Box<dyn Error>> {
    let map_source_stream = fs::read_to_string("day-6/day6_input.txt")?;

    let (mut map, guard_start) = generate_map(&map_source_stream);

    let num_guard_positions = track_guard_positions(&mut map, guard_start);

    println!("Number of distinct guard positions: {num_guard_positions}");

    let mut num_loop_options = 0;
    for row in 0..map.len() {
        for col in 0..map[row].len() {
            if row == guard_start.row && col == guard_start.col {
                continue;
            }
            if map[row][col] == 'X'
                && guard_gets_stuck(&mut map, guard_start, MapPoint { row, col })
            {
                num_loop_options += 1;
            }
        }
    }

    println!("Number of different positions to put obstructions: {num_loop_options}");

    Ok(())
}

fn generate_map(source: &str) -> (Vec<Vec<char>>, MapPoint) {
    let mut map = Vec::new();

    let mut row_idx = 0;
    let mut col_idx = 0;

    let mut guard_start = MapPoint { row: 0, col: 0 };

    let mut current_row = Vec::new();
    for c in source.chars() {
        match c {
            '.' | '#' | '^' => {
                if c == '^' {
                    guard_start.row = row_idx;
                    guard_start.col = col_idx;
                }
                current_row.push(c);
                col_idx += 1;
            }
            '\n' => {
                map.push(current_row);
                current_row = Vec::new();
                row_idx += 1;
                col_idx = 0;
            }
            _ => unreachable!("Only expect '.', '#', '^', or new line char."),
        }
    }

    (map, guard_start)
}

fn track_guard_positions(map: &mut Vec<Vec<char>>, mut guard_pos: MapPoint) -> i32 {
    map[guard_pos.row][guard_pos.col] = 'X';
    let mut num_distinct_positions = 1;

    let mut current_direction = Direction::Up;

    // Creates some maximum iteration value to panic in case of an infinite loop.
    let mut iter_ctr = 0;
    let max_iterations = map.len() * map[0].len() * 10;
    while is_next_move_valid(&map, guard_pos, current_direction) {
        assert!(iter_ctr < max_iterations);
        let (next_row, next_col) = match current_direction {
            Direction::Up => (guard_pos.row - 1, guard_pos.col),
            Direction::Down => (guard_pos.row + 1, guard_pos.col),
            Direction::Left => (guard_pos.row, guard_pos.col - 1),
            Direction::Right => (guard_pos.row, guard_pos.col + 1),
        };

        match map[next_row][next_col] {
            '.' => {
                map[next_row][next_col] = 'X';
                num_distinct_positions += 1;
            }
            '#' => {
                current_direction = rotate_right(current_direction);
                continue;
            }
            _ => (),
        }

        guard_pos.row = next_row;
        guard_pos.col = next_col;
        iter_ctr += 1;
    }

    num_distinct_positions
}

fn guard_gets_stuck(
    map: &mut Vec<Vec<char>>,
    mut guard_pos: MapPoint,
    new_obstruction: MapPoint,
) -> bool {
    // Create new obstruction.
    map[new_obstruction.row][new_obstruction.col] = '#';

    let mut current_direction = Direction::Up;
    let mut found_loop = false;
    let mut loop_tracker = HashSet::new();

    // Creates some maximum iteration value to panic in case of an infinite loop.
    let mut iter_ctr = 0;
    let max_iterations = map.len() * map[0].len() * 100;
    while is_next_move_valid(&map, guard_pos, current_direction) {
        assert!(iter_ctr < max_iterations);
        let (next_row, next_col) = match current_direction {
            Direction::Up => (guard_pos.row - 1, guard_pos.col),
            Direction::Down => (guard_pos.row + 1, guard_pos.col),
            Direction::Left => (guard_pos.row, guard_pos.col - 1),
            Direction::Right => (guard_pos.row, guard_pos.col + 1),
        };

        match map[next_row][next_col] {
            '#' => {
                current_direction = rotate_right(current_direction);
                let current_position = (next_row, next_col, current_direction);
                if !loop_tracker.insert(current_position) {
                    found_loop = true;
                    break;
                }
                continue;
            }
            _ => (),
        }

        guard_pos.row = next_row;
        guard_pos.col = next_col;
        iter_ctr += 1;
    }

    // Remove new obstruction.
    map[new_obstruction.row][new_obstruction.col] = 'X';

    found_loop
}

fn is_next_move_valid(map: &Vec<Vec<char>>, point: MapPoint, direction: Direction) -> bool {
    match direction {
        Direction::Up if point.row == 0 => false,
        Direction::Down if point.row + 1 == map.len() => false,
        Direction::Left if point.col == 0 => false,
        Direction::Right if point.col + 1 == map[0].len() => false,
        _ => true,
    }
}

fn rotate_right(direction: Direction) -> Direction {
    match direction {
        Direction::Up => Direction::Right,
        Direction::Right => Direction::Down,
        Direction::Down => Direction::Left,
        Direction::Left => Direction::Up,
    }
}
