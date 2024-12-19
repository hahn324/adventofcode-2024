use std::{
    collections::{HashMap, HashSet},
    error::Error,
    fs,
};

fn main() -> Result<(), Box<dyn Error>> {
    let towel_details = fs::read_to_string("day-19/day19_input.txt")?;
    let (towels, designs) = parse_towel_details(&towel_details);

    let mut num_possible_designs = 0;
    for design in designs.iter() {
        if is_design_possible(&towels, design) {
            num_possible_designs += 1;
        }
    }
    println!("Number of possible designs: {num_possible_designs}");

    let mut num_possible_arrangements = 0;
    let mut arrangements = HashMap::new();
    for design in designs.iter() {
        num_possible_arrangements += find_num_arrangements(&towels, design, &mut arrangements);
    }
    println!("Number of possible arrangements: {num_possible_arrangements}");

    Ok(())
}

fn is_design_possible(towels: &HashSet<&str>, design: &str) -> bool {
    if towels.contains(&design[..]) {
        return true;
    }
    for limit in 1..=design.len() {
        let found_towel = towels.contains(&design[..limit]);
        if found_towel && is_design_possible(towels, &design[limit..]) {
            return true;
        }
    }
    false
}

fn find_num_arrangements<'input>(
    towels: &HashSet<&'input str>,
    design: &'input str,
    arrangements: &mut HashMap<&'input str, usize>,
) -> usize {
    if let Some(num) = arrangements.get(&design) {
        return *num;
    }

    if design.len() == 0 {
        return 1;
    }

    let mut num_arrangements = 0;
    for limit in 1..=design.len() {
        if towels.contains(&design[..limit]) {
            match arrangements.get(&design[limit..]) {
                Some(num) => num_arrangements += *num,
                None => {
                    num_arrangements +=
                        find_num_arrangements(towels, &design[limit..], arrangements)
                }
            }
        }
    }
    arrangements.insert(design, num_arrangements);
    num_arrangements
}

fn parse_towel_details(towel_details: &str) -> (HashSet<&str>, Vec<&str>) {
    let mut details_iter = towel_details.chars();

    let mut towels = HashSet::new();
    let mut designs = Vec::new();

    let mut start = 0;
    let mut current = 0;

    while let Some(c) = details_iter.next() {
        match c {
            ',' | '\n' => {
                towels.insert(&towel_details[start..current]);
                if c == '\n' {
                    current += 1;
                    break;
                }
            }
            ' ' => start = current + 1,
            _ => (),
        }
        current += 1;
    }
    // Consume empty new line
    details_iter.next();
    current += 1;
    start = current;

    while let Some(c) = details_iter.next() {
        match c {
            '\n' => {
                designs.push(&towel_details[start..current]);
                start = current + 1;
            }
            _ => (),
        }
        current += 1;
    }

    (towels, designs)
}
