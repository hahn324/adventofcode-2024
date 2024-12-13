use std::{error::Error, fs};

#[derive(Debug, Clone, Default)]
struct ClawMachine {
    button_a: (i64, i64),
    button_b: (i64, i64),
    prize: (i64, i64),
}
impl ClawMachine {
    fn new() -> Self {
        Default::default()
    }
}

enum Axis {
    X,
    Y,
}

enum GamePart {
    ButtonA,
    ButtonB,
    Prize,
}

fn main() -> Result<(), Box<dyn Error>> {
    let machine_details_source = fs::read_to_string("day-13/day13_input.txt")?;

    let mut claw_machines = create_claw_machines(&machine_details_source);

    let mut total_tokens = 0;
    for machine in claw_machines.iter() {
        total_tokens += win_prize(machine);
    }
    println!("Minimum tokens needed to win: {total_tokens}");

    let mut total_tokens = 0;
    for machine in claw_machines.iter_mut() {
        machine.prize.0 = machine.prize.0 + 10000000000000;
        machine.prize.1 = machine.prize.1 + 10000000000000;
        total_tokens += win_prize(machine);
    }
    println!("After conversion, minimum tokens needed to win: {total_tokens}");

    Ok(())
}

fn win_prize(machine: &ClawMachine) -> i64 {
    let a = (machine.button_b.0 * machine.prize.1 - machine.button_b.1 * machine.prize.0)
        / (machine.button_b.0 * machine.button_a.1 - machine.button_b.1 * machine.button_a.0);
    let b = (machine.prize.0 - machine.button_a.0 * a) / machine.button_b.0;
    // Verify values
    let x = machine.button_a.0 * a + machine.button_b.0 * b;
    let y = machine.button_a.1 * a + machine.button_b.1 * b;

    if x == machine.prize.0 && y == machine.prize.1 {
        a * 3 + b
    } else {
        0
    }
}

fn create_claw_machines(details_source: &str) -> Vec<ClawMachine> {
    let mut machines = Vec::new();

    let mut claw_machine = ClawMachine::new();
    let mut axis = Axis::X;
    let mut game_part = GamePart::ButtonA;

    let mut current_x = 0;

    let mut details_iter = details_source.chars();
    let mut cur_idx = 0;
    while let Some(c) = details_iter.next() {
        match c {
            'A' => game_part = GamePart::ButtonA,
            'B' => game_part = GamePart::ButtonB,
            'P' => game_part = GamePart::Prize,
            'X' => axis = Axis::X,
            'Y' => axis = Axis::Y,
            '0'..='9' => {
                let start_idx = cur_idx;
                cur_idx += 1;
                while let Some(c) = details_iter.next() {
                    match c {
                        '0'..='9' => cur_idx += 1,
                        _ => {
                            let value =
                                details_source[start_idx..cur_idx]
                                    .parse::<i64>()
                                    .expect(&format!(
                                        "Failed to parse {} to an i64.",
                                        &details_source[start_idx..cur_idx],
                                    ));
                            match axis {
                                Axis::X => current_x = value,
                                Axis::Y => match game_part {
                                    GamePart::ButtonA => claw_machine.button_a = (current_x, value),
                                    GamePart::ButtonB => claw_machine.button_b = (current_x, value),
                                    GamePart::Prize => {
                                        claw_machine.prize = (current_x, value);
                                        machines.push(claw_machine);
                                        claw_machine = ClawMachine::new();
                                    }
                                },
                            }
                            break;
                        }
                    }
                }
            }
            _ => (),
        }
        cur_idx += 1;
    }

    machines
}
