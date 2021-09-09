use std::collections::HashMap;
use std::fs::File;
use std::hash::Hash;
use std::io::{BufRead, BufReader};

fn run_program(instruction_vec: Vec<Instruction>) -> isize {
    let mut line_cursor: isize = 0;
    let mut last_played = 0;
    let mut register_map: HashMap<char, isize> = HashMap::new();
    while line_cursor >= 0 && line_cursor < instruction_vec.len() as isize {
        let mut lc = line_cursor as usize;
        let instruction = &instruction_vec[line_cursor as usize];
        let target_register = if let Field::Register(c) = instruction.left_field {
            c
        } else {
            'z'
        };
        // Setting the default register value to zero
        // TODO: A better way for a HashMap to have default values
        if !register_map.contains_key(&target_register) {
            register_map.insert(target_register, 0);
        }

        let val = match instruction.right_field {
            Some(Field::Register(register_name)) => {
                Some(*register_map.get(&register_name).unwrap())
            }
            Some(Field::Data(x)) => Some(x),
            None => None,
        };
        match instruction.op.as_str() {
            "snd" => {
                last_played = *register_map.get(&target_register).unwrap();
                line_cursor += 1;
            }
            "set" => {
                register_map.insert(target_register, val.unwrap());
                line_cursor += 1;
            }
            "add" => {
                *register_map.get_mut(&target_register).unwrap() += val.unwrap();
                line_cursor += 1;
            }
            "mul" => {
                *register_map.get_mut(&target_register).unwrap() *= val.unwrap();
                line_cursor += 1;
            }
            "mod" => {
                *register_map.get_mut(&target_register).unwrap() %= val.unwrap();
                line_cursor += 1;
            }
            "rcv" => {
                if *register_map.get(&target_register).unwrap() != 0 {
                    println!("last freq: {}", last_played);
                    break;
                }
                line_cursor += 1;
            }
            "jgz" => {
                if *register_map.get(&target_register).unwrap() > 0 {
                    line_cursor += val.unwrap();
                } else {
                    line_cursor += 1;
                }
            }
            _ => panic!("Bad input."),
        }
    }

    return last_played;
}

#[derive(Debug)]
struct Instruction {
    op: String,
    left_field: Field,
    right_field: Option<Field>,
}

#[derive(Debug)]
enum Field {
    Register(char),
    Data(isize),
}

fn parse_input(path: &str) -> Vec<Instruction> {
    let file = File::open(path).unwrap();
    let buffered = BufReader::new(file).lines();
    let mut instruction_vec: Vec<Instruction> = Vec::new();
    for line in buffered {
        let cmd_str = line.unwrap();
        let mut cmd_split = cmd_str.split(" ");
        let op = cmd_split.next().unwrap();
        let instruction = match op {
            "snd" | "rcv" => Instruction {
                op: op.to_string(),
                left_field: Field::Register(cmd_split.next().unwrap().chars().next().unwrap()),
                right_field: None,
            },
            "add" | "set" | "mul" | "mod" | "jgz" => Instruction {
                op: op.to_string(),
                left_field: Field::Register(cmd_split.next().unwrap().chars().next().unwrap()),
                right_field: Some({
                    let val = cmd_split.next().unwrap();
                    match val.parse::<isize>() {
                        Ok(data) => Field::Data(data),
                        Err(_) => Field::Register(val.chars().next().unwrap()),
                    }
                }),
            },
            _ => panic!("Bad op: {}", op),
        };
        instruction_vec.push(instruction);
    }
    instruction_vec
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::{BufRead, BufReader, Split};

    #[test]
    fn test_parse_input() {
        let path = "src/inputs/day18_test.txt";
        let instruction_vec = parse_input(path);
        println!("Instructions: {:?}", instruction_vec);
    }
    #[test]
    fn test_sample_part1() {
        let path = "src/inputs/day18_test.txt";
        let instruction_vec = parse_input(path);
        let last_sound = run_program(instruction_vec);
        assert_eq!(last_sound, 4)
    }

    #[test]
    fn test_submission_part1() {
        let path = "src/inputs/day18.txt";
        let instruction_vec = parse_input(path);
        let last_sound = run_program(instruction_vec);
    }
}
