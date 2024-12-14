advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<u64> {
    let mut blocks: Vec<i32> = input
        .trim()
        .chars()
        .filter_map(|c| c.to_digit(10))
        .enumerate()
        .flat_map(|(i, size)| {
            if i % 2 == 0 {
                // file
                std::iter::repeat(i as i32 / 2)
                    .take(size as usize)
                    .collect::<Vec<_>>()
            } else {
                // free space
                std::iter::repeat(-1)
                    .take(size as usize)
                    .collect::<Vec<_>>()
            }
        })
        .collect();

    for i in 0..blocks.len() {
        if blocks[i] == -1 {
            for j in (i + 1..blocks.len()).rev() {
                if blocks[j] != -1 {
                    blocks.swap(i, j);
                    break;
                }
            }
        }
    }

    let checksum: u64 = blocks.iter().enumerate().fold(0, |acc, (pos, &id)| {
        if id == -1 {
            acc
        } else {
            acc + pos as u64 * id as u64
        }
    });

    Some(checksum)
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
        assert_eq!(result, Some(1928));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1928));
    }
}
