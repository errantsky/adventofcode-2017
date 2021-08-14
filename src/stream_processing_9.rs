use std::io::{BufReader, BufRead};
use std::fs::File;

fn solve_part1(stream: String) -> u32 {
    let mut letters = stream.chars().collect::<Vec<char>>();
    let mut stack: Vec<char> = Vec::new();
    let mut score = 0;
    let mut res = 0;
    let mut skip_flag = false;
    let mut garbage_flag = false;
    let mut garbage_count = 0;
    for idx in 0..letters.len() {
        if skip_flag {
            skip_flag = false;
            continue;
        }
        if garbage_flag && (letters[idx] != '>' && letters[idx] != '!') {
            if letters[idx-1] != '!' {
                garbage_count += 1;
            }
            continue;
        }
        match letters[idx] {
            '{' => {
                score += 1;
                stack.push(letters[idx]);
            },
            '}' => {
                res += score;
                score -= 1;
                stack.pop();
            },
            '<' => {
                garbage_flag = true;
                stack.push(letters[idx]);
            },
            '>' => {
                garbage_flag = false;
                stack.pop();
            },
            '!' => {
                skip_flag = true;
            },
            _ => {

            }
        }
    }

    println!("Garbage count: {}", garbage_count);
    res
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1_sample1() {
        assert_eq!(solve_part1("{}".to_string()), 1);
    }

    #[test]
    fn test1_sample2() {
        assert_eq!(solve_part1("{{},{}}".to_string()), 5);
    }

    #[test]
    fn test1_sample3() {
        assert_eq!(solve_part1("{{{}}}".to_string()), 6);
    }

    #[test]
    fn test1_sample4() {
        assert_eq!(solve_part1("{{{},{},{{}}}}".to_string()), 16);
    }

    #[test]
    fn test1_sample5() {
        assert_eq!(solve_part1("{<a>,<a>,<a>,<a>}".to_string()), 1);
    }

    #[test]
    fn test1_sample6() {
        assert_eq!(solve_part1("{{<a!>},{<a!>},{<a!>},{<ab>}}".to_string()), 3);
    }

    #[test]
    fn test1_sample7() {
        assert_eq!(solve_part1("{{<!!>},{<!!>},{<!!>},{<!!>}}".to_string()), 9);
    }

    #[test]
    fn test1_sample8() {
        assert_eq!(solve_part1("{{<ab>},{<ab>},{<ab>},{<ab>}}".to_string()), 9);
    }

    #[test]
    fn garbage_test1() {
        solve_part1("<random characters>".to_string());
        println!("Shoul've been 17")
    }

    #[test]
    fn garbage_test2() {
        solve_part1("<<<<>".to_string());
        println!("3");
    }

    #[test]
    fn garbage_test3() {
        solve_part1("<{!>}>".to_string());
        println!("2");
    }

    #[test]
    fn garbage_test4() {
        solve_part1("<!!>".to_string());
        println!("0");
    }
    #[test]
    fn garbage_test5() {
        solve_part1("<!!!>>".to_string());
        println!("0");
    }
    #[test]
    fn garbage_test6() {
        solve_part1("<{o\"i!a,<{i<a>".to_string());
        println!("10");
    }
    #[test]
    fn test1_submission() {
        let file = File::open("/Users/sep/CLionProjects/adventofcode-2017/src/inputs/day9.txt").unwrap();
        let buffered = BufReader::new(file);
        let input = buffered.lines().next().unwrap().unwrap();
        println!("Part 1: {}", solve_part1(input));
    }
}