advent_of_code::solution!(4);

use itertools::Itertools;
use regex::Regex;
use std::collections::HashMap;

pub fn part_one(input: &str) -> Option<u32> {
    // Alternative way: find "X" and try to complete the word by "wandering" in all directions

    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let mut search_space = extract_horizontal_lines(&grid);
    search_space.extend(extract_vertical_lines(&grid));
    search_space.extend(extract_tlbr_diagonals(&grid));
    search_space.extend(extract_bltr_diagonals(&grid));

    let search_pattern = Regex::new(r"XMAS|SAMX").unwrap();

    let count = search_space
        .iter()
        .map(|line| search_pattern.find_iter(line).count() as u32)
        .sum();

    Some(count)
}

fn extract_horizontal_lines(grid: &[Vec<char>]) -> Vec<String> {
    grid.iter().map(|row| row.iter().collect()).collect()
}

fn extract_vertical_lines(grid: &[Vec<char>]) -> Vec<String> {
    let mut verticals = Vec::new();
    let cols = grid[0].len();
    for col in 0..cols {
        let mut vertical = String::new();
        for row in grid {
            vertical.push(row[col]);
        }
        verticals.push(vertical);
    }
    verticals
}

fn extract_tlbr_diagonals(grid: &[Vec<char>]) -> Vec<String> {
    let mut diagonals: HashMap<i32, Vec<char>> = HashMap::new();

    for (i, row) in grid.iter().enumerate() {
        for (j, &cell) in row.iter().enumerate() {
            let diff = i as i32 - j as i32;
            diagonals.entry(diff).or_default().push(cell);
        }
    }

    diagonals
        .into_iter()
        .sorted_by_key(|&(k, _)| k)
        .map(|(_, v)| v.into_iter().collect())
        .collect()
}

fn extract_bltr_diagonals(grid: &[Vec<char>]) -> Vec<String> {
    let mut diagonals: HashMap<i32, Vec<char>> = HashMap::new();

    for (i, row) in grid.iter().enumerate() {
        for (j, &cell) in row.iter().enumerate() {
            diagonals.entry(i as i32 + j as i32).or_default().push(cell);
        }
    }

    diagonals
        .into_iter()
        .sorted_by_key(|&(k, _)| k)
        .map(|(_, v)| v.into_iter().collect())
        .collect()
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
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
