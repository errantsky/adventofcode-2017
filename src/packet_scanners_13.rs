use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

fn part1(scanners: Vec<(u16, u16)>) -> u16 {
    let mut scanner_pos: HashMap<u16, u16> = HashMap::new();
    for (depth, range) in scanners.iter() {
        scanner_pos.insert(depth.clone(), 0);
    }
    let mut scanner_flag: HashMap<u16, bool> = HashMap::new();
    for (depth, range) in scanners.iter() {
        scanner_flag.insert(depth.clone(), false);
    }
    let mut scn: HashMap<u16, u16> = HashMap::new();
    for (k, v) in scanners.iter() {
        scn.insert(k.clone(), v.clone());
    }

    let mut current_depth: u16 = 0;
    let mut severity = 0;

    let max_depth = scanners.last().unwrap().0;

    while current_depth <= max_depth {
        if scanner_pos.contains_key(&current_depth) {
            if *scanner_pos.get(&current_depth).unwrap() == 0 {
                severity += current_depth * scn.get(&current_depth).unwrap()
            }
        }
        for k in scn.keys() {
            if *scanner_pos.get(k).unwrap() == scn.get(k).unwrap().clone() - 1 {
                if scanner_flag.get(k).unwrap() == &false {
                    *scanner_flag.get_mut(k).unwrap() = true;
                }
            } else if *scanner_pos.get(k).unwrap() == 0 {
                if scanner_flag.get(k).unwrap() == &true {
                    *scanner_flag.get_mut(k).unwrap() = false;
                }
            }
            if scanner_flag.get(k).unwrap() == &false {
                *scanner_pos.get_mut(k).unwrap() += 1;
            } else {
                *scanner_pos.get_mut(k).unwrap() -= 1;
            }
        }

        current_depth += 1;
    }

    return severity;
}

fn part2() {}

fn read_input(path: &str) -> Vec<(u16, u16)> {
    let file = File::open(path).unwrap();
    let buffered = BufReader::new(file).lines();
    let mut scanners: Vec<(u16, u16)> = Vec::new();
    for l in buffered {
        let line: Vec<u16> = l.unwrap().split(": ").map(|x| x.parse().unwrap()).collect();
        scanners.push((line[0], line[1]))
    }

    return scanners;
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read;

    #[test]
    fn test_input_sample1() {
        let scn = read_input("src/inputs/day13_test.txt");
        assert_eq!(part1(scn), 24);
    }

    // #[test]
    // fn test_input_sample2() {
    //     let scn = read_input("src/inputs/day13_test.txt");
    //     assert_eq!(part2(scn), 10);

    // }

    #[test]
    fn test_submission_1() {
        let scn = read_input("src/inputs/day13.txt");
        println!("Part 1: {}", part1(scn));
    }
}
