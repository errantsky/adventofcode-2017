use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::{HashMap, HashSet};
use std::hash::Hash;


fn bfs(adjacency_map: &HashMap<u16, HashSet<u16>>, visited_nodes: &mut HashSet<u16>, start_node: u16) -> u16 {
    let mut stack: Vec<u16> = vec![start_node];
    let mut visited_node_count = 0;

    while !stack.is_empty() {
        let node = stack.pop().unwrap();
        if !visited_nodes.contains(&node) {
            visited_nodes.insert(node);
            visited_node_count += 1;

            for n in adjacency_map.get(&node).unwrap() {
                stack.push(n.clone());
            }
        }

    }

    return visited_node_count
}

fn bfs_alt(adjacency_map: HashMap<u16, HashSet<u16>>) -> u16 {
    let mut visited_node_count = 0;
    let mut visited_nodes: HashSet<u16> = HashSet::new();
    let mut num_groups = 0;

    while visited_node_count != adjacency_map.len() {
        let mut start_node = 0;
        for node in adjacency_map.keys() {
            if !visited_nodes.contains(node) {
                start_node = node.clone();
            }
        }

        let mut stack: Vec<u16> = vec![start_node];
        while !stack.is_empty() {
            let node = stack.pop().unwrap();
            if !visited_nodes.contains(&node) {
                visited_nodes.insert(node);
                visited_node_count += 1;
                for n in adjacency_map.get(&node).unwrap() {
                    stack.push(n.clone());
                }
            }
        }
        num_groups += 1;
    }


    return num_groups;
}


fn part1(adjacency_map: HashMap<u16, HashSet<u16>>) -> u16 {
    let mut vn: HashSet<u16> = HashSet::new();
    let cnt = bfs(&adjacency_map, &mut vn, 0);
    return cnt
}

fn part2(adjacency_map: HashMap<u16, HashSet<u16>>) -> u16 {
    return bfs_alt(adjacency_map)
}



fn read_input(path: &str) -> HashMap<u16, HashSet<u16>>{
    let mut adjacency_map: HashMap<u16, HashSet<u16>> = HashMap::new();
    let file = File::open(path).unwrap();
    let buffered = BufReader::new(file).lines();
    for l in buffered {
        let line = l.unwrap();
        let mut split_line: Vec<&str> = line.split(" <-> ").collect();
        let node: u16 = split_line[0].parse().unwrap();
        let neighbors: HashSet<u16> = split_line[1]
            .split(", ")
            .map(|x| x.parse::<u16>().unwrap())
            .collect();

        if adjacency_map.contains_key(&node) {
            adjacency_map.get(&node).unwrap().union(&neighbors);
        } else {
            adjacency_map.insert(node, neighbors);
        }
    }

    return adjacency_map
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_sample1() {
        let adj_map = read_input("/Users/sep/CLionProjects/adventofcode-2017/src/inputs/day12_test.txt");
        let count = part1(adj_map);

        assert_eq!(count, 6);
    }

    #[test]
    fn test_input_sample2() {

        let adj_map = read_input("/Users/sep/CLionProjects/adventofcode-2017/src/inputs/day12_test.txt");
        let count = part2(adj_map);

        assert_eq!(count, 2);
    }

    #[test]
    fn test_sub1_res() {
        let path = "/Users/sep/CLionProjects/adventofcode-2017/src/inputs/day12.txt";
        assert_eq!(288, part1(read_input(path)));

    }

    #[test]
    fn test_sub2_res() {
        let path = "/Users/sep/CLionProjects/adventofcode-2017/src/inputs/day12.txt";
        assert_eq!(211, part2(read_input(path)));

    }

    #[test]
    fn test_submission_part1() {
        let path = "/Users/sep/CLionProjects/adventofcode-2017/src/inputs/day12.txt";
        println!("Part 1: {}", part1(read_input(path)));
    }

    #[test]
    fn test_submission_part2() {
        let path = "/Users/sep/CLionProjects/adventofcode-2017/src/inputs/day12.txt";
        println!("Part 1: {}", part2(read_input(path)));
    }
}
