use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::AddAssign;

#[derive(Debug)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Debug)]
enum Rotation {
    Left,
    Right,
}

#[derive(Debug)]
struct Carrier {
    position: Point2D,
    direction: Direction,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Point2D {
    x: isize,
    y: isize,
}

impl AddAssign for Point2D {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

fn xy_diff(direction: &Direction) -> Point2D {
    match direction {
        Direction::Right => Point2D { x: 1, y: 0 },
        Direction::Down => Point2D { x: 0, y: -1 },
        Direction::Left => Point2D { x: -1, y: 0 },
        Direction::Up => Point2D { x: 0, y: 1 },
    }
}

fn new_direction(current_direction: &Direction, rotate: Rotation) -> Direction {
    match rotate {
        Rotation::Left => match current_direction {
            Direction::Left => Direction::Down,
            Direction::Right => Direction::Up,
            Direction::Up => Direction::Left,
            Direction::Down => Direction::Right,
        },
        Rotation::Right => match current_direction {
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
        },
    }
}

fn tick(carrier: &mut Carrier, grid_map: &mut HashMap<Point2D, char>, infected_count: &mut usize) {
    if !grid_map.contains_key(&carrier.position) {
        grid_map.insert(carrier.position.clone(), '.');
    }
    let current_node = grid_map
        .get(&carrier.position)
        .expect("current node does not exist.");
    match current_node {
        '.' => {
            carrier.direction = new_direction(&carrier.direction, Rotation::Left);
            *grid_map.get_mut(&carrier.position).unwrap() = '#';
            *infected_count += 1;
            carrier.position += xy_diff(&carrier.direction);
        }
        '#' => {
            carrier.direction = new_direction(&carrier.direction, Rotation::Right);
            *grid_map.get_mut(&carrier.position).unwrap() = '.';
            carrier.position += xy_diff(&carrier.direction);
        }
        _ => panic!("Bad grid data: {}", current_node),
    }
}

fn parse_input(path: &str) -> HashMap<Point2D, char> {
    let file = File::open(path).unwrap();
    let buffered: Vec<String> = BufReader::new(file)
        .lines()
        .map(|line| line.expect("Could not parse line."))
        .collect();

    let dim = buffered.len() as isize;

    let mut grid_map: HashMap<Point2D, char> = HashMap::new();

    // change the origin to the middle of the grid
    let mut xid = dim / 2;
    let mut yid = -1 * dim / 2;
    for (i, l) in buffered.iter().enumerate() {
        for (j, ch) in l.chars().enumerate() {
            grid_map.insert(Point2D { x: yid, y: xid }, ch);
            yid += 1;
        }
        yid = -dim / 2;
        xid -= 1;
    }

    grid_map
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        let path = "/Users/sep/CLionProjects/adventofcode-2017/src/inputs/day22_test.txt";
        let grid_map = parse_input(path);
        println!("{:?}", grid_map);
    }

    #[test]
    fn test_sample_part1() {
        let path = "/Users/sep/CLionProjects/adventofcode-2017/src/inputs/day22_test.txt";
        let mut grid_map = parse_input(path);
        let burst_count = 7;
        let mut infected_count = 0;
        let mut carrier = Carrier {
            direction: Direction::Up,
            position: Point2D { x: 0, y: 0 },
        };
        for i in 0..burst_count {
            tick(&mut carrier, &mut grid_map, &mut infected_count);
        }
        assert_eq!(5, infected_count);
    }

    #[test]
    fn test_submission_part1() {
        let path = "/Users/sep/CLionProjects/adventofcode-2017/src/inputs/day22.txt";
        let mut grid_map = parse_input(path);
        let burst_count = 10000;
        let mut infected_count = 0;
        let mut carrier = Carrier {
            direction: Direction::Up,
            position: Point2D { x: 0, y: 0 },
        };
        for i in 0..burst_count {
            tick(&mut carrier, &mut grid_map, &mut infected_count);
        }
        println!("Part1: {}", infected_count);
    }
}
