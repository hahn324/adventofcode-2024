use std::{
    collections::{HashMap, HashSet, VecDeque},
    error::Error,
    fs,
};

#[derive(Debug, Clone, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
    Start,
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut racetrack: Vec<Vec<char>> = fs::read_to_string("day-20/day20_input.txt")?
        .trim()
        .split('\n')
        .map(|row| row.trim().chars().collect())
        .collect();

    let starting_row = racetrack
        .iter()
        .position(|row| row.contains(&'S'))
        .expect("Expect to find a row with 'S' in racetrack.");
    let starting_col = racetrack[starting_row]
        .iter()
        .position(|&c| c == 'S')
        .expect("Expect row to contains starting position 'S'.");
    let starting_point = (starting_row, starting_col);

    let time_save_min = 100;
    let (num_rule_1_cheats, num_rule_2_cheats) =
        find_cheats(&mut racetrack, starting_point, time_save_min);
    println!(
        "With first cheat rules, number of cheats that save at least {time_save_min} picoseconds: {num_rule_1_cheats}"
    );
    println!(
        "With second cheat rules, number of cheats that save at least {time_save_min} picoseconds: {num_rule_2_cheats}"
    );

    Ok(())
}

fn find_cheats(
    racetrack: &mut Vec<Vec<char>>,
    start: (usize, usize),
    time_save_min: usize,
) -> (usize, usize) {
    let mut rule_1_num_viable_cheats = 0;
    let mut rule_2_num_viable_cheats = 0;

    let mut possible_cheats = Vec::new();
    let mut runtimes = HashMap::new();

    // Direction from is used to prevent moving backwards through the track.
    let mut from = Direction::Start;
    let mut current = start;
    runtimes.insert(current, 0);

    while racetrack[current.0][current.1] != 'E' {
        add_cheat_options(racetrack, current, &mut possible_cheats);
        advance(racetrack, &mut runtimes, &mut current, &mut from);
    }
    let baseline_runtime = *runtimes
        .get(&(current.0, current.1))
        .expect("Will contain runtime for end position.");

    // Collect number of valid cheats using second cheat rule.
    for cheat in possible_cheats {
        let baseline_before_cheat = runtimes
            .get(&(cheat.0, cheat.1))
            .expect("Will contain cheat start point.");
        let baseline_after_cheat = runtimes
            .get(&(cheat.2, cheat.3))
            .expect("Will contain cheat end point.");
        let runtime = baseline_before_cheat + cheat.4 + (baseline_runtime - baseline_after_cheat);

        if runtime > baseline_runtime {
            continue;
        }

        let time_save = baseline_runtime - runtime;
        if time_save >= time_save_min {
            if cheat.4 == 2 {
                rule_1_num_viable_cheats += 1;
            }
            rule_2_num_viable_cheats += 1;
        }
    }

    (rule_1_num_viable_cheats, rule_2_num_viable_cheats)
}

fn advance(
    racetrack: &Vec<Vec<char>>,
    runtimes: &mut HashMap<(usize, usize), usize>,
    loc: &mut (usize, usize),
    from: &mut Direction,
) {
    let runtime = runtimes.get(&loc).expect("Will always be a previous loc.");
    if *from != Direction::Up && racetrack[loc.0 - 1][loc.1] != '#' {
        loc.0 -= 1;
        *from = Direction::Down;
    } else if *from != Direction::Down && racetrack[loc.0 + 1][loc.1] != '#' {
        loc.0 += 1;
        *from = Direction::Up;
    } else if *from != Direction::Left && racetrack[loc.0][loc.1 - 1] != '#' {
        loc.1 -= 1;
        *from = Direction::Right;
    } else if *from != Direction::Right && racetrack[loc.0][loc.1 + 1] != '#' {
        loc.1 += 1;
        *from = Direction::Left;
    }
    runtimes.insert(*loc, runtime + 1);
}

fn add_cheat_options(
    racetrack: &Vec<Vec<char>>,
    loc: (usize, usize),
    cheats: &mut Vec<(usize, usize, usize, usize, usize)>,
) {
    let mut visit_queue = VecDeque::new();
    let mut visited_set = HashSet::new();
    visited_set.insert(loc);
    visit_queue.push_back((loc.0, loc.1, 0));

    while let Some(cur_loc) = visit_queue.pop_front() {
        let row = cur_loc.0;
        let col = cur_loc.1;
        let runtime = cur_loc.2;

        match racetrack[row][col] {
            '.' | 'S' | 'E' if runtime == 20 => {
                cheats.push((loc.0, loc.1, row, col, runtime));
            }
            _ if runtime < 20 => {
                match racetrack[row][col] {
                    '.' | 'S' | 'E' => cheats.push((loc.0, loc.1, row, col, runtime)),
                    _ => (),
                }
                // Check Up
                if row > 0 && !visited_set.contains(&(row - 1, col)) {
                    visit_queue.push_back((row - 1, col, runtime + 1));
                    visited_set.insert((row - 1, col));
                }
                // Check Down
                if row + 1 < racetrack.len() && !visited_set.contains(&(row + 1, col)) {
                    visit_queue.push_back((row + 1, col, runtime + 1));
                    visited_set.insert((row + 1, col));
                }
                // Check Left
                if col > 0 && !visited_set.contains(&(row, col - 1)) {
                    visit_queue.push_back((row, col - 1, runtime + 1));
                    visited_set.insert((row, col - 1));
                }
                // Check Right
                if col + 1 < racetrack[0].len() && !visited_set.contains(&(row, col + 1)) {
                    visit_queue.push_back((row, col + 1, runtime + 1));
                    visited_set.insert((row, col + 1));
                }
            }
            _ => (),
        }
    }
}
