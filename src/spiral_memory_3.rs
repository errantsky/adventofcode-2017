/*
   First square has 8 = 2 * 3 + 2 * (3 - 2) = 2 * (3 + (3 - 2)) cells
   Second square has 16 = 2 * 5 + 2 * (5 - 2) = 2 * (5 + (5 - 2)) cells
   ith square has 2 * ((2i + 1) + (2i + 1) - 2) cells
                   = 2 * 4i = 8i
   sum of first k squares 8*1 + 8*2 + ... + 8*(k-1) + 8*k = 8 * (n*(n+1))/2
                           = 4 * n * (n+1)
                          x = 4n^2 + 4n
                          n^2 + n - x/4
                          1 + x
                          (- 1 + sqrt(1+x)) / 2
   First square has 3 cell long sides
   Second square has 5 cell long sides
*/

// fn spiral_gen(idx: usize) -> usize {
//     let layer = ((-1. + (1. + idx as f64).sqrt()) / 2.).ceil() as usize;
//     let side = 2 * layer + 1;
//
//     let mut grid: Vec<Vec<usize>> = vec![vec![0; side]; side];
//
//     let mid = side / 2;
//
//     grid[mid][mid] = 1;
//     let mut x = mid;
//     let mut y = mid + 1;
//     let mut current_layer = 1;
//     let i = 1;
//     let directions = [(0, -1), (-1, 0), (0, 1), (1, 0)];
//
//     let current: usize = 1;
//     while current <= idx {}
//
//     return 1;
// }

// fn neighbor_sum(x: usize, y: usize, list: Vec<Vec<usize>>) -> usize {
//     let x = x as i32;
//     let y = y as i32;
//     let neighbor_xy = [(0, 1), (-1, 1), (-1, 0), (-1, -1), (0, -1), (1, -1), (1, 0), (1, 1)];
//     let mut sum = 0;
//     for (dx, dy) in neighbor_xy {
//         if x + dx >= 0 && x + dx < list.len() as i32 && y + dy >= 0 && y + dy < list.len() as i32 {
//             list[x+dx][y+dy].unw
//         }
//     }
//     return
// }

fn spiral_idx_to_xy(idx: usize) -> usize {
    let layer = ((-1. + (1. + idx as f64).sqrt()) / 2.).ceil() as usize;

    let side = 2 * layer + 1;
    let diff = idx - (4 * layer * (layer - 1) + 1);
    let diff_range = ((-(layer as isize))..=layer as isize).collect::<Vec<isize>>();
    let dist2_i = diff.rem_euclid(side - 1);
    let dist2 = diff_range[dist2_i].abs();

    (layer + dist2 as usize) as usize
}

/*
https://old.reddit.com/r/adventofcode/comments/7h7ufl/2017_day_3_solutions/dqplga3/

 */
type I = i32;
type U = usize;

fn layer_index(p: I) -> (I, I, I, I) {
    if p == 1 {
        return (0, 0, 0, 0);
    }
    fn last_on_layer(i: I) -> I {
        let x = 2 * i + 1;
        x * x
    }
    let layer = (0..).find(|&i| p <= last_on_layer(i)).unwrap();
    let start = last_on_layer(layer - 1) + 1;
    let zero = start + layer - 1;
    let pos_rel_to_quadrant_start = mod_pos(p - zero, 2 * layer);
    let dist_from_edge_center = if pos_rel_to_quadrant_start > layer {
        2 * layer - pos_rel_to_quadrant_start
    } else {
        pos_rel_to_quadrant_start
    };
    let layer_len = 2 * layer * 4;
    let quadrant = mod_pos((p - zero), layer_len) / (2 * layer);
    (
        layer,
        dist_from_edge_center,
        quadrant,
        pos_rel_to_quadrant_start,
    )
}

pub fn mod_pos(a: I, b: I) -> I {
    (a % b + b) % b
}

fn manhattan(p: I) -> I {
    let (layer, dist_from_edge_center, _, _) = layer_index(p);
    layer + dist_from_edge_center
}

fn part2(target: I) -> I {
    let size = 1000;
    let mut a = vec![vec![0; size as U]; size as U];
    let c = size / 2;
    a[c as U][c as U] = 1;
    for p in 2.. {
        fn coords(p: I) -> (I, I) {
            let (layer, dist_from_edge_center, quadrant, pos_rel_to_quadrant_start) =
                layer_index(p);
            // pos in quadrant 0
            let (x, y) = if pos_rel_to_quadrant_start > layer {
                (dist_from_edge_center, layer)
            } else {
                (layer, dist_from_edge_center)
            };
            // rotate to actual quadrant
            match quadrant {
                0 => (x, y),   // top right
                1 => (-y, x),  // top left
                2 => (-x, -y), // bottom left
                3 => (y, -x),  // bottom right
                _ => unreachable!(),
            }
        }
        let (px, py) = coords(p);
        let sum = {
            let a_ = &a;
            (-1..=1)
                .flat_map(|y| (-1..=1).map(move |x| a_[(c + py + y) as U][(c + px + x) as U]))
                .sum::<I>()
        };
        let a_ = &mut a[(c + py) as U][(c + px) as U];
        *a_ = sum;
        if *a_ > target {
            return *a_;
        }
    }
    -1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() {
        assert_eq!(0, spiral_idx_to_xy(1))
    }

    #[test]
    fn sample2() {
        assert_eq!(3, spiral_idx_to_xy(12))
    }

    #[test]
    fn sample3() {
        assert_eq!(2, spiral_idx_to_xy(23))
    }

    #[test]
    fn sample4() {
        assert_eq!(31, spiral_idx_to_xy(1024))
    }

    #[test]
    fn submission() {
        println!("{}", spiral_idx_to_xy(325489));
    }

    #[test]
    fn submission2() {
        println!("part 2: {}", part2(325489))
    }
}
