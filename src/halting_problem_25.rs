use std::collections::VecDeque;

fn tm1_sample(count: isize) -> usize {
    let mut tape: VecDeque<bool> = VecDeque::new();
    tape.push_front(false);
    let mut cursor = 0;
    let mut current_state = 'A';

    for i in 0..count {
        let current_value = tape[cursor];
        match current_state {
            'A' => {
                if !current_value {
                    tape[cursor] = true;
                    if cursor == tape.len() - 1 {
                        tape.push_back(false);
                    }
                    cursor += 1;
                } else {
                    tape[cursor] = false;
                    if cursor == 0 {
                        tape.push_front(false);
                    } else {
                        cursor -= 1;
                    }
                }
                current_state = 'B';
            }
            'B' => {
                if !current_value {
                    tape[cursor] = true;
                    if cursor == 0 {
                        tape.push_front(false);
                    } else {
                        cursor -= 1;
                    }
                } else {
                    tape[cursor] = true;
                    if cursor == tape.len() - 1 {
                        tape.push_back(false);
                    }
                    cursor += 1;
                }
                current_state = 'A';
            }
            _ => panic!("Bad state"),
        }
    }

    tape.iter().filter(|x| **x == true).count()
}

fn tm1_submission(count: isize) -> usize {
    let mut tape: VecDeque<bool> = VecDeque::new();
    tape.push_front(false);
    let mut cursor = 0;
    let mut current_state = 'A';

    for i in 0..count {
        let current_value = tape[cursor];
        match current_state {
            'A' => {
                if !current_value {
                    tape[cursor] = true;
                    if cursor == tape.len() - 1 {
                        tape.push_back(false);
                    }
                    cursor += 1;
                } else {
                    tape[cursor] = false;
                    if cursor == 0 {
                        tape.push_front(false);
                    } else {
                        cursor -= 1;
                    }
                }
                current_state = 'B';
            }
            'B' => {
                if current_value {
                    tape[cursor] = true;
                    if cursor == 0 {
                        tape.push_front(false);
                    } else {
                        cursor -= 1;
                    }
                    current_state = 'B';
                } else {
                    tape[cursor] = false;
                    if cursor == tape.len() - 1 {
                        tape.push_back(false);
                    }
                    cursor += 1;
                    current_state = 'C';
                }
            }
            'C' => {
                if !current_value {
                    tape[cursor] = true;
                    if cursor == tape.len() - 1 {
                        tape.push_back(false);
                    }
                    cursor += 1;
                    current_state = 'D';
                } else {
                    tape[cursor] = false;
                    if cursor == 0 {
                        tape.push_front(false);
                    } else {
                        cursor -= 1;
                    }
                    current_state = 'A';
                }
            }
            'D' => {
                if !current_value {
                    tape[cursor] = true;
                    if cursor == 0 {
                        tape.push_front(false);
                    } else {
                        cursor -= 1;
                    }
                    current_state = 'E';
                } else {
                    tape[cursor] = true;
                    if cursor == 0 {
                        tape.push_front(false);
                    } else {
                        cursor -= 1;
                    }
                    current_state = 'F';
                }
            }
            'E' => {
                if !current_value {
                    tape[cursor] = true;
                    if cursor == 0 {
                        tape.push_front(false);
                    } else {
                        cursor -= 1;
                    }
                    current_state = 'A';
                } else {
                    tape[cursor] = false;
                    if cursor == 0 {
                        tape.push_front(false);
                    } else {
                        cursor -= 1;
                    }
                    current_state = 'D';
                }
            }
            'F' => {
                if !current_value {
                    tape[cursor] = true;
                    if cursor == tape.len() - 1 {
                        tape.push_back(false);
                    }
                    cursor += 1;
                    current_state = 'A';
                } else {
                    tape[cursor] = true;
                    if cursor == 0 {
                        tape.push_front(false);
                    } else {
                        cursor -= 1;
                    }
                    current_state = 'E';
                }
            }
            _ => panic!("Bad state"),
        }
    }

    tape.iter().filter(|x| **x == true).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_part1() {
        let one_count = tm1_sample(6);
        assert_eq!(3, one_count);
    }

    #[test]
    fn test_submission_part1() {
        let one_count = tm1_submission(12629077);
        println!("Part 1: {}", one_count);
    }
}
