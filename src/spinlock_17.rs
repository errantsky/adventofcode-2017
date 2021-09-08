fn spinlock_tick(current_pos: usize, step: usize, buffer: &mut Vec<usize>) -> usize {
    let next_val = buffer[current_pos] + 1;
    let next_pos = (current_pos + step).rem_euclid(buffer.len()) + 1;
    buffer.insert(next_pos, next_val);
    return next_pos;
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use std::hash::Hash;

    #[test]
    fn test_sample_part1() {
        let step: usize = 3;
        let mut b: Vec<usize> = vec![0];
        let mut pos = spinlock_tick(0, step, &mut b);
        assert_eq!(b, vec![0, 1]);
        assert_eq!(pos, 1);
        pos = spinlock_tick(pos, step, &mut b);
        assert_eq!(b, vec![0, 2, 1]);
        assert_eq!(pos, 1);

        for i in 0..(2017 - 2) {
            pos = spinlock_tick(pos, step, &mut b);
        }
        println!("b[pos]={}, b[pos + 1]={}", b[pos], b[pos + 1]);
        assert_eq!(b[pos], 2017);
        assert_eq!(b[pos + 1], 638)
    }
    #[test]
    fn test_submission_part1() {
        let step = 363;
        let mut b = vec![0];
        let mut pos = 0;
        for i in 0..2017 {
            pos = spinlock_tick(pos, step, &mut b);
        }
        println!("{}, {}", b[pos], b[pos + 1]);
        // assert_eq!(b[pos], 2017);
    }

    #[test]
    fn test_submission_part2() {
        // I don't need to keep the whole buffer, as I only need b[1]
        // So, I keep running the spinlock, only to check if b[1] is changing
        // If it is, I will update the `first` variable, which will hold the result
        let step = 363;
        let mut pos = 0;
        let mut first = 0;
        for i in 0..50000000 {
            let next_val = i + 1;
            let next_pos = (pos + step) % (i + 1) + 1;
            if next_pos == 1 {
                first = next_val;
            }
            pos = next_pos;
        }
        println!("b[1]={}", first);
    }

    #[test]
    fn test_cycling_part2() {
        // My first guess was to see if the there is a pattern
        // to how many steps it takes for b[1] to change
        // but it did not go anywhere
        let step = 363;
        let mut b = vec![0];
        let mut pos = 0;
        let mut index_map: HashMap<usize, usize> = HashMap::new();
        let mut prev = 1;
        for i in 0..50000000 {
            pos = spinlock_tick(pos, step, &mut b);
            // println!("Iteration: {}, After 0: {}", i, b[1]);
            if prev != b[1] {
                let mut ind = *(index_map.get(&prev).unwrap());
                println!("After zero was {} for {} Iterations", prev, i - ind);
            }
            if !index_map.contains_key(&b[1]) {
                index_map.insert(b[1], i);
            }

            prev = b[1];
        }
    }
}
