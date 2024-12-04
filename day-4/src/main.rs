use std::{error::Error, fs};

fn main() -> Result<(), Box<dyn Error>> {
    let word_search_input = fs::read_to_string("day-4/day4_input.txt")?;
    let word_search_matrix = generate_matrix(&word_search_input);

    let mut xmas_count = 0;
    let mut cross_mas_count = 0;

    for row in 0..word_search_matrix.len() {
        for col in 0..word_search_matrix[row].len() {
            match word_search_matrix[row][col] {
                'X' => xmas_count += xmas_check(&word_search_matrix, row, col),
                'M' | 'S' => cross_mas_count += cross_mas_check(&word_search_matrix, row, col),
                _ => (),
            }
        }
    }

    println!("Number of XMAS's: {xmas_count}");
    println!("Number of X-MAS's: {cross_mas_count}");

    Ok(())
}

fn generate_matrix(word_search: &str) -> Vec<Vec<char>> {
    let mut matrix = Vec::new();
    let mut current_row = Vec::new();
    for c in word_search.chars() {
        if c == '\n' {
            matrix.push(current_row);
            current_row = Vec::new();
        } else {
            current_row.push(c);
        }
    }

    matrix
}

fn xmas_check(matrix: &Vec<Vec<char>>, row: usize, col: usize) -> i32 {
    let mut xmas_ctr = 0;

    // Check Up.
    if row >= 3 {
        match (
            matrix[row][col],
            matrix[row - 1][col],
            matrix[row - 2][col],
            matrix[row - 3][col],
        ) {
            ('X', 'M', 'A', 'S') => xmas_ctr += 1,
            _ => (),
        }
    }

    // Check Up Right
    if row >= 3 && col + 3 < matrix[row].len() {
        match (
            matrix[row][col],
            matrix[row - 1][col + 1],
            matrix[row - 2][col + 2],
            matrix[row - 3][col + 3],
        ) {
            ('X', 'M', 'A', 'S') => xmas_ctr += 1,
            _ => (),
        }
    }

    // Check Right
    if col + 3 < matrix[row].len() {
        match (
            matrix[row][col],
            matrix[row][col + 1],
            matrix[row][col + 2],
            matrix[row][col + 3],
        ) {
            ('X', 'M', 'A', 'S') => xmas_ctr += 1,
            _ => (),
        }
    }

    // Check Down Right
    if row + 3 < matrix.len() && col + 3 < matrix[row].len() {
        match (
            matrix[row][col],
            matrix[row + 1][col + 1],
            matrix[row + 2][col + 2],
            matrix[row + 3][col + 3],
        ) {
            ('X', 'M', 'A', 'S') => xmas_ctr += 1,
            _ => (),
        }
    }

    // Check Down
    if row + 3 < matrix.len() {
        match (
            matrix[row][col],
            matrix[row + 1][col],
            matrix[row + 2][col],
            matrix[row + 3][col],
        ) {
            ('X', 'M', 'A', 'S') => xmas_ctr += 1,
            _ => (),
        }
    }

    // Check Down Left
    if row + 3 < matrix.len() && col >= 3 {
        match (
            matrix[row][col],
            matrix[row + 1][col - 1],
            matrix[row + 2][col - 2],
            matrix[row + 3][col - 3],
        ) {
            ('X', 'M', 'A', 'S') => xmas_ctr += 1,
            _ => (),
        }
    }

    // Check Left
    if col >= 3 {
        match (
            matrix[row][col],
            matrix[row][col - 1],
            matrix[row][col - 2],
            matrix[row][col - 3],
        ) {
            ('X', 'M', 'A', 'S') => xmas_ctr += 1,
            _ => (),
        }
    }

    // Check Up Left
    if row >= 3 && col >= 3 {
        match (
            matrix[row][col],
            matrix[row - 1][col - 1],
            matrix[row - 2][col - 2],
            matrix[row - 3][col - 3],
        ) {
            ('X', 'M', 'A', 'S') => xmas_ctr += 1,
            _ => (),
        }
    }

    xmas_ctr
}

fn cross_mas_check(matrix: &Vec<Vec<char>>, row: usize, col: usize) -> i32 {
    if row + 2 >= matrix.len() || col + 2 >= matrix[row].len() || matrix[row + 1][col + 1] != 'A' {
        return 0;
    }

    // Check line from Top Left to Bottom Right of cross.
    match (
        matrix[row][col],
        matrix[row + 1][col + 1],
        matrix[row + 2][col + 2],
    ) {
        ('M', 'A', 'S') | ('S', 'A', 'M') => (),
        _ => {
            return 0;
        }
    }

    // Check line from Bottom Left to Top Right of cross.
    match (
        matrix[row + 2][col],
        matrix[row + 1][col + 1],
        matrix[row][col + 2],
    ) {
        ('M', 'A', 'S') | ('S', 'A', 'M') => (),
        _ => {
            return 0;
        }
    }

    1
}
