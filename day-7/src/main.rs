use std::{error::Error, fs};

fn main() -> Result<(), Box<dyn Error>> {
    let equations_source = fs::read_to_string("day-7/day7_input.txt")?;
    let equations = parse_equations(&equations_source);

    let mut calibration_result_two_operators = 0;
    for equation in equations.iter() {
        let mut operator_sequence = Vec::new();
        let operators = ['+', '*'];
        calibration_result_two_operators +=
            produce_test_result(equation, &mut operator_sequence, &operators);
    }

    println!("Total calibration results with two operators: {calibration_result_two_operators}");

    let mut calibration_result_three_operators = 0;
    for equation in equations.iter() {
        let mut operator_sequence = Vec::new();
        let operators = ['+', '*', '|'];
        calibration_result_three_operators +=
            produce_test_result(equation, &mut operator_sequence, &operators);
    }

    println!(
        "Total calibration results with three operators: {calibration_result_three_operators}"
    );

    Ok(())
}

#[derive(Debug)]
struct Equation {
    result: i64,
    operands: Vec<i64>,
}

fn parse_equations(data_input: &str) -> Vec<Equation> {
    let mut equations = Vec::new();

    let mut row_result = 0;
    let mut row_operands = Vec::new();

    let mut num_start_idx = 0;
    let mut current_idx = 0;
    let mut data_iter = data_input.chars();
    while let Some(c) = data_iter.next() {
        match c {
            '0'..='9' => current_idx += 1,
            ':' => {
                row_result = data_input[num_start_idx..current_idx]
                    .parse::<i64>()
                    .expect(&format!(
                        "Failed to parse {} to an i64.",
                        &data_input[num_start_idx..current_idx]
                    ));
                // Consume the white space that comes after a colon char.
                data_iter.next();
                current_idx += 2;
                num_start_idx = current_idx;
            }
            ' ' => {
                row_operands.push(
                    data_input[num_start_idx..current_idx]
                        .parse::<i64>()
                        .expect(&format!(
                            "Failed to parse {} to an i64.",
                            &data_input[num_start_idx..current_idx]
                        )),
                );
                current_idx += 1;
                num_start_idx = current_idx;
            }
            '\n' => {
                row_operands.push(
                    data_input[num_start_idx..current_idx]
                        .parse::<i64>()
                        .expect(&format!(
                            "Failed to parse {} to an i64.",
                            &data_input[num_start_idx..current_idx]
                        )),
                );
                let new_equation = Equation {
                    result: row_result,
                    operands: row_operands,
                };
                equations.push(new_equation);
                // Reset row_operands for next equation.
                row_operands = Vec::new();
                current_idx += 1;
                num_start_idx = current_idx;
            }
            _ => unreachable!("Found unexpected char while parsing equations input."),
        }
    }

    equations
}

fn produce_test_result(
    equation: &Equation,
    operator_sequence: &mut Vec<char>,
    operators: &[char],
) -> i64 {
    if operator_sequence.len() == equation.operands.len() - 1 {
        let mut res = equation.operands[0];
        for idx in 0..operator_sequence.len() {
            match operator_sequence[idx] {
                '+' => res += equation.operands[idx + 1],
                '*' => res *= equation.operands[idx + 1],
                '|' => {
                    res = format!("{res}{}", equation.operands[idx + 1])
                        .parse::<i64>()
                        .unwrap()
                }
                _ => panic!("Cannot handle operator {}", operator_sequence[idx]),
            }
        }
        if res == equation.result {
            return res;
        }
        return 0;
    }

    for &op in operators {
        operator_sequence.push(op);
        if produce_test_result(equation, operator_sequence, operators) == equation.result {
            return equation.result;
        }
        operator_sequence.pop();
    }

    0
}
