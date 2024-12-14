use std::{error::Error, fs};

#[derive(Debug, Clone)]
struct Robot {
    pos: (i64, i64),
    velocity: (i64, i64),
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut robots: Vec<Robot> = fs::read_to_string("day-14/day14_input.txt")?
        .trim()
        .split('\n')
        .map(|row| {
            let mut robot_details = row.split(' ');
            let pos_vals: Vec<&str> = robot_details.next().unwrap()[2..].split(',').collect();
            let velocity_vals: Vec<&str> = robot_details.next().unwrap()[2..].split(',').collect();
            Robot {
                pos: (
                    pos_vals[0].trim().parse().unwrap(),
                    pos_vals[1].trim().parse().unwrap(),
                ),
                velocity: (
                    velocity_vals[0].trim().parse().unwrap(),
                    velocity_vals[1].trim().parse().unwrap(),
                ),
            }
        })
        .collect();

    let num_rows = 103;
    let num_cols = 101;

    for iter_num in 1..=10000 {
        move_robots(&mut robots, num_rows, num_cols);
        calculate_safety_factor(&robots, num_rows, num_cols, iter_num);
        if iter_num == 100 {
            println!(
                "Safety factor after 100 seconds: {}",
                calculate_safety_factor(&robots, num_rows, num_cols, iter_num)
            );
        }
    }

    Ok(())
}

fn move_robots(robots: &mut Vec<Robot>, num_rows: i64, num_cols: i64) {
    for robot in robots.iter_mut() {
        let mut x = robot.pos.0 + robot.velocity.0;
        if x >= 0 {
            x = x % num_cols;
        } else {
            x = num_cols + x;
        }

        let mut y = robot.pos.1 + robot.velocity.1;
        if y >= 0 {
            y = y % num_rows;
        } else {
            y = num_rows + y;
        }

        robot.pos = (x, y);
    }
}

fn calculate_safety_factor(
    robots: &Vec<Robot>,
    num_rows: i64,
    num_cols: i64,
    iter_num: i32,
) -> usize {
    let mut num_in_q1 = 0;
    let mut num_in_q2 = 0;
    let mut num_in_q3 = 0;
    let mut num_in_q4 = 0;

    for robot in robots {
        let x = robot.pos.0;
        let y = robot.pos.1;
        if x < num_cols / 2 && y < num_rows / 2 {
            num_in_q1 += 1;
        } else if x > num_cols / 2 && y < num_rows / 2 {
            num_in_q2 += 1;
        } else if x < num_cols / 2 && y > num_rows / 2 {
            num_in_q3 += 1;
        } else if x > num_cols / 2 && y > num_rows / 2 {
            num_in_q4 += 1;
        }
    }
    if num_in_q1 >= robots.len() / 2
        || num_in_q2 >= robots.len() / 2
        || num_in_q3 >= robots.len() / 2
        || num_in_q4 >= robots.len() / 2
    {
        println!("Christmas tree easter egg found after {iter_num} seconds.");
        print_map(robots, num_rows as usize, num_cols as usize);
    }

    num_in_q1 * num_in_q2 * num_in_q3 * num_in_q4
}

fn print_map(robots: &Vec<Robot>, num_rows: usize, num_cols: usize) {
    let mut map = vec![vec!['.'; num_cols]; num_rows];
    for robot in robots {
        map[robot.pos.1 as usize][robot.pos.0 as usize] = '#';
    }
    for row in map {
        println!(
            "{}",
            row.iter().map(|num| num.to_string()).collect::<String>(),
        );
    }
}
