advent_of_code::solution!(15);

use glam::IVec2;

#[derive(Debug)]
enum Direction {
    North,
    South,
    West,
    East,
}

impl Direction {
    fn from_char(c: char) -> Option<Self> {
        match c {
            '^' => Some(Direction::North),
            'v' => Some(Direction::South),
            '<' => Some(Direction::West),
            '>' => Some(Direction::East),
            _ => None,
        }
    }

    fn to_vec(&self) -> IVec2 {
        match self {
            Direction::North => IVec2::new(0, -1),
            Direction::South => IVec2::new(0, 1),
            Direction::West => IVec2::new(-1, 0),
            Direction::East => IVec2::new(1, 0),
        }
    }
}

fn print_board(board: &Vec<Vec<char>>) {
    for row in board {
        for &cell in row {
            print!("{}", cell);
        }
        println!();
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let parts: Vec<&str> = input.split("\n\n").collect();

    // indexed in (row, col) format: (y, x)
    let mut board: Vec<Vec<char>> = parts[0]
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let directions: Vec<Direction> = parts[1]
        .lines()
        .flat_map(|line| line.chars().filter_map(Direction::from_char))
        .collect();

    let mut position: IVec2 = board
        .iter()
        .enumerate()
        .find_map(|(y, row)| {
            row.iter()
                .position(|&c| c == '@')
                .map(|x| IVec2::new(x as i32, y as i32))
        })
        .expect("huh?? no robot on this board?");

    // print_board(&board);

    for dir in directions {
        // dbg!(&dir);

        let new_robot_pos = position + dir.to_vec();

        if board[new_robot_pos.y as usize][new_robot_pos.x as usize] == '#' {
            // encountered wall
            continue;
        }

        if board[new_robot_pos.y as usize][new_robot_pos.x as usize] == 'O' {
            // encountered object, try to move (possibly multiple) object(s)

            let mut new_obj_pos = new_robot_pos + dir.to_vec();
            // find first free coord in direction
            while board[new_obj_pos.y as usize][new_obj_pos.x as usize] != '#' {
                if board[new_obj_pos.y as usize][new_obj_pos.x as usize] == '.' {
                    // (b) found free spot: move object(s)
                    board[new_obj_pos.y as usize][new_obj_pos.x as usize] = 'O';
                    // also move robot
                    board[position.y as usize][position.x as usize] = '.';
                    board[new_robot_pos.y as usize][new_robot_pos.x as usize] = '@';
                    position = new_robot_pos;

                    break;
                }

                new_obj_pos += dir.to_vec();
            }

            // dont move robot if loop failed because
            // (a) a wall is in the way
            // (b) free spot was found and robot already moved
            continue;
        }

        board[position.y as usize][position.x as usize] = '.';
        board[new_robot_pos.y as usize][new_robot_pos.x as usize] = '@';
        position = new_robot_pos;

        // print_board(&board);
    }

    let mut gps_sum = 0;
    let board_height = board.len();
    let board_width = board[0].len();

    (0..board_height).for_each(|y| {
        (0..board_width).for_each(|x| {
            if board[y][x] == 'O' {
                let gps_coordinate = 100 * y as u32 + x as u32;
                gps_sum += gps_coordinate;
            }
        });
    });

    Some(gps_sum)
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
        assert_eq!(result, Some(10092));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
