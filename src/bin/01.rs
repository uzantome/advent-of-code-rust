use std::collections::HashMap;

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let mut list_a = Vec::new();
    let mut list_b = Vec::new();

    for line in input.lines() {
        let mut parts = line.split_whitespace();
        if let (Some(a), Some(b)) = (parts.next(), parts.next()) {
            let a = a.parse::<u32>().unwrap();
            let b = b.parse::<u32>().unwrap();
            list_a.push(a);
            list_b.push(b);
        }
    }

    list_a.sort();
    list_b.sort();

    let sum_of_differences: u32 = list_a
        .iter()
        .zip(list_b.iter())
        .map(|(a, b)| (*a as i32 - *b as i32).unsigned_abs())
        .sum();

    Some(sum_of_differences)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut list_a = Vec::new();
    let mut list_b = Vec::new();

    for line in input.lines() {
        let mut parts = line.split_whitespace();
        if let (Some(a), Some(b)) = (parts.next(), parts.next()) {
            let a = a.parse::<u32>().unwrap();
            let b = b.parse::<u32>().unwrap();
            list_a.push(a);
            list_b.push(b);
        }
    }

    let mut right_occurence_count = HashMap::new();
    for &b in list_b.iter() {
        *right_occurence_count.entry(b).or_insert(0) += 1;
    }

    // // print right_occurence_count
    // for (key, value) in &right_occurence_count {
    //     println!("{}: {}", key, value);
    // }

    let mut similarity_score: u32 = 0;
    for &a in list_a.iter() {
        if let Some(count) = right_occurence_count.get(&a) {
            similarity_score += a * count;
        }
    }

    Some(similarity_score)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
