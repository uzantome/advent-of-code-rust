advent_of_code::solution!(7);

pub fn part_one(input: &str) -> Option<u64> {
    let equations: Vec<(u64, Vec<u32>)> = input
        .lines()
        .map(|line| {
            let parts: Vec<&str> = line.split(": ").collect();
            let test_value: u64 = parts[0].parse().unwrap();
            let numbers: Vec<u32> = parts[1].split(" ").map(|x| x.parse().unwrap()).collect();
            (test_value, numbers)
        })
        .collect();

    let sum: u64 = equations
        .iter()
        .filter(|(test_value, numbers)| is_solvable(*test_value, numbers, numbers.len() - 1))
        .map(|(test_value, _)| test_value)
        .sum();

    Some(sum)
}

/// Checks whether the test value can be solved by combining the given numbers (in the given order) with ADD or MUL operations.
fn is_solvable(test_value: u64, numbers: &[u32], index: usize) -> bool {
    if index == 0 {
        return numbers[0] as u64 == test_value;
    }

    let a = numbers[index];
    let i = index - 1;

    let t = test_value as i64 - a as i64;
    if t >= 0 && is_solvable(t as u64, numbers, i) {
        return true;
    }

    let t = test_value / a as u64;
    let no_remainder = test_value % a as u64 == 0;
    if no_remainder && is_solvable(t, numbers, i) {
        return true;
    }

    false
}

/// Checks whether the test value can be solved by combining the given numbers (in the given order) with ADD, MUL or CONCAT operations.
/// works in the reverse way as is_solvable
fn is_solvable_2(test_value: u64, numbers: &[u32], index: usize, curr: u64) -> bool {
    if curr > test_value {
        return false;
    }

    if index == numbers.len() {
        return curr == test_value;
    }

    let a = numbers[index];
    let i = index + 1;

    let t = curr + a as u64;
    if is_solvable_2(test_value, numbers, i, t) {
        return true;
    }

    let t = curr * a as u64;
    if is_solvable_2(test_value, numbers, i, t) {
        return true;
    }

    let concat = format!("{}{}", curr, a);
    let t = concat.parse().unwrap();
    if is_solvable_2(test_value, numbers, i, t) {
        return true;
    }

    false
}

pub fn part_two(input: &str) -> Option<u64> {
    let equations: Vec<(u64, Vec<u32>)> = input
        .lines()
        .map(|line| {
            let parts: Vec<&str> = line.split(": ").collect();
            let test_value: u64 = parts[0].parse().unwrap();
            let numbers: Vec<u32> = parts[1].split(" ").map(|x| x.parse().unwrap()).collect();
            (test_value, numbers)
        })
        .collect();

    let sum: u64 = equations
        .iter()
        .filter(|(test_value, numbers)| is_solvable_2(*test_value, numbers, 0, 0))
        .map(|(test_value, _)| test_value)
        .sum();

    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }
}
