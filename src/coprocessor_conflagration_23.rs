use std::collections::HashMap;
use std::fs::File;
use std::hash::Hash;
use std::io::{BufRead, BufReader};

fn run_program(instruction_vec: Vec<Instruction>) -> isize {
    let mut mul_count = 0;
    let mut line_cursor: isize = 0;
    let mut register_map: HashMap<char, isize> = HashMap::new();

    // Part 2 modifications
    register_map.insert('a', 1);

    while line_cursor >= 0 && line_cursor < instruction_vec.len() as isize {
        let mut lc = line_cursor as usize;
        let instruction = &instruction_vec[line_cursor as usize];
        let target_register = if let Field::Register(c) = instruction.left_field {
            // Setting the default register value to zero
            if !register_map.contains_key(&c) {
                register_map.insert(c, 0);
            }
            c
        } else {
            'z'
        };

        let val = match instruction.right_field {
            Field::Register(register_name) => *register_map.get(&register_name).unwrap(),
            Field::Data(x) => x,
        };
        match instruction.op.as_str() {
            "set" => {
                register_map.insert(target_register, val);
                line_cursor += 1;
            }
            "add" => {
                *register_map.get_mut(&target_register).unwrap() += val;
                line_cursor += 1;
            }
            "sub" => {
                *register_map.get_mut(&target_register).unwrap() -= val;
                line_cursor += 1;
            }
            "mul" => {
                *register_map.get_mut(&target_register).unwrap() *= val;
                line_cursor += 1;
                mul_count += 1;
            }
            "mod" => {
                *register_map.get_mut(&target_register).unwrap() %= val;
                line_cursor += 1;
            }
            "jnz" => match instruction.left_field {
                Field::Register(c) => {
                    if *register_map.get(&target_register).unwrap() != 0 {
                        line_cursor += val;
                    } else {
                        line_cursor += 1;
                    }
                }
                Field::Data(data) => {
                    if data != 0 {
                        line_cursor += val;
                    } else {
                        line_cursor += 1;
                    }
                }
            },
            _ => panic!("Bad input."),
        }
    }

    println!("Part 2: Register h: {}", register_map.get(&'h').unwrap());
    return mul_count;
}

#[derive(Debug)]
struct Instruction {
    op: String,
    left_field: Field,
    right_field: Field,
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
            "add" | "sub" | "set" | "mul" | "mod" | "jnz" => Instruction {
                op: op.to_string(),
                left_field: {
                    let val = cmd_split.next().unwrap();
                    match val.parse::<isize>() {
                        Ok(data) => Field::Data(data),
                        Err(_) => Field::Register(val.chars().next().unwrap()),
                    }
                },
                right_field: {
                    let val = cmd_split.next().unwrap();
                    match val.parse::<isize>() {
                        Ok(data) => Field::Data(data),
                        Err(_) => Field::Register(val.chars().next().unwrap()),
                    }
                },
            },
            _ => panic!("Bad op: {}", op),
        };
        instruction_vec.push(instruction);
    }
    instruction_vec
}

pub fn test_submission_part1() {
    let path = "src/inputs/day23.txt";
    let instruction_vec = parse_input(path);
    let mul_count = run_program(instruction_vec);
    println!("Part1: {}", mul_count);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::{BufRead, BufReader, Split};

    #[test]
    fn test_parse_input() {
        let path = "src/inputs/day23.txt";
        let instruction_vec = parse_input(path);
        println!("Instructions: {:?}", instruction_vec);
    }

    #[test]
    fn test_submission_part1() {
        let path = "src/inputs/day23.txt";
        let instruction_vec = parse_input(path);
        let mul_count = run_program(instruction_vec);
        println!("Part1: {}", mul_count);
    }
}
