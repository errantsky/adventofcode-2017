use std::collections::HashSet;


fn is_valid(passphrase: String) -> bool {
    let mut words_set = HashSet::new();
    for word in passphrase.split_whitespace() {
        if words_set.contains(word) {
            return false;
        }
        words_set.insert(word);
    }

    true
}

fn is_valid_advanced(passphrase: String) -> bool {
    if !is_valid(passphrase.clone()) {
        return false;
    }

    let mut word_set = HashSet::new();
    for word in passphrase.split_whitespace() {
        let mut char_set = word.chars().collect::<Vec<char>>();
        char_set.sort_unstable();
        if word_set.contains(&*char_set) {
            return false;
        }
        word_set.insert(char_set);
    }

    true
}

#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    use super::*;

    #[test]
    fn p1test1() {
        assert!(is_valid("aa bb cc dd ee".to_string()))
    }

    #[test]
    fn p1test2() {
        assert!(!is_valid("aa bb cc dd aa".to_string()))
    }

    #[test]
    fn p1test3() {
        assert!(is_valid("aa bb cc dd aaa".to_string()))
    }

    #[test]
    fn submission1() {
        // ToDo: try functional sum, per line reading, and all lines
        let mut count = 0;
        let f =
            File::open("/Users/sep/CLionProjects/adventofcode-2017/src/inputs/day4.txt").unwrap();
        let buffered = BufReader::new(f);
        for line in buffered.lines() {
            let mut l = line.unwrap();
            if is_valid(l) {
                count += 1;
            }
        }
        println!("Part 1: {}", count)
    }

    #[test]
    fn p2test1() {
        assert!(is_valid_advanced("abcde fghij".to_string()))
    }

    #[test]
    fn p2test2() {
        assert!(!is_valid_advanced("abcde xyz ecdab".to_string()))
    }

    #[test]
    fn p2test3() {
        assert!(is_valid_advanced("a ab abc abd abf abj".to_string()))
    }

    #[test]
    fn p2test4() {
        assert!(is_valid_advanced("iiii oiii ooii oooi oooo".to_string()))
    }

    #[test]
    fn p2test5() {
        assert!(!is_valid_advanced("oiii ioii iioi iiio".to_string()))
    }

    #[test]
    fn hash_set_test() {
        let mut hs = HashSet::new();
        let mut char_set = "adc".chars().collect::<Vec<char>>();
        char_set.sort();
        hs.insert(char_set);
        println!("{:?}", hs);
    }

    #[test]
    fn submission2() {
        let mut count = 0;
        let f =
            File::open("/Users/sep/CLionProjects/adventofcode-2017/src/inputs/day4.txt").unwrap();
        let buffered = BufReader::new(f);
        for line in buffered.lines() {
            let mut l = line.unwrap();
            if is_valid_advanced(l) {
                count += 1;
            }
        }
        println!("Part 2: {}", count)
    }
}
