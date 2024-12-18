use std::{collections::VecDeque, error::Error, fs};

fn main() -> Result<(), Box<dyn Error>> {
    let byte_locations: Vec<(usize, usize)> = fs::read_to_string("day-18/day18_input.txt")?
        .split('\n')
        .filter(|row| row.len() > 0)
        .map(|row| {
            let mut positions = row.trim().split(',');
            let x = positions
                .next()
                .expect("Failed to find X coord")
                .parse::<usize>()
                .expect("Failed to parse X coord");
            let y = positions
                .next()
                .expect("Failed to find Y coord")
                .parse::<usize>()
                .expect("Failed to parse Y coord");
            (x, y)
        })
        .collect();

    let grid_size = 71;
    let num_simulated = 1024;
    let min_path_len = find_exit(&byte_locations[..num_simulated], grid_size);
    println!("Minimum number of steps to exit after {num_simulated} bytes fall: {min_path_len}");

    for test_limit in num_simulated..=byte_locations.len() {
        if find_exit(&byte_locations[..test_limit], grid_size) == 0 {
            let blocking_byte = byte_locations[test_limit - 1];
            println!(
                "First byte that blocks exit falls at {},{}",
                blocking_byte.0, blocking_byte.1
            );
            break;
        }
    }

    Ok(())
}

fn find_exit(obstacles: &[(usize, usize)], grid_size: usize) -> usize {
    let mut grid = vec![vec!['.'; grid_size]; grid_size];
    for (row, col) in obstacles {
        grid[*row][*col] = '#';
    }
    grid[0][0] = 'O';
    // VecDeque items are (row, column, current_path_len)
    let mut queue: VecDeque<(usize, usize, usize)> = VecDeque::new();
    queue.push_back((0, 0, 0));
    while let Some(current_pos) = queue.pop_front() {
        let row = current_pos.0;
        let col = current_pos.1;
        let path_len = current_pos.2;

        if row == grid_size - 1 && col == grid_size - 1 {
            return path_len;
        }

        // Check Up.
        if row > 0 && grid[row - 1][col] == '.' {
            grid[row - 1][col] = 'O';
            queue.push_back((row - 1, col, path_len + 1));
        }

        // Check Right.
        if col + 1 < grid_size && grid[row][col + 1] == '.' {
            grid[row][col + 1] = 'O';
            queue.push_back((row, col + 1, path_len + 1));
        }

        // Check Down.
        if row + 1 < grid_size && grid[row + 1][col] == '.' {
            grid[row + 1][col] = 'O';
            queue.push_back((row + 1, col, path_len + 1));
        }

        // Check Left.
        if col > 0 && grid[row][col - 1] == '.' {
            grid[row][col - 1] = 'O';
            queue.push_back((row, col - 1, path_len + 1));
        }
    }
    0
}
