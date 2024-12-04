advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u32> {
    //find "X" and try to complete the word by "wandering" in all directions
    let directions = [
        (0, 1),   // Right
        (0, -1),  // Left
        (1, 0),   // Down
        (-1, 0),  // Up
        (1, 1),   // Down-Right
        (1, -1),  // Down-Left
        (-1, 1),  // Up-Right
        (-1, -1), // Up-Left
    ];

    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let rows = grid.len();
    let cols = grid[0].len();

    let word: Vec<char> = "XMAS".chars().collect();

    let mut count = 0;

    for r in 0..rows {
        for c in 0..cols {
            if grid[r][c] != 'X' {
                continue;
            }

            for &(dr, dc) in &directions {
                if follow_word(&grid, &word[1..], r, c, dr, dc) {
                    count += 1;
                }
            }
        }
    }

    Some(count)
}

fn follow_word(
    grid: &[Vec<char>],
    word_residual: &[char],
    r: usize,
    c: usize,
    dr: isize,
    dc: isize,
) -> bool {
    let rows = grid.len() as isize;
    let cols = grid[0].len() as isize;

    let mut r = r as isize;
    let mut c = c as isize;

    for i in 0..word_residual.len() {
        r += dr;
        c += dc;
        if r < 0 || r >= rows || c < 0 || c >= cols {
            return false;
        }
        if grid[r as usize][c as usize] != word_residual[i] {
            return false;
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
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
