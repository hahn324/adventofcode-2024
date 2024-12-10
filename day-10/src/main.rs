use std::{error::Error, fs};

#[derive(Debug, Clone, PartialEq)]
struct MapPoint {
    row: usize,
    col: usize,
}

#[derive(Debug, Clone)]
struct Trailhead {
    loc: MapPoint,
    peaks: Vec<MapPoint>,
    rating: usize,
}

#[derive(Debug, Clone)]
struct TrailMap {
    map: Vec<Vec<i8>>,
    trailheads: Vec<Trailhead>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let map_source = fs::read_to_string("day-10/day10_input.txt")?;
    let mut trail_map = parse_topographic_map(&map_source);
    calculate_trailhead_scores(&mut trail_map);

    let mut trailhead_scores_sum = 0;
    let mut trailhead_ratings_sum = 0;
    for trailhead in trail_map.trailheads.iter() {
        trailhead_scores_sum += trailhead.peaks.len();
        trailhead_ratings_sum += trailhead.rating;
    }

    println!("Sum of scores of all trailheads: {trailhead_scores_sum}");
    println!("Sum of ratings of all trailheads: {trailhead_ratings_sum}");

    Ok(())
}

fn parse_topographic_map(topographic_map: &str) -> TrailMap {
    let mut trail_map = TrailMap {
        map: Vec::new(),
        trailheads: Vec::new(),
    };

    let mut row_idx = 0;
    let mut col_idx = 0;

    let mut current_row = Vec::new();
    for c in topographic_map.chars() {
        match c {
            '\n' => {
                row_idx += 1;
                col_idx = 0;
                trail_map.map.push(current_row);
                current_row = Vec::new();
            }
            '0' => {
                current_row.push(0);
                let loc = MapPoint {
                    row: row_idx,
                    col: col_idx,
                };
                trail_map.trailheads.push(Trailhead {
                    loc,
                    peaks: Vec::new(),
                    rating: 0,
                });
                col_idx += 1;
            }
            _ => {
                current_row.push(c.to_digit(10).unwrap() as i8);
                col_idx += 1;
            }
        }
    }

    trail_map
}

fn calculate_trailhead_scores(trail_map: &mut TrailMap) {
    for trailhead in trail_map.trailheads.iter_mut() {
        trailhead.rating =
            find_trailhead_peaks(&trailhead.loc, &trail_map.map, &mut trailhead.peaks);
    }
}

fn find_trailhead_peaks(loc: &MapPoint, map: &Vec<Vec<i8>>, peaks: &mut Vec<MapPoint>) -> usize {
    let cur_height = map[loc.row][loc.col];

    if map[loc.row][loc.col] == 9 {
        if !peaks.contains(loc) {
            peaks.push(loc.clone());
        }
        return 1;
    }
    // Search Up
    let mut score_up = 0;
    if loc.row > 0 && map[loc.row - 1][loc.col] - cur_height == 1 {
        score_up = find_trailhead_peaks(
            &MapPoint {
                row: loc.row - 1,
                col: loc.col,
            },
            map,
            peaks,
        );
    }
    // Search Right
    let mut score_right = 0;
    if loc.col + 1 < map[0].len() && map[loc.row][loc.col + 1] - cur_height == 1 {
        score_right = find_trailhead_peaks(
            &MapPoint {
                row: loc.row,
                col: loc.col + 1,
            },
            map,
            peaks,
        );
    }
    // Search Down
    let mut score_down = 0;
    if loc.row + 1 < map.len() && map[loc.row + 1][loc.col] - cur_height == 1 {
        score_down = find_trailhead_peaks(
            &MapPoint {
                row: loc.row + 1,
                col: loc.col,
            },
            map,
            peaks,
        );
    }
    // Search Left
    let mut score_left = 0;
    if loc.col > 0 && map[loc.row][loc.col - 1] - cur_height == 1 {
        score_left = find_trailhead_peaks(
            &MapPoint {
                row: loc.row,
                col: loc.col - 1,
            },
            map,
            peaks,
        );
    }

    score_up + score_right + score_down + score_left
}
