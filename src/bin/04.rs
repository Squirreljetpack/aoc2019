#![allow(unused_variables, unused_macros, dead_code)]

use aoc_lib::{_dbg, parse::get};

advent_of_code::solution!(4);

fn parse(s: &str) -> [u64; 2] {
    let u = s.split_once('-').unwrap();
    [get(u.0), get(u.1)]
}

fn vton(v: &Vec<u64>) -> u64 {
    v.into_iter().fold(0, |acc, d| acc * 10 + d)
}
fn ntov(mut n: u64) -> Vec<u64> {
    if n == 0 {
        return vec![0];
    }
    let mut v = Vec::new();
    while n > 0 {
        v.push(n % 10);
        n /= 10;
    }
    v.reverse();
    v
}

fn next_valid<const F: bool>(mut n: u64) -> u64 {
    loop {
        let mut v = ntov(n);
        let mut last = v[0];
        let mut double = false;
        let mut ignore = false;
        
        for (i, x) in v[1..].iter_mut().enumerate() {
            *x = last.max(*x);

            if F {
                if *x == last {
                    double = true;
                }
            } else {
                if *x != last {
                    ignore = false;
                    if double && ! ignore {
                        ignore = true;
                    }
                } else if ! ignore {
                    if double {
                        ignore = true;
                        double = false;
                    } else {
                        double = true;
                    }
                }
            }
            
            last = *x;
        }
        n = vton(&v);
        if double {
            _dbg!(n);
            break n
        }
        n += 1;
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let [mut start, end] = parse(input);
    // let mut curr = ntod(start);
    let mut count = 0;
    
    while { start = next_valid::<true>(start); true } && start <= end {
        count += 1;
        start += 1;
    }
    
    Some(count as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let [mut start, end] = parse(input);
    // let mut curr = ntod(start);
    let mut count = 0;
    
    while { start = next_valid::<false>(start); true } && start <= end {
        count += 1;
        start += 1;
    }
    
    Some(count as u64)
}

// static mut TARGET: u64 = 0;

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
