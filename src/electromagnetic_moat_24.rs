use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
// Well, I failed to write a good solution and ended up checking the AoC reddit forum
// This is the solution I chose to study, to better understand Rust's functional style
// Solution courtesy of https://old.reddit.com/r/adventofcode/comments/7lte5z/2017_day_24_solutions/drp30si/

fn extend(
    pieces: &[(usize, usize)],
    used: &HashSet<usize>,
    last: usize,
    p2: bool,
) -> (usize, usize) {
    if pieces.len() == used.len() {
        return (0, 0);
    }
    let mut u = used.clone();
    pieces
        .iter()
        .enumerate()
        .filter(|&(i, p)| (p.0 == last || p.1 == last) && !used.contains(&i))
        .map(|(i, p)| {
            u.insert(i);
            let (ll, ss) = extend(pieces, &u, p.0 + p.1 - last, p2);
            u.remove(&i);
            (ll + p2 as usize, ss + p.0 + p.1)
        })
        .max() // tuples have an order, so max works out of the box
        .unwrap_or((0, 0))
}

pub fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_part1() {
        // let pieces = File::open("src/inputs/day24.txt").unwrap();
        // let pieces = BufReader::new(pieces).lines()
        //     .map(|l| l.split('/'))
        //     .map(|mut ws| {
        //         (
        //             ws.next().unwrap().parse::<usize>().unwrap(),
        //             ws.next().unwrap().parse::<usize>().unwrap(),
        //         )
        //     })
        //     .collect::<Vec<_>>();
        // println!("P1: {}", extend(&pieces, &HashSet::new(), 0, false).1);
        // println!("P2: {}", extend(&pieces, &HashSet::new(), 0, true).1);
    }
}
