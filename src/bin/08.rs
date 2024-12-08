use glam::IVec2;
use std::collections::{HashMap, HashSet};

advent_of_code::solution!(8);

pub fn part_one(input: &str) -> Option<u32> {
    let mut antenna_groups: HashMap<char, Vec<IVec2>> = HashMap::new();
    let mut antinodes: HashSet<IVec2> = HashSet::new();
    let mut max_x = 0;
    let mut max_y = 0;

    for (y, line) in input.lines().enumerate() {
        for (x, freq) in line.chars().enumerate() {
            if freq != '.' {
                antenna_groups
                    .entry(freq)
                    .or_default()
                    .push(IVec2::new(x as i32, y as i32));
            }
            max_x = max_x.max(x);
        }
        max_y = max_y.max(y);
    }

    antenna_groups.values().for_each(|antennas| {
        for i in 0..antennas.len() {
            for j in (i + 1)..antennas.len() {
                let a = antennas[i];
                let b = antennas[j];

                let diff = a - b;

                let a1 = a + diff;
                let a2 = b - diff;

                if is_on_board(a1, max_x, max_y) {
                    antinodes.insert(a1);
                }
                if is_on_board(a2, max_x, max_y) {
                    antinodes.insert(a2);
                }
            }
        }
    });

    Some(antinodes.len() as u32)
}

fn is_on_board(coord: IVec2, max_x: usize, max_y: usize) -> bool {
    coord.x >= 0 && coord.y >= 0 && coord.x <= max_x as i32 && coord.y <= max_y as i32
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut antenna_groups: HashMap<char, Vec<IVec2>> = HashMap::new();
    let mut antinodes: HashSet<IVec2> = HashSet::new();
    let mut max_x = 0;
    let mut max_y = 0;

    for (y, line) in input.lines().enumerate() {
        for (x, freq) in line.chars().enumerate() {
            if freq != '.' {
                antenna_groups
                    .entry(freq)
                    .or_default()
                    .push(IVec2::new(x as i32, y as i32));
            }
            max_x = max_x.max(x);
        }
        max_y = max_y.max(y);
    }

    antenna_groups.values().for_each(|antennas| {
        for i in 0..antennas.len() {
            for j in (i + 1)..antennas.len() {
                let a = antennas[i];
                let b = antennas[j];

                let diff = a - b;

                let mut i = 0;
                loop {
                    let a1 = a + i * diff;
                    if !is_on_board(a1, max_x, max_y) {
                        break;
                    }
                    antinodes.insert(a1);
                    i += 1;
                }

                let mut i = 0;
                loop {
                    let a2 = b - i * diff;
                    if !is_on_board(a2, max_x, max_y) {
                        break;
                    }
                    antinodes.insert(a2);
                    i += 1;
                }
            }
        }
    });

    Some(antinodes.len() as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34));
    }
}
