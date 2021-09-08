use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};

fn part1(lines: Lines<BufReader<File>>) -> (i32, i32) {
    let mut registers: HashMap<String, i32> = HashMap::new();
    let mut max_val = i32::MIN;
    for l in lines {
        let line = l.unwrap();
        let tokens = line.split_whitespace().collect::<Vec<&str>>();
        let command_register = tokens[0].to_owned();
        let command = tokens[1].to_owned();
        let command_val = tokens[2].parse::<i32>().unwrap();
        let condition_register = tokens[4].to_owned();
        let condition_operation = tokens[5].to_owned();
        let condition_val = tokens[6].parse::<i32>().unwrap();

        if !registers.contains_key(&*condition_register) {
            registers.insert(condition_register.clone(), 0);
        }
        match condition_operation.as_str() {
            ">" => {
                if !(registers[&condition_register] > condition_val) {
                    continue;
                }
            }
            "<" => {
                if !(registers[&condition_register] < condition_val) {
                    continue;
                }
            }
            "==" => {
                if !(registers[&condition_register] == condition_val) {
                    continue;
                }
            }
            ">=" => {
                if !(registers[&condition_register] >= condition_val) {
                    continue;
                }
            }
            "<=" => {
                if !(registers[&condition_register] <= condition_val) {
                    continue;
                }
            }
            "!=" => {
                if !(registers[&condition_register] != condition_val) {
                    continue;
                }
            }
            _ => panic!("Wrong condition operation."),
        }
        match command.as_str() {
            "inc" => *registers.entry(command_register.clone()).or_default() += command_val,
            "dec" => *registers.entry(command_register.clone()).or_default() -= command_val,
            _ => panic!("Wrong command."),
        }
        if *registers.values().max().unwrap() > max_val {
            max_val = *registers.values().max().unwrap();
        }
    }
    (*registers.values().max().unwrap(), max_val)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_sample1() {
        let file =
            File::open("/Users/sep/CLionProjects/adventofcode-2017/src/inputs/day8_test.txt")
                .unwrap();
        let buffered = BufReader::new(file).lines();
        assert_eq!(part1(buffered).0, 1);
    }

    #[test]
    fn test_submission() {
        let file =
            File::open("/Users/sep/CLionProjects/adventofcode-2017/src/inputs/day8.txt").unwrap();
        let buffered = BufReader::new(file).lines();
        let res = part1(buffered);
        println!("Part1: {}\nPart2: {}", res.0, res.1);
    }
}
