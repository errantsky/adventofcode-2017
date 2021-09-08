const GEN1_STARTER_SAMPLE: usize = 65;
const GEN2_STARTER_SAMPLE: usize = 8921;
const GEN1_STARTER: usize = 679;
const GEN2_STARTER: usize = 771;
const GEN1_FACTOR: usize = 16807;
const GEN2_FACTOR: usize = 48271;

pub fn part1() {
    let mut judge_count: usize = 0;
    let mut prev1 = GEN1_STARTER;
    let mut prev2 = GEN2_STARTER;
    for i in 0..(40 * 10_usize.pow(6)) {
        prev1 = (prev1 * GEN1_FACTOR).rem_euclid(2147483647);
        prev2 = (prev2 * GEN2_FACTOR).rem_euclid(2147483647);

        if format!("{:0>32b}", prev1)[16..] == format!("{:0>32b}", prev2)[16..] {
            judge_count += 1;
        }
    }
    print!("Part1: {}", judge_count);
}

pub fn part2() {
    let mut judge_count: usize = 0;
    let mut prev1 = GEN1_STARTER;
    let mut prev2 = GEN2_STARTER;
    for i in 0..(5 * 10_usize.pow(6)) {
        prev1 = gen_logic(4, prev1, GEN1_FACTOR);
        prev2 = gen_logic(8, prev2, GEN2_FACTOR);

        if format!("{:0>32b}", prev1)[16..] == format!("{:0>32b}", prev2)[16..] {
            judge_count += 1;
        }
    }
    print!("Part2: {}", judge_count);
}

fn gen_logic(multiple_factor: usize, starter: usize, factor: usize) -> usize {
    let mut prev = (starter * factor).rem_euclid(2147483647);
    while prev.rem_euclid(multiple_factor) != 0 {
        prev = (prev * factor).rem_euclid(2147483647);
    }
    return prev;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_15sample_part1() {
        let mut judge_count: usize = 0;
        let mut prev1 = GEN1_STARTER_SAMPLE;
        let mut prev2 = GEN2_STARTER_SAMPLE;
        for i in 0..5 {
            prev1 = (prev1 * GEN1_FACTOR).rem_euclid(2147483647);
            prev2 = (prev2 * GEN2_FACTOR).rem_euclid(2147483647);
            println!("{:0>32b}", prev1);
            println!("{:0>32b}", prev2);
            if format!("{:0>32b}", prev1)[16..] == format!("{:0>32b}", prev2)[16..] {
                judge_count += 1;
            }
        }
        assert_eq!(judge_count, 1);
    }
}
