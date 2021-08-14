use std::fs::File;
use std::io::{BufReader, BufRead};

fn solve_part_1(mut list: Vec<i32>) -> usize {
    // ToDo proper way to check if two integers vars with different types

    let mut idx: i32 = 0;
    let mut steps: usize = 0;
    while idx < list.len() as i32 && idx >= 0 {
        let offset = list[idx as usize];
        list[idx as usize] += 1;
        idx += offset;
        steps += 1;
    }

    steps
}

fn solve_part_2(mut list: Vec<i32>) -> usize {
    // proper way to check if two integers vars with different types
    let mut idx: i32 = 0;
    let mut steps: usize = 0;
    while idx < list.len() as i32 && idx >= 0 {
        let offset = list[idx as usize];
        if offset >= 3 {
            list[idx as usize] -= 1;
        }
        else {
            list[idx as usize] += 1;
        }
        idx += offset;
        steps += 1;
    }

    steps
}


fn read_input() -> Vec<i32> {
    // ToDo the for loop did not work as a .map() called on .lines(). Why?
    let file = File::open("/Users/sep/CLionProjects/adventofcode-2017/src/inputs/day5.txt").unwrap();
    let buffered = BufReader::new(file);
    let mut list: Vec<i32> = Vec::new();
    for line in buffered.lines() {
        let num = line.unwrap().parse::<i32>().unwrap();
        list.push(num);
    }

    list
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample1() {
        let lines = vec![0, 3, 0, 1, -3];
        assert_eq!(solve_part_1(lines), 5);
    }

    #[test]
    fn test_sample2() {
        let lines = vec![0, 3, 0, 1, -3];
        assert_eq!(solve_part_2(lines), 10);
    }

    #[test]
    fn test_submission1() {
        let list = read_input();
        println!("Part 1: {}", solve_part_1(list));
    }

    #[test]
    fn test_submission2() {
        let list = read_input();
        println!("Part 2: {}", solve_part_2(list));
    }
}