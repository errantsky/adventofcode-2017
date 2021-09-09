


use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use regex::Regex;

fn find_bottom_program(program_map: HashMap<String, (usize, Vec<String>)>) -> String {
    let mut parent_map: HashMap<String, String> = HashMap::new();
    for (parent_name, val) in program_map.iter() {
            let children = &val.1;
            for child_name in children.iter() {
                parent_map.insert(child_name.clone(), parent_name.clone());
            }
    }

    program_map.keys()
        .filter(|x| !parent_map.contains_key(x.as_str()))
        .next()
        .unwrap()
        .clone()

}

// I failed to get part 2 working
// I ended up studying it from https://github.com/thejpster/rust-advent-of-code/blob/master/src/m2017/problem_7.rs
//
// fn part2(root: String, pmap: HashMap<String, Vec<String>>, wmap: HashMap<String, u32>) -> u32 {
//     let mut weights = Vec::new();
//     for child in pmap.get(root.as_str()).unwrap() {
//         let mut sum = find_sum(child.clone(), &pmap, &wmap);
//         weights.push((child.clone(), sum));
//     }
//
//
//         0
// }
//
// fn find_anomaly(weights: &Vec<(String, u32)>) -> String {
//     for (n, w) in weights.iter() {
//        if weights.iter().filter(|(c, n)| n == w).count() == 1 {
//            n.clone()
//        }
//     }
//     "Nope".to_string()
// }
//
// fn find_sum(root: String, pmap: &HashMap<String, Vec<String>>, wmap: &HashMap<String, u32>) -> u32 {
//     if !pmap.contains_key(root.as_str()) {
//         wmap.get(root.as_str()).unwrap()
//     } else {
//         let mut sum = 0;
//         for child in pmap.get(root.as_str()).unwrap() {
//             sum += find_sum(child.clone(), pmap, wmap)
//         }
//     }
//     sum
// }


fn parse_input(path: &str) -> HashMap<String, (usize, Vec<String>)> {
    let file = File::open(path).unwrap();
    let regex_pattern = Regex::new(r"(\w+) \((\d+)\)( -> )?((\w+)(, \w+)*)?").unwrap();
    let mut program_map: HashMap<String, (usize, Vec<String>)> = HashMap::new();
    for line in BufReader::new(file).lines() {
        let str = line.unwrap();
        let mut captured_groups = regex_pattern
            .captures(str.as_str())
            .unwrap()
            .iter()
            .collect::<Vec<_>>();
        let name = captured_groups[1].unwrap().as_str();
        let weight: usize = captured_groups[2].unwrap().as_str().parse().unwrap();
        let mut children: Vec<String> = Vec::new();
        if let Some(mtch) = captured_groups[4] {
            let mut ch = mtch
                .as_str()
                .split(", ")
                .map(|str| {
                    str.to_string()
                })
                .collect::<Vec<_>>();
            children.append(&mut ch);
        }
        program_map.insert(name.to_string(), (weight, children));
    }
    program_map
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_part1() {
        let path = "src/inputs/day7_test.txt";
        let program_map = parse_input(path);
        assert_eq!("tknk", find_bottom_program(program_map));
    }

    #[test]
    fn test_submission_part1() {
        let path = "src/inputs/day7.txt";
        let program_map = parse_input(path);
        assert_eq!("vgzejbd", find_bottom_program(program_map));
    }

    #[test]
    fn test_sample_part2() {
        let path = "src/inputs/day7_test.txt";
        let program_map = parse_input(path);
        let root = find_bottom_program(program_map);

    }
}
