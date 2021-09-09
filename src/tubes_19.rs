use std::collections::HashMap;

struct Point {
    x: isize,
    y: isize,
}

struct Packet {
    loc: Point,
    seen_letters: Vec<char>,
    direction: Direction,
}

#[derive(PartialEq, Eq, Hash)]
enum Direction {
    Left,
    Right,
    Down,
    Up,
}

fn follow_route(route_map: Vec<Vec<char>>) -> String {
    let mut step_count = 0;
    // map each direction enum to displacement in 2d
    let mut dir_map: HashMap<Direction, Point> = HashMap::new();
    dir_map.insert(Direction::Down, Point { x: 1, y: 0 });
    dir_map.insert(Direction::Left, Point { x: 0, y: -1 });
    dir_map.insert(Direction::Right, Point { x: 0, y: 1 });
    dir_map.insert(Direction::Up, Point { x: -1, y: 0 });

    let seen_vec: Vec<char> = Vec::new();
    let mut cur_x = 0;
    let mut cur_y = 0;
    // Find the entry point of the route
    for i in 0..route_map[0].len() {
        if route_map[0][i] != ' ' {
            cur_y = i;
        }
    }

    let mut packet = Packet {
        loc: Point {
            x: cur_x,
            y: cur_y as isize,
        },
        seen_letters: Vec::new(),
        direction: Direction::Down,
    };

    while route_map[packet.loc.x as usize][packet.loc.y as usize] != ' ' {
        let current_data = route_map[packet.loc.x as usize][packet.loc.y as usize];
        if current_data != ' ' && current_data != '+' && current_data != '|' && current_data != '-'
        {
            packet.seen_letters.push(current_data);
        }
        // '+' signifies a junction where a change of direction is needed
        // change all four possible directions, only set if new direction
        // is not the opposite of the current direction
        if current_data == '+' {
            let mut new_dir = Direction::Left;
            if (packet.loc.x as usize + 1) < route_map.len()
                && route_map[packet.loc.x as usize + 1][packet.loc.y as usize] != ' '
            {
                if packet.direction != Direction::Up {
                    new_dir = Direction::Down;
                }
            }
            if (packet.loc.x - 1) >= 0
                && route_map[packet.loc.x as usize - 1][packet.loc.y as usize] != ' '
            {
                if packet.direction != Direction::Down {
                    new_dir = Direction::Up;
                }
            }
            if (packet.loc.y as usize + 1) < route_map[packet.loc.x as usize].len()
                && route_map[packet.loc.x as usize][packet.loc.y as usize + 1] != ' '
            {
                if packet.direction != Direction::Left {
                    new_dir = Direction::Right;
                }
            }
            if (packet.loc.y - 1) >= 0
                && route_map[packet.loc.x as usize][packet.loc.y as usize - 1] != ' '
            {
                if packet.direction != Direction::Right {
                    new_dir = Direction::Left;
                }
            }

            packet.direction = new_dir;
        }
        let diff = dir_map.get(&packet.direction).unwrap();
        packet.loc.x += diff.x;
        packet.loc.y += diff.y;
        step_count += 1;
    }

    println!("Steps: {}", step_count);
    packet.seen_letters.into_iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::{BufRead, BufReader};

    #[test]
    fn test_sample_part1() {
        let path = "src/inputs/day19_test.txt";
        let file = File::open(path).unwrap();
        let buffered = BufReader::new(file).lines();
        let mut lines: Vec<Vec<char>> = Vec::new();
        for l in buffered {
            lines.push(l.unwrap().chars().collect());
        }

        // To simplify edge cases of finding the new direction,
        // all lines in the route map are right aligned with ' '
        for i in 0..6 {
            while lines[i].len() != 15 {
                lines[i].push(' ');
            }
        }

        let seen_letters = follow_route(lines);
        assert_eq!(seen_letters, "ABCDEF")
    }

    #[test]
    fn test_submission_part1() {
        let path = "src/inputs/day19.txt";
        let file = File::open(path).unwrap();
        let buffered = BufReader::new(file).lines();
        let mut lines: Vec<Vec<char>> = Vec::new();
        for l in buffered {
            lines.push(l.unwrap().chars().collect());
        }

        // To simplify edge cases of finding the new direction,
        // all lines in the route map are right aligned with ' '
        for i in 0..lines.len() {
            while lines[i].len() != 200 {
                lines[i].push(' ');
            }
        }

        let seen_letters = follow_route(lines);
        println!("Part1: {}", seen_letters);
    }
}
