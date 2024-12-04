advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    let mut safe_count = 0;

    let reports: Vec<Vec<i32>> = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|n| n.parse().unwrap())
                .collect()
        })
        .collect();

    for report in reports {
        if is_safe(&report) {
            safe_count += 1;
        }
    }

    Some(safe_count)
}

fn is_safe(report: &[i32]) -> bool {
    #[derive(Debug, PartialEq)]
    enum Direction {
        Increasing,
        Decreasing,
        Unset,
    }

    let initial_state = (Direction::Unset, report[0]);

    let result = report
        .iter()
        .skip(1)
        .try_fold(initial_state, |(direction, prev), &current| {
            let delta = current - prev;

            // "Any two adjacent levels differ by at least one and at most three."
            if !(1..=3).contains(&delta.abs()) {
                return Err(false);
            }

            // "The levels are either all increasing or all decreasing."
            let new_direction = if delta < 0 {
                Direction::Decreasing
            } else {
                Direction::Increasing
            };

            if direction != Direction::Unset && new_direction != direction {
                return Err(false);
            }

            Ok((new_direction, current))
        });

    result.is_ok()
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut safe_count = 0;

    let reports: Vec<Vec<i32>> = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|n| n.parse().unwrap())
                .collect()
        })
        .collect();

    for report in reports {
        if is_safe(&report) {
            safe_count += 1;
            continue;
        }

        // check whether report can be made safe by removing exactly one level
        // VERY NAIVE IMPLEMENTATION
        for i in 0..report.len() {
            let mut report_copy = report.clone();
            report_copy.remove(i);
            if is_safe(&report_copy) {
                safe_count += 1;
                break;
            }
        }
    }

    Some(safe_count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
