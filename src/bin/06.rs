advent_of_code::solution!(6);

use rayon::prelude::*;
use std::collections::HashSet;

pub fn part_one(input: &str) -> Option<u32> {
    let map: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let directions: [(i32, i32); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)]; // Up, Right, Down, Left
    let mut dir_index: usize = 0;
    let mut position: (usize, usize) = (0, 0);
    let mut visited: HashSet<(usize, usize)> = HashSet::new();

    for (i, row) in map.iter().enumerate() {
        for (j, c) in row.iter().enumerate() {
            if *c == '^' {
                position = (i, j);
                break;
            }
        }
    }

    loop {
        visited.insert(position);

        // determine next step
        let (x, y) = position;
        let (dx, dy) = directions[dir_index];
        let (nx, ny) = (x as i32 + dx, y as i32 + dy);

        // check if step leaves map
        if nx < 0 || nx >= map.len() as i32 || ny < 0 || ny >= map[0].len() as i32 {
            // End of map
            break;
        }
        let (nx, ny) = (nx as usize, ny as usize);

        // turn 90° right if encountering object
        if map[nx][ny] == '#' {
            dir_index = (dir_index + 1) % 4;
            continue;
        }

        position = (nx, ny);
    }

    Some(visited.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let map: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let directions: [(isize, isize); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)]; // Up, Right, Down, Left
    let mut start_position: (usize, usize) = (0, 0);

    for (i, row) in map.iter().enumerate() {
        for (j, c) in row.iter().enumerate() {
            if *c == '^' {
                start_position = (i, j);
                break;
            }
        }
    }

    let mut position = start_position;
    let mut dir_index: usize = 0;
    let mut visited: HashSet<(usize, usize)> = HashSet::new();

    loop {
        visited.insert(position);

        // determine next step
        let (x, y) = position;
        let (dx, dy) = directions[dir_index];
        let (nx, ny) = (x as isize + dx, y as isize + dy);

        // check if step leaves map
        if nx < 0 || nx >= map.len() as isize || ny < 0 || ny >= map[0].len() as isize {
            // End of map
            break;
        }
        let (nx, ny) = (nx as usize, ny as usize);

        // turn 90° right if encountering object
        if map[nx][ny] == '#' {
            dir_index = (dir_index + 1) % 4;
            continue;
        }

        position = (nx, ny);
    }

    //
    // Part Two
    //

    // try to place an obstruction at a visited position and check if this causes a loop
    let num_placeable_obstructions = visited
        .par_iter()
        .filter_map(|&(i, j)| {
            if map[i][j] == '.' {
                let mut map_copy = map.clone();
                map_copy[i][j] = '#'; // add obstruction

                if simulate(&map_copy, start_position, 0, &directions) {
                    Some(1)
                } else {
                    None
                }
            } else {
                None
            }
        })
        .sum();

    Some(num_placeable_obstructions)
}

/// returns whether a loop was found
/// `true` if loop was found
/// `false` if end of map was reached
fn simulate(
    map: &[Vec<char>],
    mut position: (usize, usize),
    mut dir_index: usize,
    directions: &[(isize, isize)],
) -> bool {
    // HashSet<position, direction-index>
    let mut visited: HashSet<((usize, usize), usize)> = HashSet::new();
    loop {
        if !visited.insert((position, dir_index)) {
            return true; // Loop detected
        }

        // determine next step
        let (x, y) = position;
        let (dx, dy) = directions[dir_index];
        let (nx, ny) = (x as isize + dx, y as isize + dy);

        // check if step leaves map
        if nx < 0 || nx >= map.len() as isize || ny < 0 || ny >= map[0].len() as isize {
            return false;
        }
        let (nx, ny) = (nx as usize, ny as usize);

        // turn 90° right if encountering object
        if map[nx][ny] == '#' {
            dir_index = (dir_index + 1) % 4;
            continue;
        }

        position = (nx, ny);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
