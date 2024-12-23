use std::{
    cmp,
    collections::{HashMap, VecDeque},
    error::Error,
    fs,
};

fn main() -> Result<(), Box<dyn Error>> {
    let door_codes: Vec<Vec<char>> = fs::read_to_string("day-21/day21_input.txt")?
        .trim()
        .split('\n')
        .map(|code| code.trim().chars().collect())
        .collect();

    let numeric_keypad = vec![
        vec!['7', '8', '9'],
        vec!['4', '5', '6'],
        vec!['1', '2', '3'],
        vec![' ', '0', 'A'],
    ];
    let numeric_keypad_lut = compute_all_movements(&numeric_keypad);

    let directional_keypad = vec![vec![' ', '^', 'A'], vec!['<', 'v', '>']];
    let directional_keypad_lut = compute_all_movements(&directional_keypad);

    let mut sequence_len_cache: HashMap<(Vec<char>, usize), usize> = HashMap::new();
    let mut complexities_sum = 0;
    let num_directional_keypads = 26;

    for numeric_code in door_codes {
        let mut first_directional_keypad = vec![];
        let mut sequence_options = vec![];

        let mut start = 'A';
        for target in numeric_code.iter() {
            let sequence = numeric_keypad_lut
                .get(&(start, *target))
                .expect("Expect to find ({start}, {target}) in numeric lut.");
            sequence_options.push(sequence);
            start = *target;
        }
        combine_sequence_options(0, vec![], &sequence_options, &mut first_directional_keypad);

        let mut person_sequence_len = usize::MAX;
        for code in first_directional_keypad.iter() {
            let sequence_len = find_shortest_sequence_len(
                'A',
                code,
                &directional_keypad_lut,
                2,
                num_directional_keypads,
                &mut sequence_len_cache,
            );
            person_sequence_len = cmp::min(person_sequence_len, sequence_len);
        }

        let code_numeric_part = &numeric_code[..numeric_code.len() - 1]
            .iter()
            .collect::<String>()
            .parse::<usize>()
            .expect(&format!("Failed to parse number from numeric door code"));

        let complexity = person_sequence_len * code_numeric_part;
        complexities_sum += complexity;
    }
    println!("Sum of complexities: {complexities_sum}");

    Ok(())
}

fn find_shortest_sequence_len(
    mut start: char,
    code: &Vec<char>,
    directional_lut: &HashMap<(char, char), Vec<Vec<char>>>,
    current_keypad_num: usize,
    num_total_keypad: usize,
    sequence_len_cache: &mut HashMap<(Vec<char>, usize), usize>,
) -> usize {
    if let Some(sequence_len) = sequence_len_cache.get(&(code.clone(), current_keypad_num)) {
        return *sequence_len;
    }

    let mut final_sequence_len = 0;
    let mut target_sequences = vec![];

    for target in code {
        let key = (start, *target);
        let sequence = directional_lut
            .get(&key)
            .expect("Expect to fine {key:?} in directional LUT.");
        start = *target;
        target_sequences.push(sequence);
    }

    for sequence_options in target_sequences {
        let mut min_sequence_len = usize::MAX;
        for sequence in sequence_options {
            if current_keypad_num == num_total_keypad {
                min_sequence_len = sequence.len();
                break;
            } else {
                let sequence_len = find_shortest_sequence_len(
                    'A',
                    sequence,
                    directional_lut,
                    current_keypad_num + 1,
                    num_total_keypad,
                    sequence_len_cache,
                );
                min_sequence_len = cmp::min(sequence_len, min_sequence_len);
            }
        }
        final_sequence_len += min_sequence_len;
    }
    sequence_len_cache.insert((code.clone(), current_keypad_num), final_sequence_len);
    final_sequence_len
}

fn compute_all_movements(keypad: &Vec<Vec<char>>) -> HashMap<(char, char), Vec<Vec<char>>> {
    let row_bound = keypad.len();
    let col_bound = keypad[0].len();
    let mut sequences = HashMap::new();
    for s1 in 0..row_bound {
        for s2 in 0..col_bound {
            for t1 in 0..row_bound {
                for t2 in 0..col_bound {
                    if keypad[s1][s2] == ' ' || keypad[t1][t2] == ' ' {
                        continue;
                    }
                    sequences.insert(
                        (keypad[s1][s2], keypad[t1][t2]),
                        compute_sequence((s1, s2), (t1, t2), &keypad),
                    );
                }
            }
        }
    }
    sequences
}

fn compute_sequence(
    start: (usize, usize),
    target: (usize, usize),
    keypad: &Vec<Vec<char>>,
) -> Vec<Vec<char>> {
    let row_bound = keypad.len();
    let col_bound = keypad[0].len();

    let mut visit_queue = VecDeque::new();
    visit_queue.push_back((start.0, start.1, vec![]));
    let mut visited = vec![];
    let mut sequences: Vec<Vec<char>> = vec![];

    while let Some(mut cur_pos) = visit_queue.pop_front() {
        let row = cur_pos.0;
        let col = cur_pos.1;
        visited.push((row, col));

        // Exit early if path is longer than shortest.
        if !sequences.is_empty() && cur_pos.2.len() >= sequences[0].len() {
            continue;
        }
        if (row, col) == target {
            // Add sequence to target to main sequence.
            cur_pos.2.push('A');
            sequences.push(cur_pos.2);
            continue;
        }
        // Check Up
        if row > 0 && keypad[row - 1][col] != ' ' && !visited.contains(&(row - 1, col)) {
            let mut cur_seq = cur_pos.2.clone();
            cur_seq.push('^');
            visit_queue.push_back((row - 1, col, cur_seq));
        }
        // Check Down
        if row + 1 < row_bound && keypad[row + 1][col] != ' ' && !visited.contains(&(row + 1, col))
        {
            let mut cur_seq = cur_pos.2.clone();
            cur_seq.push('v');
            visit_queue.push_back((row + 1, col, cur_seq));
        }
        // Check Left
        if col > 0 && keypad[row][col - 1] != ' ' && !visited.contains(&(row, col - 1)) {
            let mut cur_seq = cur_pos.2.clone();
            cur_seq.push('<');
            visit_queue.push_back((row, col - 1, cur_seq));
        }
        // Check Right
        if col + 1 < col_bound && keypad[row][col + 1] != ' ' && !visited.contains(&(row, col + 1))
        {
            let mut cur_seq = cur_pos.2.clone();
            cur_seq.push('>');
            visit_queue.push_back((row, col + 1, cur_seq));
        }
    }

    sequences
}

fn combine_sequence_options(
    level_idx: usize,
    cur_sequence: Vec<char>,
    sequence_options: &Vec<&Vec<Vec<char>>>,
    final_sequences: &mut Vec<Vec<char>>,
) {
    for sequence in sequence_options[level_idx].iter() {
        let mut tmp_seq = cur_sequence.clone();
        tmp_seq.append(&mut sequence.clone());
        if level_idx == sequence_options.len() - 1 {
            final_sequences.push(tmp_seq);
        } else {
            combine_sequence_options(level_idx + 1, tmp_seq, sequence_options, final_sequences);
        }
    }
}
