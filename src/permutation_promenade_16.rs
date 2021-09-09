use std::collections::HashMap;

fn spin(shift_amount: usize, line: &mut Vec<&str>) {
    line.rotate_right(shift_amount);
}

fn exchange(prog1_pos: usize, prog2_pos: usize, line: &mut Vec<&str>) {
    line.swap(prog1_pos, prog2_pos);
}

fn partner(prog1: &str, prog2: &str, line: &mut Vec<&str>) {
    let mut prog1_pos = line.len();
    let mut prog2_pos = line.len();
    for (idx, p) in line.iter().enumerate() {
        if *p == prog1 {
            prog1_pos = idx;
        }
        if *p == prog2 {
            prog2_pos = idx;
        }
        if (prog1_pos != line.len()) & (prog2_pos != line.len()) {
            break;
        }
    }

    line.swap(prog1_pos, prog2_pos);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dueling_generators_15::part2;
    use std::fs::File;
    use std::io::{BufRead, BufReader};

    #[test]
    fn test_sample_part1() {
        let mut line: Vec<&str> = "a b c d e".split(" ").collect();
        let commands = "s1,x3/4,pe/b".split(",");
        for cmd in commands {
            match cmd.chars().next().unwrap() {
                's' => {
                    let rotation_val: usize = cmd[1..].parse().unwrap();
                    spin(rotation_val, &mut line);
                }
                'x' => {
                    let slash_pos = cmd.chars().position(|x| x == '/').unwrap();
                    let prog1_pos = cmd
                        .chars()
                        .skip(1)
                        .take(slash_pos - 1)
                        .collect::<String>()
                        .parse()
                        .unwrap();
                    let prog2_pos = cmd
                        .chars()
                        .skip(slash_pos + 1)
                        .collect::<String>()
                        .parse()
                        .unwrap();

                    exchange(prog1_pos, prog2_pos, &mut line);
                }
                'p' => {
                    let mut split = cmd[1..].split("/");
                    let prog1 = split.next().unwrap();
                    let prog2 = split.next().unwrap();
                    partner(prog1, prog2, &mut line);
                }
                _ => panic!("Bad input given: {}", cmd),
            }
        }
        let result: Vec<&str> = "b a e d c".split(" ").collect();
        assert_eq!(result, line);
    }

    #[test]
    fn test_submission_part1() {
        let mut line: Vec<&str> = "a b c d e f g h i j k l m n o p".split(" ").collect();
        let file = File::open("src/inputs/day16.txt").unwrap();
        let mut buffered = BufReader::new(file).lines();
        let l = buffered.next().unwrap().unwrap();
        let commands = l.split(",");
        for cmd in commands {
            match cmd.chars().next().unwrap() {
                's' => {
                    let rotation_val: usize = cmd[1..].parse().unwrap();
                    spin(rotation_val, &mut line);
                }
                'x' => {
                    let slash_pos = cmd.chars().position(|x| x == '/').unwrap();
                    let prog1_pos = cmd
                        .chars()
                        .skip(1)
                        .take(slash_pos - 1)
                        .collect::<String>()
                        .parse()
                        .unwrap();
                    let prog2_pos = cmd
                        .chars()
                        .skip(slash_pos + 1)
                        .collect::<String>()
                        .parse()
                        .unwrap();

                    exchange(prog1_pos, prog2_pos, &mut line);
                }
                'p' => {
                    let mut split = cmd[1..].split("/");
                    let prog1 = split.next().unwrap();
                    let prog2 = split.next().unwrap();
                    partner(prog1, prog2, &mut line);
                }
                _ => panic!("Bad input given: {}", cmd),
            }
        }

        println!("Part 1: {}", line.join(""));
    }

    #[test]
    fn test_part2_cycle() {}
}
