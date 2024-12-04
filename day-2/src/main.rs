use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let input_reports = fs::read_to_string("day-2/day2_input.txt")?;

    let mut num_safe_reports = 0;
    let mut num_safe_reports_with_dampener = 0;

    let mut current_report = Vec::new();
    let mut cur_num_idx = 0;
    for (cur_pos, input) in input_reports.chars().enumerate() {
        match input {
            ' ' | '\n' => {
                current_report.push(input_reports[cur_num_idx..cur_pos].parse::<i32>()?);
                // Number delimiters are just 1 index position long.
                cur_num_idx = cur_pos + 1;
            }
            '0'..='9' => continue,
            _ => unreachable!("Only expect ascii digits, spaces, and new line chars in the input"),
        }

        if input == '\n' {
            let check_report_increasing =
                check_report(&current_report, true, 0..current_report.len());
            let check_report_decreasing =
                check_report(&current_report, false, 0..current_report.len());

            match (check_report_increasing, check_report_decreasing) {
                (Ok(_), _) | (_, Ok(_)) => {
                    num_safe_reports += 1;
                    num_safe_reports_with_dampener += 1;
                }
                (Err(inc_err_indexes), Err(dec_err_indexes)) => {
                    let inc_first_dampened = check_report(
                        &current_report,
                        true,
                        (0..current_report.len()).filter(|idx| *idx != inc_err_indexes.0),
                    );
                    let inc_second_dampened = check_report(
                        &current_report,
                        true,
                        (0..current_report.len()).filter(|idx| *idx != inc_err_indexes.1),
                    );
                    let dec_first_dampened = check_report(
                        &current_report,
                        false,
                        (0..current_report.len()).filter(|idx| *idx != dec_err_indexes.0),
                    );
                    let dec_second_dampened = check_report(
                        &current_report,
                        false,
                        (0..current_report.len()).filter(|idx| *idx != dec_err_indexes.1),
                    );

                    if inc_first_dampened.is_ok()
                        || inc_second_dampened.is_ok()
                        || dec_first_dampened.is_ok()
                        || dec_second_dampened.is_ok()
                    {
                        num_safe_reports_with_dampener += 1;
                    }
                }
            }
            current_report.clear();
        }
    }

    println!("Number of Safe reports: {num_safe_reports}");
    println!("Number of Safe reports with Problem Dampener: {num_safe_reports_with_dampener}");

    Ok(())
}

/// Returns Result Ok(()) if report is Safe, and Err((idx_1, idx_2)) containing the
/// indexes of the two levels that made the report Unsafe.
/// A report is Safe if it follows these two rules:
///   - The levels are either all increasing or all decreasing.
///   - Any two adjacent levels differ by at least one and at most three.
fn check_report(
    report: &Vec<i32>,
    is_increasing: bool,
    mut report_idx_iter: impl Iterator<Item = usize>,
) -> Result<(), (usize, usize)> {
    if report.len() <= 1 {
        return Ok(());
    }

    let mut last_num_idx = report_idx_iter.next().unwrap();
    for cur_num_idx in report_idx_iter {
        let diff = (report[last_num_idx] - report[cur_num_idx]).abs();
        if diff < 1 || diff > 3 {
            return Err((last_num_idx, cur_num_idx));
        }
        match is_increasing {
            true if report[last_num_idx] >= report[cur_num_idx] => {
                return Err((last_num_idx, cur_num_idx))
            }
            false if report[last_num_idx] <= report[cur_num_idx] => {
                return Err((last_num_idx, cur_num_idx))
            }
            _ => (),
        }
        last_num_idx = cur_num_idx;
    }

    Ok(())
}
