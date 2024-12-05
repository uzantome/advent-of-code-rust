use std::collections::HashMap;

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
    None
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
        assert_eq!(result, None);
    }
}
