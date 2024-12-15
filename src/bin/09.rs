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

pub fn part_two(input: &str) -> Option<u64> {
    let mut blocks: Vec<(i32, u32)> = input
        .trim()
        .chars()
        .filter_map(|c| c.to_digit(10))
        .enumerate()
        .map(|(i, size)| {
            if i % 2 == 0 {
                // file
                (i as i32 / 2, size)
            } else {
                // free space
                (-1, size)
            }
        })
        .collect();

    // for i in (0..blocks.len()).rev() {
    let mut i = blocks.len() - 1;
    while i > 0 {
        if blocks[i].0 == -1 {
            // skip free space
            i -= 1;
            continue;
        }

        let filesize = blocks[i].1;

        let mut j = 0;
        while j < i {
            if blocks[j].0 != -1 {
                // skip files
                j += 1;
                continue;
            }

            let freesize = blocks[j].1;

            if freesize < filesize {
                j += 1;
                continue;
            }

            // found gap: move file
            blocks[j] = blocks[i];

            // move "displaced" free space to moved file's old position
            blocks[i] = (-1, filesize);
            if (i + 1) < blocks.len() && blocks[i + 1].0 == -1 {
                blocks[i].1 += blocks[i + 1].1;
                blocks.remove(i + 1);
            }
            if blocks[i - 1].0 == -1 {
                blocks[i - 1].1 += blocks[i].1;
                blocks.remove(i);
            }

            // if free space is left after moving file, extend existing free block or
            // insert new one
            let slacksize = freesize - filesize;
            if slacksize > 0 {
                if blocks[j + 1].0 == -1 {
                    blocks[j + 1].1 += slacksize;
                } else {
                    blocks.insert(j + 1, (-1, slacksize));
                }
            }

            // stop looking for gaps for this file
            break;
        }

        i -= 1;
    }

    let sorted_map: Vec<i32> = blocks
        .into_iter()
        .flat_map(|(id, size)| {
            if id == -1 {
                std::iter::repeat(-1)
                    .take(size as usize)
                    .collect::<Vec<_>>()
            } else {
                std::iter::repeat(id)
                    .take(size as usize)
                    .collect::<Vec<_>>()
            }
        })
        .collect();

    let checksum: u64 = sorted_map.iter().enumerate().fold(0, |acc, (pos, &id)| {
        if id == -1 {
            acc
        } else {
            acc + pos as u64 * id as u64
        }
    });

    Some(checksum)
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
        assert_eq!(result, Some(2858));
    }
}
