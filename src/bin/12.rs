#![allow(unused_variables, unused_macros)]
static mut TARGET: usize = 1000;

use core::fmt;
use std::array;

use aoc_lib::{_eprintln, parse::parse_split_words};
use num_integer::lcm;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Moon {
    pos: [N; 3],
    vel: [N; 3],
}
type N = i32;

impl fmt::Display for Moon {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "pos=<x={:>3}, y={:>3}, z={:>3}>, vel=<x={:>3}, y={:>3}, z={:>3}>",
            self.pos[0], self.pos[1], self.pos[2],
            self.vel[0], self.vel[1], self.vel[2]
        )
    }
}

fn parse(s: &str) -> Vec<Moon> {
    parse_split_words(s, '=').map(|a| {
        Moon {
            pos: std::array::from_fn(|i| a[i][..a[i].len() - 1].parse().expect(&format!("Failed: {}", a[i]))),
            vel: [0; 3]
        }
    }).collect()
}

fn step(moons: &mut Vec<Moon>) {
    let mut coords: [Vec<_>; 3] = array::from_fn(|_| {
        (0..moons.len()).collect()
    });
    
    coords
    .iter_mut()
    .enumerate()
    .for_each(|(d, c)|
    {
        c.sort_by_key(|i| moons[*i].pos[d]);
        let mut same = 0;
        let mut before = 0;
        let mut last = moons[0].pos[d];
        let mut changes = vec![0; c.len()];
        // step 2: m_i == last => same = 2, before = 0
        for (i, mi) in c.iter().enumerate() {
            let v = moons[*mi].pos[d];
            if v != last {
                before += same;
                same = 1;
            } else {
                same += 1;
            }
            changes[i] -= before;
            last = v;
        }
        let mut same = 0;
        let mut before = 0;
        let mut last = moons[*c.last().unwrap()].pos[d];
        for (i, mi) in c.iter().enumerate().rev() {
            let v = moons[*mi].pos[d];
            if v != last {
                before += same;
                same = 1;
            } else {
                same += 1;
            }
            changes[i] += before;
            moons[*mi].vel[d] += changes[i];
            moons[*mi].pos[d] += moons[*mi].vel[d];
            last = v;
        }
    })
}

fn energy(moons: &Vec<Moon>) -> N {
    moons.iter().map(|m|
        m.pos.map(|x| x.abs()).iter().sum::<N>() *
        m.vel.map(|x| x.abs()).iter().sum::<N>()
    ).sum()
}

pub fn part_one(input: &str) -> Option<N> {
    let mut moons = parse(input);
    for m in moons.iter() {
        _eprintln!("{m}")
    }
    
    for s in 1..=unsafe { TARGET } {
        step(&mut moons);
        
        if s % 10 == 0 {
            _eprintln!("After {} steps:", s);
            for m in moons.iter() {
                _eprintln!("{m}")
            }
        }
    }
    
    let ret = energy(&moons);
    Some(ret as N)
}

fn step_d(moons: &mut Vec<Moon>, d: usize) {
    let mut c: Vec<usize> = (0..moons.len()).collect();
    
    c.sort_by_key(|i| moons[*i].pos[d]);
    let mut same = 0;
    let mut before = 0;
    let mut last = moons[0].pos[d];
    let mut changes = vec![0; c.len()];
    // step 2: m_i == last => same = 2, before = 0
    for (i, mi) in c.iter().enumerate() {
        let v = moons[*mi].pos[d];
        if v != last {
            before += same;
            same = 1;
        } else {
            same += 1;
        }
        changes[i] -= before;
        last = v;
    }
    let mut same = 0;
    let mut before = 0;
    let mut last = moons[*c.last().unwrap()].pos[d];
    for (i, mi) in c.iter().enumerate().rev() {
        let v = moons[*mi].pos[d];
        if v != last {
            before += same;
            same = 1;
        } else {
            same += 1;
        }
        changes[i] += before;
        moons[*mi].vel[d] += changes[i];
        moons[*mi].pos[d] += moons[*mi].vel[d];
        last = v;
    }
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut moons = parse(input);
    
    let cl: Vec<_> = (0..3).map(|d| {
        let start = moons.clone(); // no smaller cycle because reversible, but not necessarily bounded
        let mut ret = 0;
        
        for s in 1.. {
            step_d(&mut moons, d);
            if moons == start {
                ret = s;
                break;
            }
        }
        ret
    }).collect();

    let a = lcm(cl[0], cl[1]);
    let ret = lcm(a, cl[2]);

    Some(ret)
}

// --------------------------------------------- //
advent_of_code::solution!(12);
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_part_one() {
        unsafe { TARGET = 100 };
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1940));
    }
    
    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4686774924));
    }
}
