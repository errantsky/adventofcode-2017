use std::fs::File;
use std::io::{BufRead, BufReader};

fn find_checksum() -> u32 {
    let path = "day2.txt";
    let file = File::open(path).unwrap();
    let buffered = BufReader::new(file);
    buffered
        .lines()
        .map(|line| {
            let list: Vec<_> = line
                .unwrap()
                .split_whitespace()
                .map(|num_str| num_str.parse::<u32>().unwrap())
                .collect();

            list.iter().max().unwrap() - list.iter().min().unwrap()
        })
        .sum()
}

fn find_checksum2() -> u32 {
    let path = "day2.txt";
    let file = File::open(path).unwrap();
    let buffered = BufReader::new(file);
    buffered
        .lines()
        .map(|line| {
            let list: Vec<u32> = line
                .unwrap()
                .split_whitespace()
                .map(|num_str| num_str.parse::<u32>().unwrap())
                .collect();

            let mut res: u32 = 0;
            for i in 0..list.len() {
                for j in 0..list.len() {
                    if i != j && list[i] % list[j] == 0 {
                        res = list[i] / list[j]
                    }
                }
            }
            res
        })
        .sum()
}
#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn sample1_test1() {}

    #[test]
    fn submission1_test() {
        println!("Part 1 answer: {}", find_checksum());
    }

    #[test]
    fn sample2_test1() {}

    #[test]
    fn submission2_test() {
        println!("Part 2 answer: {}", find_checksum2());
    }
}
