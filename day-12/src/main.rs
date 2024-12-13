use std::{collections::HashSet, error::Error, fs};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct MapPoint {
    row: usize,
    col: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Debug, Clone)]
struct Region {
    area: usize,
    perimeter: usize,
    fence_sides: usize,
}
impl Region {
    fn new() -> Self {
        Region {
            area: 0,
            perimeter: 0,
            fence_sides: 0,
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let garden_plots: Vec<Vec<char>> = fs::read_to_string("day-12/day12_input.txt")?
        .trim()
        .split('\n')
        .map(|r| r.chars().collect())
        .collect();

    let (price_of_fencing, bulk_discount_price) = calculate_fencing_price(&garden_plots);
    println!("Total price of fencing all regions: {price_of_fencing}");
    println!("Total price of fencing all regions with bulk discount: {bulk_discount_price}");

    Ok(())
}

fn calculate_fencing_price(garden_plots: &Vec<Vec<char>>) -> (usize, usize) {
    let mut visited_set = HashSet::new();
    let mut price = 0;
    let mut discount_price = 0;

    for row in 0..garden_plots.len() {
        for col in 0..garden_plots[row].len() {
            let loc = MapPoint { row, col };
            if !visited_set.contains(&loc) {
                let mut region = Region::new();
                traverse_region(&mut region, loc, garden_plots, &mut visited_set);
                price += region.area * region.perimeter;
                discount_price += region.area * region.fence_sides;
            }
        }
    }

    (price, discount_price)
}

fn traverse_region(
    region: &mut Region,
    loc: MapPoint,
    garden_plots: &Vec<Vec<char>>,
    visited_set: &mut HashSet<MapPoint>,
) {
    visited_set.insert(loc);
    region.area += 1;

    // Check Up.
    if match_plot_type(&loc, Direction::Up, garden_plots) {
        let up = MapPoint {
            row: loc.row - 1,
            col: loc.col,
        };
        if !visited_set.contains(&up) {
            traverse_region(region, up, garden_plots, visited_set);
        }
    } else {
        region.perimeter += 1;
        if check_new_fence_side(&loc, Direction::Up, garden_plots) {
            region.fence_sides += 1;
        }
    }

    // Check Right.
    if match_plot_type(&loc, Direction::Right, garden_plots) {
        let right = MapPoint {
            row: loc.row,
            col: loc.col + 1,
        };
        if !visited_set.contains(&right) {
            traverse_region(region, right, garden_plots, visited_set);
        }
    } else {
        region.perimeter += 1;
        if check_new_fence_side(&loc, Direction::Right, garden_plots) {
            region.fence_sides += 1;
        }
    }

    // Check Down.
    if match_plot_type(&loc, Direction::Down, garden_plots) {
        let down = MapPoint {
            row: loc.row + 1,
            col: loc.col,
        };
        if !visited_set.contains(&down) {
            traverse_region(region, down, garden_plots, visited_set);
        }
    } else {
        region.perimeter += 1;
        if check_new_fence_side(&loc, Direction::Down, garden_plots) {
            region.fence_sides += 1;
        }
    }

    // Check Left.
    if match_plot_type(&loc, Direction::Left, garden_plots) {
        let left = MapPoint {
            row: loc.row,
            col: loc.col - 1,
        };
        if !visited_set.contains(&left) {
            traverse_region(region, left, garden_plots, visited_set);
        }
    } else {
        region.perimeter += 1;
        if check_new_fence_side(&loc, Direction::Left, garden_plots) {
            region.fence_sides += 1;
        }
    }
}

fn match_plot_type(loc: &MapPoint, direction: Direction, garden_plots: &Vec<Vec<char>>) -> bool {
    match direction {
        Direction::Up => {
            if loc.row == 0 {
                return false;
            }
            garden_plots[loc.row][loc.col] == garden_plots[loc.row - 1][loc.col]
        }
        Direction::Right => {
            if loc.col == garden_plots[0].len() - 1 {
                return false;
            }
            garden_plots[loc.row][loc.col] == garden_plots[loc.row][loc.col + 1]
        }
        Direction::Down => {
            if loc.row == garden_plots.len() - 1 {
                return false;
            }
            garden_plots[loc.row][loc.col] == garden_plots[loc.row + 1][loc.col]
        }
        Direction::Left => {
            if loc.col == 0 {
                return false;
            }
            garden_plots[loc.row][loc.col] == garden_plots[loc.row][loc.col - 1]
        }
    }
}

fn check_new_fence_side(
    loc: &MapPoint,
    fence_direction: Direction,
    garden_plots: &Vec<Vec<char>>,
) -> bool {
    match fence_direction {
        Direction::Up => {
            let next_right_point = MapPoint {
                row: loc.row,
                col: loc.col + 1,
            };
            if match_plot_type(loc, Direction::Right, garden_plots)
                && !match_plot_type(&next_right_point, Direction::Up, garden_plots)
            {
                false
            } else {
                true
            }
        }
        Direction::Right => {
            let next_down_point = MapPoint {
                row: loc.row + 1,
                col: loc.col,
            };
            if match_plot_type(loc, Direction::Down, garden_plots)
                && !match_plot_type(&next_down_point, Direction::Right, garden_plots)
            {
                false
            } else {
                true
            }
        }
        Direction::Down => {
            let next_right_point = MapPoint {
                row: loc.row,
                col: loc.col + 1,
            };
            if match_plot_type(loc, Direction::Right, garden_plots)
                && !match_plot_type(&next_right_point, Direction::Down, garden_plots)
            {
                false
            } else {
                true
            }
        }

        Direction::Left => {
            let next_down_point = MapPoint {
                row: loc.row + 1,
                col: loc.col,
            };
            if match_plot_type(loc, Direction::Down, garden_plots)
                && !match_plot_type(&next_down_point, Direction::Left, garden_plots)
            {
                false
            } else {
                true
            }
        }
    }
}
