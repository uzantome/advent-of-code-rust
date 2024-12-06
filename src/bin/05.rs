use std::collections::{HashMap, HashSet};

advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u32> {
    let parts: Vec<&str> = input.split("\n\n").collect();
    let rules_str = parts[0];
    let updates_str = parts[1];

    // parse rules as tuples
    let rules: Vec<(i32, i32)> = rules_str
        .lines()
        .map(|line| {
            let rule: Vec<&str> = line.split("|").collect();
            let a = rule[0].parse::<i32>().unwrap();
            let b = rule[1].parse::<i32>().unwrap();
            (a, b)
        })
        .collect();

    // put each updates' page-numbers as (page-num, index-in-update) tuple into a HashMap
    // also save the updates' middle element for later
    // Vec<(HashMap<pagenum, index-in-update>, middle-pagenum)
    let updates: Vec<(HashMap<i32, usize>, i32)> = updates_str
        .lines()
        .map(|line| {
            let page_numbers: Vec<i32> = line
                .split(',')
                .map(|page_number| page_number.parse::<i32>().unwrap())
                .collect();

            let middle_pagenum = page_numbers[page_numbers.len() / 2];

            let page_map: HashMap<i32, usize> = page_numbers
                .iter()
                .enumerate()
                .map(|(ix, &page_number)| (page_number, ix))
                .collect();

            (page_map, middle_pagenum)
        })
        .collect();

    // filter correctly ordered updates
    let correct_updates: Vec<&(HashMap<i32, usize>, i32)> = updates
        .iter()
        .filter(|update| is_valid_order(&update.0, &rules))
        .collect();

    let middle_pagenums: Vec<i32> = correct_updates.iter().map(|update| update.1).collect();

    Some(middle_pagenums.iter().sum::<i32>() as u32)
}

fn is_valid_order(update: &HashMap<i32, usize>, rules: &Vec<(i32, i32)>) -> bool {
    for rule in rules {
        let a = rule.0;
        let b = rule.1;
        if let (Some(ix_a), Some(ix_b)) = (update.get(&a), update.get(&b)) {
            if ix_a > ix_b {
                return false;
            }
        }
    }
    true
}

pub fn part_two(input: &str) -> Option<u32> {
    let parts: Vec<&str> = input.split("\n\n").collect();
    let rules_str = parts[0];
    let updates_str = parts[1];

    // parse rules as tuples
    let rules: Vec<(i32, i32)> = rules_str
        .lines()
        .map(|line| {
            let rule: Vec<&str> = line.split("|").collect();
            let a = rule[0].parse::<i32>().unwrap();
            let b = rule[1].parse::<i32>().unwrap();
            (a, b)
        })
        .collect();

    // put each updates' page-numbers as (page-num, index-in-update) tuple into a HashMap
    // also save the updates' middle element for later
    // Vec<(HashMap<pagenum, index-in-update>, Vec<paegenum>)
    let updates: Vec<(HashMap<i32, usize>, Vec<i32>)> = updates_str
        .lines()
        .map(|line| {
            let page_numbers: Vec<i32> = line
                .split(',')
                .map(|page_number| page_number.parse::<i32>().unwrap())
                .collect();

            let page_map: HashMap<i32, usize> = page_numbers
                .iter()
                .enumerate()
                .map(|(ix, &page_number)| (page_number, ix))
                .collect();

            (page_map, page_numbers)
        })
        .collect();

    // filter incorrectly ordered updates
    let incorrect_updates: Vec<&(HashMap<i32, usize>, Vec<i32>)> = updates
        .iter()
        .filter(|update| !is_valid_order(&update.0, &rules))
        .collect();

    let mut graph: HashMap<i32, Vec<i32>> = HashMap::new(); // adjacency list
                                                            // TODO: maybe replace Vec with HashSet to eliminate O(n) contains in sort_update
    for line in rules_str.lines() {
        let rule: Vec<&str> = line.split('|').collect();
        let a = rule[0].parse::<i32>().unwrap();
        let b = rule[1].parse::<i32>().unwrap();
        graph.entry(a).or_default().push(b);
    }

    let corrected_updates: Vec<Vec<i32>> = incorrect_updates
        .iter()
        .map(|update| sort_update(&update.1, &graph))
        .collect();

    let sum = corrected_updates
        .iter()
        .map(|update| update[update.len() / 2])
        .sum::<i32>();

    Some(sum as u32)
}

fn sort_update(update: &Vec<i32>, graph: &HashMap<i32, Vec<i32>>) -> Vec<i32> {
    let mut sorted_update = update.clone();
    sorted_update.sort_by(|&a, &b| {
        if let Some(neighbors) = graph.get(&a) {
            if neighbors.contains(&b) {
                return std::cmp::Ordering::Less;
            }
        }

        if let Some(neighbors) = graph.get(&b) {
            if neighbors.contains(&a) {
                return std::cmp::Ordering::Greater;
            }
        }

        std::cmp::Ordering::Equal
    });
    sorted_update
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
