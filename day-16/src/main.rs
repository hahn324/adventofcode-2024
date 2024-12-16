use std::{
    cmp,
    collections::{BinaryHeap, HashSet},
    error::Error,
    fs, usize,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct MazePoint {
    row: usize,
    col: usize,
    direction: Direction,
    score: usize,
    path: Vec<(usize, usize)>,
}
impl PartialOrd for MazePoint {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(other.score.cmp(&self.score))
    }
}
impl Ord for MazePoint {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        other.score.cmp(&self.score)
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let maze: Vec<Vec<char>> = fs::read_to_string("day-16/day16_input.txt")?
        .trim()
        .split('\n')
        .map(|row| row.chars().collect())
        .collect();

    let (lowest_score, num_seat_options) = solve_maze(&maze);

    println!("Lowest score to solve maze: {}", lowest_score);
    println!(
        "Number of seating options on a best path: {}",
        num_seat_options
    );

    Ok(())
}

fn solve_maze(maze: &Vec<Vec<char>>) -> (usize, usize) {
    let mut visited_set = HashSet::new();
    let mut visit_queue = BinaryHeap::new();
    // Find position of S
    for row in 0..maze.len() {
        for col in 0..maze[row].len() {
            if maze[row][col] == 'S' {
                visit_queue.push(MazePoint {
                    row,
                    col,
                    direction: Direction::East,
                    score: 0,
                    path: Vec::new(),
                });
                break;
            }
        }
        if visit_queue.len() != 0 {
            break;
        }
    }
    let mut lowest_score = usize::MAX;
    let mut seat_options = HashSet::new();
    while let Some(mut point) = visit_queue.pop() {
        point.path.push((point.row, point.col));
        visited_set.insert((point.row, point.col, point.direction));
        if maze[point.row][point.col] == 'E' {
            lowest_score = cmp::min(point.score, lowest_score);
            if point.score == lowest_score {
                for seat in point.path {
                    seat_options.insert(seat);
                }
            }
            continue;
        }
        // Check North
        if maze[point.row - 1][point.col] != '#'
            && !visited_set.contains(&(point.row - 1, point.col, Direction::North))
        {
            let score = point.score + 1 + cost_to_turn(point.direction, Direction::North);
            let row = point.row - 1;
            let col = point.col;
            let path = point.path.clone();
            visit_queue.push(MazePoint {
                row,
                col,
                direction: Direction::North,
                score,
                path,
            });
        }
        // Check East
        if maze[point.row][point.col + 1] != '#'
            && !visited_set.contains(&(point.row, point.col + 1, Direction::East))
        {
            let score = point.score + 1 + cost_to_turn(point.direction, Direction::East);
            let row = point.row;
            let col = point.col + 1;
            let path = point.path.clone();
            visit_queue.push(MazePoint {
                row,
                col,
                direction: Direction::East,
                score,
                path,
            });
        }
        // Check South
        if maze[point.row + 1][point.col] != '#'
            && !visited_set.contains(&(point.row + 1, point.col, Direction::South))
        {
            let score = point.score + 1 + cost_to_turn(point.direction, Direction::South);
            let row = point.row + 1;
            let col = point.col;
            let path = point.path.clone();
            visit_queue.push(MazePoint {
                row,
                col,
                direction: Direction::South,
                score,
                path,
            });
        }
        // Check West
        if maze[point.row][point.col - 1] != '#'
            && !visited_set.contains(&(point.row, point.col - 1, Direction::West))
        {
            let score = point.score + 1 + cost_to_turn(point.direction, Direction::West);
            let row = point.row;
            let col = point.col - 1;
            let path = point.path.clone();
            visit_queue.push(MazePoint {
                row,
                col,
                direction: Direction::West,
                score,
                path,
            });
        }
    }
    (lowest_score, seat_options.len())
}

fn cost_to_turn(current: Direction, target: Direction) -> usize {
    match current {
        Direction::North => match target {
            Direction::North => 0,
            Direction::East | Direction::West => 1000,
            Direction::South => 2000,
        },
        Direction::East => match target {
            Direction::North | Direction::South => 1000,
            Direction::East => 0,
            Direction::West => 2000,
        },
        Direction::South => match target {
            Direction::North => 2000,
            Direction::East | Direction::West => 1000,
            Direction::South => 0,
        },
        Direction::West => match target {
            Direction::North | Direction::South => 1000,
            Direction::West => 0,
            Direction::East => 2000,
        },
    }
}
