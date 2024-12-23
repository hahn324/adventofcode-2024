use std::{
    collections::{HashMap, HashSet},
    error::Error,
    fs,
};

type ComputerName = [char; 2];

fn main() -> Result<(), Box<dyn Error>> {
    let mut connections_adjacency_list: HashMap<ComputerName, Vec<ComputerName>> = HashMap::new();
    fs::read_to_string("day-23/day23_input.txt")?
        .trim()
        .split('\n')
        .for_each(|conn| {
            let mut conn_chars_iter = conn.chars();

            let mut comp_1 = [char::default(); 2];
            comp_1[0] = conn_chars_iter.next().unwrap();
            comp_1[1] = conn_chars_iter.next().unwrap();

            // Consume the '-' between the two computer names in the connection.
            conn_chars_iter.next();

            let mut comp_2 = [char::default(); 2];
            comp_2[0] = conn_chars_iter.next().unwrap();
            comp_2[1] = conn_chars_iter.next().unwrap();

            connections_adjacency_list
                .entry(comp_1)
                .and_modify(|conns| conns.push(comp_2))
                .or_insert(vec![comp_2]);
            connections_adjacency_list
                .entry(comp_2)
                .and_modify(|conns| conns.push(comp_1))
                .or_insert(vec![comp_1]);
        });

    let mut sets_of_three = vec![];
    let mut visited_set = HashSet::new();
    for (&first_comp, first_comp_conn_list) in connections_adjacency_list.iter() {
        for second_comp_idx in 0..first_comp_conn_list.len() {
            let second_comp = first_comp_conn_list[second_comp_idx];
            if visited_set.contains(&second_comp) {
                continue;
            }
            let second_comp_conn_list = connections_adjacency_list
                .get(&first_comp_conn_list[second_comp_idx])
                .unwrap();

            for third_comp_idx in second_comp_idx..first_comp_conn_list.len() {
                let third_comp = first_comp_conn_list[third_comp_idx];
                if visited_set.contains(&third_comp) {
                    continue;
                }
                if second_comp_conn_list.contains(&third_comp) {
                    if first_comp[0] == 't' || second_comp[0] == 't' || third_comp[0] == 't' {
                        sets_of_three.push((first_comp, second_comp, third_comp));
                    }
                }
            }
        }
        visited_set.insert(first_comp);
    }
    visited_set.clear();
    println!(
        "Number of sets that have a computer that start with 't': {}",
        sets_of_three.len()
    );

    let mut lan_party = vec![];
    let mut possible_lan_party = vec![];
    for (&comp, comp_conn_list) in connections_adjacency_list.iter() {
        possible_lan_party.push(comp);
        for start_idx in 0..comp_conn_list.len() {
            for other_comp_idx in start_idx..comp_conn_list.len() {
                let other_comp = comp_conn_list[other_comp_idx];
                let other_comp_conn_list = connections_adjacency_list.get(&other_comp).unwrap();
                if possible_lan_party
                    .iter()
                    .all(|c| other_comp_conn_list.contains(c))
                {
                    possible_lan_party.push(other_comp);
                }
            }
            if possible_lan_party.len() > lan_party.len() {
                lan_party = possible_lan_party;
            }
            possible_lan_party = vec![comp];
        }
        possible_lan_party = vec![];
    }

    lan_party.sort();
    println!(
        "LAN party password: {}",
        lan_party
            .iter()
            .map(|name| name.iter().collect::<String>())
            .collect::<Vec<String>>()
            .join(",")
    );

    Ok(())
}
