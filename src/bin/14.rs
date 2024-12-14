use glam::IVec2;
use std::collections::HashMap;
use std::thread;
use std::time::Duration;

advent_of_code::solution!(14);

pub fn part_one(input: &str) -> Option<u32> {
    let mut robots: Vec<(IVec2, IVec2)> = Vec::new();

    let width = 101;
    let height = 103;
    // let width = 11;
    // let height = 7;
    let board_size = IVec2::new(width, height);

    for line in input.lines() {
        let parts: Vec<_> = line.split_whitespace().collect();
        let p_str = parts[0]
            .trim_start_matches("p=")
            .split(',')
            .collect::<Vec<_>>();
        let v_str = parts[1]
            .trim_start_matches("v=")
            .split(',')
            .collect::<Vec<_>>();

        let position = IVec2::new(p_str[0].parse().unwrap(), p_str[1].parse().unwrap());
        let velocity = IVec2::new(v_str[0].parse().unwrap(), v_str[1].parse().unwrap());
        robots.push((position, velocity));
    }

    let positions_after_100s: Vec<IVec2> = robots
        .iter()
        .map(|(p, v)| mod_positive(p + v * 100, board_size))
        .collect();

    let x_divider = width / 2;
    let y_divider = height / 2;

    let quadrant_robot_counts = positions_after_100s
        .iter()
        .fold([0; 4], |mut counts, position| {
            match (position.x, position.y) {
                (x, y) if x > x_divider && y < y_divider => counts[0] += 1,
                (x, y) if x > x_divider && y > y_divider => counts[1] += 1,
                (x, y) if x < x_divider && y > y_divider => counts[2] += 1,
                (x, y) if x < x_divider && y < y_divider => counts[3] += 1,
                _ => {}
            }
            counts
        });

    let product = quadrant_robot_counts.iter().product();

    Some(product)
}

fn mod_positive(a: IVec2, b: IVec2) -> IVec2 {
    IVec2::new(((a.x % b.x) + b.x) % b.x, ((a.y % b.y) + b.y) % b.y)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut robots: Vec<(IVec2, IVec2)> = Vec::new();

    let width = 101;
    let height = 103;
    // let width = 11;
    // let height = 7;
    let board_size = IVec2::new(width, height);
    let x_divider = width / 2;
    let y_divider = height / 2;

    for line in input.lines() {
        let parts: Vec<_> = line.split_whitespace().collect();
        let p_str = parts[0]
            .trim_start_matches("p=")
            .split(',')
            .collect::<Vec<_>>();
        let v_str = parts[1]
            .trim_start_matches("v=")
            .split(',')
            .collect::<Vec<_>>();

        let position = IVec2::new(p_str[0].parse().unwrap(), p_str[1].parse().unwrap());
        let velocity = IVec2::new(v_str[0].parse().unwrap(), v_str[1].parse().unwrap());
        robots.push((position, velocity));
    }

    for iteration in 0..10000 {
        let positions_after_iterations: Vec<IVec2> = robots
            .iter()
            .map(|(p, v)| mod_positive(p + v * iteration, board_size))
            .collect();

        // used for printing the board
        let mut board: HashMap<IVec2, u32> = HashMap::new();
        for &position in &positions_after_iterations {
            *board.entry(position).or_insert(0) += 1;
        }

        let quadrant_robot_counts =
            positions_after_iterations
                .iter()
                .fold([0; 4], |mut counts, position| {
                    match (position.x, position.y) {
                        (x, y) if x > x_divider && y < y_divider => counts[0] += 1,
                        (x, y) if x > x_divider && y > y_divider => counts[1] += 1,
                        (x, y) if x < x_divider && y > y_divider => counts[2] += 1,
                        (x, y) if x < x_divider && y < y_divider => counts[3] += 1,
                        _ => {}
                    }
                    counts
                });

        let total_robots: u32 = quadrant_robot_counts.iter().sum();
        let expected = total_robots as f32 / 4.0;
        let is = *quadrant_robot_counts.iter().max().unwrap() as f32;

        if is > expected * 1.8 {
            println!("Iteration: {}", iteration);
            for y in 0..height {
                for x in 0..width {
                    let pos = IVec2::new(x, y);
                    if let Some(&count) = board.get(&pos) {
                        print!("{}", count);
                    } else {
                        print!(" ");
                    }
                }
                println!();
            }
            println!();
            thread::sleep(Duration::from_millis(300));
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        //let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        //assert_eq!(result, Some(12));
    }

    #[test]
    fn test_part_two() {
        //let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        //assert_eq!(result, None);
    }
}
