use std::{
    cmp,
    collections::{HashMap, HashSet},
    error::Error,
    fs,
};

fn main() -> Result<(), Box<dyn Error>> {
    let map_source = fs::read_to_string("day-8/day8_input.txt")?;
    let map_details = get_map_details(&map_source);

    let antinodes = get_antinodes(&map_details);
    println!("Number of unqiue antinode locations: {}", antinodes.len());

    let antinodes_with_harmoics = get_antinodes_with_harmonics(&map_details);
    println!(
        "Number of unqiue antinode locations including effects of resonant harmonics: {}",
        antinodes_with_harmoics.len()
    );

    Ok(())
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct MapPoint {
    row: i64,
    col: i64,
}

impl MapPoint {
    fn next_antinode_from(&self, other: &Self) -> Self {
        let distance = self.distance(other);

        let antinode_row = match self.row.cmp(&other.row) {
            cmp::Ordering::Less => self.row - distance.row,
            cmp::Ordering::Greater => self.row + distance.row,
            cmp::Ordering::Equal => self.row,
        };
        let antinode_col = match self.col.cmp(&other.col) {
            cmp::Ordering::Less => self.col - distance.col,
            cmp::Ordering::Greater => self.col + distance.col,
            cmp::Ordering::Equal => self.col,
        };

        MapPoint {
            row: antinode_row,
            col: antinode_col,
        }
    }

    fn distance(&self, other: &Self) -> Self {
        MapPoint {
            row: (self.row - other.row).abs(),
            col: (self.col - other.col).abs(),
        }
    }
}

#[derive(Debug, Clone)]
struct MapDetails {
    antennas: HashMap<char, Vec<MapPoint>>,
    max_row: i64,
    max_col: i64,
}
impl MapDetails {
    fn is_valid_point(&self, point: &MapPoint) -> bool {
        if point.row < 0 || point.row > self.max_row || point.col < 0 || point.col > self.max_col {
            false
        } else {
            true
        }
    }
}

fn get_map_details(input: &str) -> MapDetails {
    let mut antennas: HashMap<char, Vec<MapPoint>> = HashMap::new();
    let mut max_col = 0;

    let mut row = 0;
    let mut col = 0;
    for c in input.chars() {
        match c {
            '.' => col += 1,
            '\n' => {
                max_col = col - 1;
                col = 0;
                row += 1;
            }
            _ => {
                antennas
                    .entry(c)
                    .and_modify(|frequency| frequency.push(MapPoint { row, col }))
                    .or_insert(vec![MapPoint { row, col }]);
                col += 1;
            }
        }
    }
    // Subtracts 1 since last char is new line.
    let max_row = row - 1;

    MapDetails {
        antennas,
        max_row,
        max_col,
    }
}

fn get_antinodes(map_details: &MapDetails) -> HashSet<MapPoint> {
    let mut antinodes = HashSet::new();
    for antenna_points in map_details.antennas.values() {
        let mut antenna_pairs = Vec::new();
        create_antenna_pairs(antenna_points, &mut vec![], &mut antenna_pairs, 0);
        for (point_1, point_2) in antenna_pairs {
            let antinode_1 = point_1.next_antinode_from(&point_2);
            if map_details.is_valid_point(&antinode_1) {
                antinodes.insert(antinode_1);
            }

            let antinode_2 = point_2.next_antinode_from(&point_1);
            if map_details.is_valid_point(&antinode_2) {
                antinodes.insert(antinode_2);
            }
        }
    }

    antinodes
}

fn get_antinodes_with_harmonics(map_details: &MapDetails) -> HashSet<MapPoint> {
    let mut antinodes = HashSet::new();
    for antenna_points in map_details.antennas.values() {
        let mut antenna_pairs = Vec::new();
        create_antenna_pairs(antenna_points, &mut vec![], &mut antenna_pairs, 0);
        for (point_1, point_2) in antenna_pairs {
            // Calculates antinodes down line from point_2 -> point_1
            antinodes.insert(point_1.clone());
            let mut start_point = point_1.clone();
            let mut reference_point = point_2.clone();
            loop {
                let antinode = start_point.next_antinode_from(&reference_point);
                match map_details.is_valid_point(&antinode) {
                    true => {
                        antinodes.insert(antinode.clone());
                        reference_point = start_point;
                        start_point = antinode;
                    }
                    false => {
                        break;
                    }
                }
            }

            // Calculates antinodes down line from point_1 -> point_2
            antinodes.insert(point_2.clone());
            start_point = point_2.clone();
            reference_point = point_1.clone();
            loop {
                let antinode = start_point.next_antinode_from(&reference_point);
                match map_details.is_valid_point(&antinode) {
                    true => {
                        antinodes.insert(antinode.clone());
                        reference_point = start_point;
                        start_point = antinode;
                    }
                    false => {
                        break;
                    }
                }
            }
        }
    }

    antinodes
}

fn create_antenna_pairs<'antenna_points, 'antenna_pairs, 'current_pair>(
    antenna_points: &'antenna_points Vec<MapPoint>,
    current_pair: &'current_pair mut Vec<&'antenna_points MapPoint>,
    antenna_pairs: &'antenna_pairs mut Vec<(&'antenna_points MapPoint, &'antenna_points MapPoint)>,
    start_idx: usize,
) {
    if current_pair.len() == 2 {
        let antenna_pair = (current_pair[0], current_pair[1]);
        antenna_pairs.push(antenna_pair);
        return;
    }

    for idx in start_idx..antenna_points.len() {
        current_pair.push(&antenna_points[idx]);
        create_antenna_pairs(antenna_points, current_pair, antenna_pairs, idx + 1);
        current_pair.pop();
    }
}
