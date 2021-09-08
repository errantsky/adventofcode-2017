// use std::collections::{HashMap, HashSet};
// fn part_1(programs_map: HashMap<String, Vec<String>>) -> String {
//     // ToDo: Proper linking to reduce running time to O(log(n))
//     let mut temp_prog = programs_map.keys().next().unwrap();
//     let mut found_parent = true;
//     while found_parent {
//         found_parent = false;
//         for (k, v) in programs_map.iter() {
//             if v.contains(temp_prog) {
//                 temp_prog = k;
//                 found_parent = true;
//             }
//         }
//     }
//     temp_prog.clone()
// }
//
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
//
// #[cfg(test)]
// mod tests {
//     use std::fs::File;
//     use std::io::{BufReader, BufRead};
//
//     use super::*;
//
//     fn parse_input(path: &str) -> (HashMap<String, Vec<String>>, HashMap<String, u32>){
//         let mut program_map: HashMap<String, Vec<String>> = HashMap::new();
//         let mut weight_map: HashMap<String, u32> = HashMap::new();
//         let file = File::open(path).unwrap();
//         let buffered = BufReader::new(file);
//         for l in buffered.lines() {
//             let line = l.unwrap();
//             let name = line.split_whitespace().next().unwrap().to_string();
//             let left  = line.find('(').unwrap();
//             let right = line.find(')').unwrap();
//             let weight = &line[left+1..right].parse::<u32>().unwrap();
//             weight_map.insert(name.clone(), *weight);
//             let right_side = line.split(" -> ").skip(1).next();
//             if let Some(list) = right_side {
//                 let programs_above = list.split(", ").map(|c| c.to_string()).collect::<Vec<String>>();
//                 program_map.insert(name, programs_above);
//             }
//         }
//         (program_map, weight_map)
//     }
//
//     #[test]
//     fn test_sample1_part1() {
//         let (pmap, _) = parse_input("/Users/sep/CLionProjects/adventofcode-2017/src/inputs/day7_test.txt");
//
//         assert_eq!(part_1(pmap), "tknk");
//     }
//
//     #[test]
//     fn test_sample1_part2() {
//         let (pmap, wmap) = parse_input("/Users/sep/CLionProjects/adventofcode-2017/src/inputs/day7_test.txt");
//         let root = part_1(pmap.clone());
//         assert_eq!(part2(root, pmap, wmap), 60);
//     }
//
//     #[test]
//     fn test_submission_part1() {
//         let (pmap, _) = parse_input("/Users/sep/CLionProjects/adventofcode-2017/src/inputs/day7.txt");
//         println!("Part 1: {}", part_1(pmap));
//     }
// }
