#![allow(unused_variables, unused_macros)]

use aoc_lib::_dbg;
static mut TARGET: isize = 1;

fn parse(s: &str) -> Vec<isize> {
    s.split(',').map(|s| s.trim().parse().expect(&format!("Failed to parse: {s}"))).collect()
}

fn get_modes(n: isize) -> ([bool; 3], u8) {
    let opcode = n % 100;
    let mut modes = [false; 3];
    let mut rem = n / 100;

    for i in 0..3 {
        modes[i] = rem % 10 != 0;
        rem /= 10;
    }

    (modes, opcode as u8)
}

fn run(v: &mut Vec<isize>, mut i: usize) -> Option<Vec<isize>> {
    let mut ret = Vec::new();

    loop {
        let (modes, op) = get_modes(v[i]);
        match op {
            99 => break,
            3 => {
                let a = v[i+1];
                v[a as usize] = unsafe { TARGET };

                i += 2;
                if i > v.len() {
                    return None;
                }
            },
            4 => {
                let a = v[i+1];
                if modes[0] {
                    ret.push(a);
                } else {
                    ret.push(v[a as usize]);
                }

                i += 2;
                if i > v.len() {
                    return None;
                }
            }
            1|2 =>
            {
                let (mut a, mut b, target) = (v[i + 1], v[i + 2], v[i + 3]);

                if !modes[0] {
                    a = v[a as usize]
                }
                if !modes[1] {
                    b = v[b as usize]
                }
                assert!(! modes[2]);
                
                v[target as usize] = if op == 1 {
                    a + b
                } else {
                    a * b
                };

                i += 4;
                if i > v.len() {
                    return None;
                }
            }
            5|6 => {
                let (mut a, mut b) = (v[i + 1], v[i + 2]);
                if !modes[0] {
                    a = v[a as usize]
                }
                if !modes[1] {
                    b = v[b as usize]
                }

                if (op == 5 && a != 0) || (op == 6 && a == 0) {
                    i = b as usize;
                } else {
                    i += 3;
                }
                if i > v.len() {
                    return None;
                }
            }
            7|8 => {
                let (mut a, mut b, target) = (v[i + 1], v[i + 2], v[i + 3]);

                if !modes[0] {
                    a = v[a as usize]
                }
                if !modes[1] {
                    b = v[b as usize]
                }

                v[target as usize] = if (op == 7 && a < b) || (op == 8 && a == b) {
                    1
                } else {
                    0
                };
                
                i += 4;
                if i > v.len() {
                    return None;
                }
            }
            _ => panic!("{}", v[i])
        }
    }
    Some(ret)
}


pub fn part_one(input: &str) -> Option<u64> {
    let mut v = parse(input);
    let ret = run(&mut v, 0).unwrap();
    _dbg!(&ret);
    
    Some(*ret.last().unwrap() as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    unsafe { TARGET = 5 };
    let mut v = parse(input);
    let ret = run(&mut v, 0).unwrap();
    _dbg!(&ret);
    
    Some(*ret.last().unwrap() as u64)
}

// --------------------------------------------- //
advent_of_code::solution!(5);
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
