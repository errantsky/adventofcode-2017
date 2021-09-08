use std::fs::File;
use std::io::{BufRead, BufReader};

struct Component {}

fn parse_input(path: &str) {
    let file = File::open(path).expect("Failed openning the file.");
    let buffered = BufReader::new(file).lines();
    let component_vec: Vec<Component> = Vec::new();

    for l in buffered {
        let line = l.expect("Reading the line caused and error.");
        let split = line.split("/");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_part1() {}
}
