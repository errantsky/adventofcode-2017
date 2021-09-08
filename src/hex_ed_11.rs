// use std::fs::File;
// use std::io::{BufReader, BufRead};
// use std::collections::HashMap;
//
// fn part1(directions: String) -> u32 {
//     let mut x_axis = 0;
//     let mut y_axis = 0;
//     let dir_moves = vec![
//         ("n", (0, 1)),
//         ("s", (0, -1)),
//         ("ne", (1, 1)),
//         ("nw", (-1, 1)),
//         ("se", (1, -1)),
//         ("sw", (-1, -1)),
//     ];
//     let mut dir_map: HashMap<String, (i32, i32)> = HashMap::new();
//     dir_moves.map(|(dir, diff)| dir_map.insert(dir.to_string(), diff));
//     directions.split(',').map(|direction| {
//         let (dx, dy) = dir_map[direction.to_string()];
//         x_axis += dx;
//         y_axis += dy;
//     });
//     return x_axis + y_axis
// }
//
// fn knapsack(destination: (i32, i32), weights: Vec<(String, (i32, i32))>) {
//     let mut memo: HashMap<(i32, i32), f32> = HashMap::new();
//     memo.insert((0, 0), 0.);
//     for (_, (dx, dy)) in weights {
//         memo.insert((dx, dy), 1.);
//     }
//     for i in 1..destination.0 {
//         for j in 1..destination.1 {
//             let mut temp_min = f32::MAX;
//             for k in 0..weights.len() {
//                 let (name, (x, y)) = &weights[k];
//                 let res = dist((0, 0), (i + x, j + y));
//                 if res < temp_min {
//                     temp_min = res;
//                     let nk = name;
//                 }
//             }
//
//
//
//         }
//     }
//
// }
//
// fn dist(a: (i32, i32), b: (i32, i32)) -> f32 {
//     f32::sqrt((a.0 - b.0).pow(2) + (a.1 - b.1).pow(2) as f32)
// }
// #[cfg(test)]
// mod tests {
//     use super::*;
//     #[test]
//     fn test1_1() {
//         assert_eq!(part1(String::from("ne,ne,ne")), 3);
//     }
//     fn test1_2() {
//         assert_eq!(part1(String::from("ne,ne,sw,sw")), 0);
//     }
//     fn test1_3() {
//         assert_eq!(part1(String::from("ne,ne,s,s")), 2);
//     }
//     fn test1_4() {
//         assert_eq!(part1(String::from("se,sw,se,sw,sw")), 3);
//     }
//
//     fn submission1() {
//         let file = File::open("/Users/sep/CLionProjects/adventofcode-2017/src/inputs/day11.txt").unwrap();
//         let buffered = BufReader::new(file);
//         let input = buffered.lines().next().unwrap().unwrap();
//         println!("Part1: {}", part1(input))
//     }
// }
