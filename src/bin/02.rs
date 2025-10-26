#![allow(unused_variables, unused_macros)]

use aoc_lib::{_dbg};
use itertools::{iproduct};

advent_of_code::solution!(2);

fn parse(s: &str) -> Vec<usize> {
    s.split(',').map(|s| s.trim().parse().expect(&format!("Failed to parse: {s}"))).collect()
}

fn run(v: &mut Vec<usize>, mut i: usize) -> Option<usize> {
    loop {
        match v[i] {
            99 => break,
            1 | 2 =>
            {
                let (a, b, target) = (v[i + 1], v[i + 2], v[i + 3]);

                assert!(target < v.len());
                
                v[target] = if v[i] == 1 {
                    v[a] + v[b]
                } else {
                    v[a] * v[b]
                };
                i += 4;
                if i > v.len() {
                    return None;
                }
            }
            _ => panic!("{}", v[i])
        }
    }
    Some(v[0])
}

static TARGET: usize = 19690720;

pub fn part_one(input: &str) -> Option<u64> {
    let mut v = parse(input);
    _dbg!(v.len());
    v[1] = 12;
    v[2] = 2;
    run(&mut v, 0);
    
    let ret = v[0];
    Some(ret as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut _v = parse(input);
    iproduct!(0..99, 0..99).find_map(|(x, y)| {
        let mut v = _v.clone();
        v[1] = x;
        v[2] = y;
        match run(&mut v, 0) {
            Some(ret) if ret == TARGET => {
                let ret = 100 * x + y;
                Some(ret as u64)
            },
            _ => None
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
    
    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
