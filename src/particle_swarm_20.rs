use std::collections::HashMap;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Point3D {
    x: isize,
    y: isize,
    z: isize,
}

#[derive(Debug)]
struct Particle {
    position: Point3D,
    velocity: Point3D,
    acceleration: Point3D,
}

fn tick(particle: &mut Particle) {
    particle.velocity.x += particle.acceleration.x;
    particle.velocity.y += particle.acceleration.y;
    particle.velocity.z += particle.acceleration.z;

    particle.position.x += particle.velocity.x;
    particle.position.y += particle.velocity.y;
    particle.position.z += particle.velocity.z;
}

fn find_closest_index(particles: &Vec<Particle>) -> usize {
    let (min_index, min_distance) = particles
        .iter()
        .map(|p| p.position.x.abs() + p.position.y.abs() + p.position.z.abs())
        .enumerate()
        .min_by(|x, y| x.1.cmp(&y.1))
        .unwrap();

    min_index
}

fn find_collisions(particles: &mut Vec<Particle>) {
    let mut occupied_points: HashMap<Point3D, Vec<usize>> = HashMap::new();
    for (i, p) in particles.iter_mut().enumerate() {
        if occupied_points.contains_key(&p.position) {
            occupied_points.get_mut(&p.position).unwrap().push(i);
        } else {
            occupied_points.insert(p.position.clone(), vec![i]);
        }
    }

    for (key, val) in occupied_points.iter() {
        if val.len() > 1 {
            for i in val.iter().rev() {
                particles.remove(*i);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use std::fs::File;
    use std::io::{BufRead, BufReader};

    fn parse_input(path: &str) -> Vec<Particle> {
        let file = File::open(path).unwrap();
        let buffered = BufReader::new(file).lines();
        let mut particle_vec: Vec<Particle> = Vec::new();
        for line in buffered {
            let ln_str = line.unwrap();
            // input lines look like this: "p=<4,0,0>, v=<0,0,0>, a=<-2,0,0>"
            let mut triple = &mut ln_str.split(", ");
            let mut point_vec: Vec<Point3D> = Vec::new();
            for i in 0..3 {
                let point_str = triple.next().unwrap();
                let mut scalars = point_str[3..(point_str.len() - 1)].split(",");
                let point = Point3D {
                    x: scalars.next().unwrap().parse().unwrap(),
                    y: scalars.next().unwrap().parse().unwrap(),
                    z: scalars.next().unwrap().parse().unwrap(),
                };
                point_vec.push(point);
            }
            let particle = Particle {
                position: point_vec[0].clone(),
                velocity: point_vec[1].clone(),
                acceleration: point_vec[2].clone(),
            };
            particle_vec.push(particle);
        }
        particle_vec
    }

    #[test]
    fn test_sample_part1() {
        let path = "src/inputs/day20_test.txt";
        let mut particles = parse_input(path);
        let ticks_count = 5;
        for i in 0..ticks_count {
            for p in &mut particles {
                tick(p);
            }
        }
        assert_eq!(find_closest_index(&particles), 0);
    }

    #[test]
    fn test_submission_part1() {
        let path = "src/inputs/day20.txt";
        let mut particles = parse_input(path);
        let ticks_count = 5000;
        for i in 0..ticks_count {
            for p in &mut particles {
                tick(p);
            }
        }
        println!("Part 1: {}", find_closest_index(&particles));
    }

    #[test]
    fn test_sample_part2() {
        let path = "src/inputs/day20_test2.txt";
        let mut particles = parse_input(path);
        let ticks_count = 5;
        for i in 0..ticks_count {
            for p in &mut particles {
                tick(p);
            }
            find_collisions(&mut particles);
        }
        assert_eq!(1, particles.len());
    }

    #[test]
    fn test_submission_part2() {
        let path = "src/inputs/day20.txt";
        let mut particles = parse_input(path);
        let ticks_count = 5000;
        for i in 0..ticks_count {
            for p in &mut particles {
                tick(p);
            }

            find_collisions(&mut particles);
        }
        println!("Part 2: {}", particles.len());
    }
}
