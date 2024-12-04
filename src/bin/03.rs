advent_of_code::solution!(3);

use regex::Regex;

pub fn part_one(input: &str) -> Option<u32> {
    // https://regex101.com/r/wq49K5/1
    let mul_pattern = Regex::new(r"mul\((?P<x>\d{1,3}),(?P<y>\d{1,3})\)").unwrap();

    let sum = mul_pattern
        .captures_iter(input)
        .map(|mul| {
            let a = mul["x"].parse::<u32>().unwrap();
            let b = mul["y"].parse::<u32>().unwrap();
            a * b
        })
        .sum();

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    // https://regex101.com/r/4HfUt2/1
    let mul_pattern =
        Regex::new(r"(?P<do>do\(\))|(?P<dont>don't\(\))|mul\((?P<x>\d{1,3}),(?P<y>\d{1,3})\)")
            .unwrap();

    let mut mul_is_allowed = true;

    let sum = mul_pattern
        .captures_iter(input)
        .map(|caps| {
            if caps.name("do").is_some() {
                mul_is_allowed = true;
                return 0;
            }

            if caps.name("dont").is_some() {
                mul_is_allowed = false;
                return 0;
            }

            if !mul_is_allowed {
                return 0;
            }

            let a = caps["x"].parse::<u32>().unwrap();
            let b = caps["y"].parse::<u32>().unwrap();
            a * b
        })
        .sum();

    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(48));
    }
}
