advent_of_code::solution!(15);

use glam::IVec2;
use std::collections::HashSet;

#[derive(Debug, PartialEq)]
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
    let parts: Vec<&str> = input.split("\n\n").collect();

    // indexed in (row, col) format: (y, x)
    let board: Vec<Vec<char>> = parts[0]
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let directions: Vec<Direction> = parts[1]
        .lines()
        .flat_map(|line| line.chars().filter_map(Direction::from_char))
        .collect();

    // Part Two board transformation
    let mut board: Vec<Vec<char>> = board
        .into_iter()
        .map(|row| {
            row.iter()
                .flat_map(|&cell| match cell {
                    '#' => vec!['#', '#'],
                    'O' => vec!['[', ']'],
                    '.' => vec!['.', '.'],
                    '@' => vec!['@', '.'],
                    _ => vec![cell],
                })
                .collect()
        })
        .collect();

    let mut robot_pos: IVec2 = board
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
        // println!("dir: {:?}", dir);

        let facing_coords_1 = robot_pos + dir.to_vec();
        let facing_cell = board[facing_coords_1.y as usize][facing_coords_1.x as usize];

        if facing_cell == '#' {
            // println!("encountered wall");
            continue;
        }

        if facing_cell == '[' || facing_cell == ']' {
            // encountered object, try to move (possibly multiple) object(s)
            // println!("object");

            // collect all (possibly) affected objects
            let mut aff_objs: HashSet<(IVec2, IVec2)> = HashSet::new();
            let mut outer_aff_objs: HashSet<(IVec2, IVec2)> = HashSet::new();

            // add first double-object to (possibly) affected objects
            if facing_cell == '[' {
                let facing_coords_2 = IVec2::new(facing_coords_1.x + 1, facing_coords_1.y);
                aff_objs.insert((facing_coords_1, facing_coords_2));
                outer_aff_objs.insert((facing_coords_1, facing_coords_2));
            }
            if facing_cell == ']' {
                let facing_coords_2 = IVec2::new(facing_coords_1.x - 1, facing_coords_1.y);
                aff_objs.insert((facing_coords_2, facing_coords_1));
                outer_aff_objs.insert((facing_coords_2, facing_coords_1));
            }

            // look around for other (possibly) affected objects and check whether moving is possible

            let mut move_possible = true;

            while let Some(&(obj_coords_1, obj_coords_2)) = outer_aff_objs.iter().next() {
                outer_aff_objs.remove(&(obj_coords_1, obj_coords_2));

                // check whether this affected outer object makes moving impossible
                if dir != Direction::East {
                    let next_obj_coords_1 = obj_coords_1 + dir.to_vec();
                    if board[next_obj_coords_1.y as usize][next_obj_coords_1.x as usize] == '#' {
                        move_possible = false;
                        break;
                    }
                }
                if dir != Direction::West {
                    let next_obj_coords_2 = obj_coords_2 + dir.to_vec();
                    if board[next_obj_coords_2.y as usize][next_obj_coords_2.x as usize] == '#' {
                        move_possible = false;
                        break;
                    }
                }

                let facing_coords_1 = if dir == Direction::East {
                    obj_coords_2 + dir.to_vec()
                } else {
                    // West, North, South
                    obj_coords_1 + dir.to_vec()
                };

                // add first object for next round
                let facing_cell = board[facing_coords_1.y as usize][facing_coords_1.x as usize];

                if facing_cell == '[' {
                    let facing_coords_2 = IVec2::new(facing_coords_1.x + 1, facing_coords_1.y);
                    aff_objs.insert((facing_coords_1, facing_coords_2));
                    outer_aff_objs.insert((facing_coords_1, facing_coords_2));
                }
                if facing_cell == ']' {
                    let facing_coords_2 = IVec2::new(facing_coords_1.x - 1, facing_coords_1.y);
                    aff_objs.insert((facing_coords_2, facing_coords_1));
                    outer_aff_objs.insert((facing_coords_2, facing_coords_1));
                }

                // moving (one object) horizontally can only ever affect one (other) object at a
                // time
                if dir == Direction::East || dir == Direction::West {
                    continue;
                }

                // add second object for next round
                let facing_coords_1 = obj_coords_2 + dir.to_vec();
                let facing_cell = board[facing_coords_1.y as usize][facing_coords_1.x as usize];

                if facing_cell == '[' {
                    let facing_coords_2 = IVec2::new(facing_coords_1.x + 1, facing_coords_1.y);
                    aff_objs.insert((facing_coords_1, facing_coords_2));
                    outer_aff_objs.insert((facing_coords_1, facing_coords_2));
                }
                if facing_cell == ']' {
                    let facing_coords_2 = IVec2::new(facing_coords_1.x - 1, facing_coords_1.y);
                    aff_objs.insert((facing_coords_2, facing_coords_1));
                    outer_aff_objs.insert((facing_coords_2, facing_coords_1));
                }
            }

            if move_possible {
                // move all affected objects
                // this has to happen in order to prevent overwriting an object's new position with
                // '.'
                let mut objs_to_move: Vec<_> = aff_objs.into_iter().collect();

                objs_to_move.sort_by(|a, b| match dir {
                    Direction::North => a.0.y.cmp(&b.0.y),
                    Direction::South => b.0.y.cmp(&a.0.y),
                    Direction::West => a.0.x.cmp(&b.0.x),
                    Direction::East => b.0.x.cmp(&a.0.x),
                });

                for (left, right) in objs_to_move {
                    board[left.y as usize][left.x as usize] = '.';
                    board[right.y as usize][right.x as usize] = '.';

                    let nleft = left + dir.to_vec();
                    board[nleft.y as usize][nleft.x as usize] = '[';
                    let nright = right + dir.to_vec();
                    board[nright.y as usize][nright.x as usize] = ']';
                }

                // move robot
                board[robot_pos.y as usize][robot_pos.x as usize] = '.';
                board[facing_coords_1.y as usize][facing_coords_1.x as usize] = '@';
                robot_pos = facing_coords_1;
            }

            continue;
        }

        board[robot_pos.y as usize][robot_pos.x as usize] = '.';
        board[facing_coords_1.y as usize][facing_coords_1.x as usize] = '@';
        robot_pos = facing_coords_1;

        // print_board(&board);
    }

    let mut gps_sum = 0;
    let board_height = board.len();
    let board_width = board[0].len();

    (0..board_height).for_each(|y| {
        (0..board_width).for_each(|x| {
            if board[y][x] == '[' {
                let gps_coordinate = 100 * y as u32 + x as u32;
                gps_sum += gps_coordinate;
            }
        });
    });

    Some(gps_sum)
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
        assert_eq!(result, Some(9021));
    }
}
