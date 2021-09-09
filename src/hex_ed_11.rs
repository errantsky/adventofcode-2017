//   \ n  /
// nw +--+  ne (x)
//(z)/     \
// -+      +-
//   \     /
// sw +--+ se
//   / s  \
//      (y)
// https://www.redblobgames.com/grids/hexagons/

use std::cmp::max;

fn find_distance(input: Vec<&str>) -> usize {
    let mut current_position: (isize, isize, isize) = (0, 0, 0);
    let mut max_dist: usize = 0;
    for step in input {
        current_position = match step {
            "n" => (
                current_position.0 - 1,
                current_position.1,
                current_position.2 + 1,
            ),
            "nw" => (
                current_position.0 - 1,
                current_position.1 + 1,
                current_position.2,
            ),
            "ne" => (
                current_position.0,
                current_position.1 - 1,
                current_position.2 + 1,
            ),
            "s" => (
                current_position.0 + 1,
                current_position.1,
                current_position.2 - 1,
            ),
            "se" => (
                current_position.0 + 1,
                current_position.1 - 1,
                current_position.2,
            ),
            "sw" => (
                current_position.0,
                current_position.1 + 1,
                current_position.2 - 1,
            ),
            _ => panic!("D'oh!"),
        };
        max_dist = max(max_dist, hex_dist(current_position));
    }
    println!("Part 2: {}", max_dist);
    hex_dist(current_position)
}

fn hex_dist(current_position: (isize, isize, isize)) -> usize {
    ((current_position.0.abs() + current_position.1.abs() + current_position.2.abs()) / 2) as usize
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::{BufRead, BufReader};

    #[test]
    fn test_sample_part1() {
        let path = "src/inputs/day11_test.txt";
        let file = File::open(path).unwrap();
        let input = BufReader::new(file).lines().next().unwrap();
        let input = input.unwrap();
        let input = input.split(",").collect::<Vec<_>>();
        assert_eq!(find_distance(input), 3);
    }

    #[test]
    fn test_submission_part1() {
        let path = "src/inputs/day11.txt";
        let file = File::open(path).unwrap();
        let input = BufReader::new(file).lines().next().unwrap();
        let input = input.unwrap();
        let input = input.split(",").collect::<Vec<_>>();
        println!("Part 1: {}", find_distance(input));
    }
}
