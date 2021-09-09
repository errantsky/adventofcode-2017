use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

fn solve_p1(mut banks: Vec<usize>) -> (usize, usize) {
    let mut steps = 0;
    let len = banks.len();
    let mut seen_banks: HashMap<Vec<usize>, usize> = HashMap::new();
    while !seen_banks.contains_key(&banks) {
        seen_banks.insert(banks.clone(), steps);
        let (mut mid, mut mval) = banks
            .iter_mut()
            .enumerate()
            .max_by(|(i, n), (j, m)| {
                if n == m {
                    (-(*i as isize)).cmp(&-(*j as isize))
                } else {
                    n.cmp(m)
                }
            })
            .unwrap();
        let blocks = *mval;
        *mval = 0;
        for i in 0..blocks {
            mid += 1;
            banks[mid % len] += 1
        }
        steps += 1;
    }
    (steps, steps - seen_banks.get(&banks).unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample1() {
        let banks = vec![0, 2, 7, 0];
        // assert_eq!(solve_p1(banks), 5)
    }

    #[test]
    fn test_submission1() {
        let file = File::open("src/inputs/day6.txt").unwrap();
        let mut buffered = BufReader::new(file);
        let inp = buffered.lines().next().unwrap().unwrap();
        // TODO try filter_map
        let banks: Vec<usize> = inp
            .split(&"\t".to_string())
            .map(|s| s.parse().unwrap())
            .collect();
        let (a, b) = solve_p1(banks);
        println!("Part 1: {}, {}", a, b);
    }
}
