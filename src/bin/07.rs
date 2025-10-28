#![allow(unused_variables, unused_macros)]
// static mut TARGET: u64 = 0;

use itertools::Itertools;


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

#[derive(Clone)]
struct Amplifier {
    v: Vec<isize>,
    i: usize,
    halted: bool,
}

impl Amplifier {
    fn new(v: Vec<isize>) -> Self {
        Self {
            v,
            i: 0,
            halted: false,
        }
    }
    
    fn run(&mut self, inputs: Vec<isize>) -> Option<isize> {
        let o = self._run(inputs);
        if let Some(i) = o.last() {
            Some(*i)
        } else {
            assert!(&self.halted);
            None
        }
    }
    fn _run(&mut self, mut inputs: Vec<isize>) -> Vec<isize> {
        let mut outputs = vec![];
        
        loop {
            let (modes, op) = get_modes(self.v[self.i]);
            match op {
                99 => {
                    self.halted = true;
                    break;
                }
                3 => {
                    if inputs.is_empty() {
                        break;
                    }
                    let a = self.v[self.i + 1];
                    self.v[a as usize] = inputs.remove(0);
                    self.i += 2;
                }
                4 => {
                    let a = self.v[self.i + 1];
                    if modes[0] {
                        outputs.push(a);
                    } else {
                        outputs.push(self.v[a as usize]);
                    }
                    self.i += 2;
                    break;
                }
                1 | 2 | 7 | 8 => {
                    let (mut a, mut b, target) = (self.v[self.i + 1], self.v[self.i + 2], self.v[self.i + 3]);
                    
                    if !modes[0] {
                        a = self.v[a as usize]
                    }
                    if !modes[1] {
                        b = self.v[b as usize]
                    }
                    
                    self.v[target as usize] = if op == 1 { a + b } 
                    else if op == 2 { a * b }
                    else if (op == 7 && a < b) || (op == 8 && a == b) {
                        1
                    } else { 0 };
                    
                    self.i += 4;
                }
                5 | 6 => {
                    let (mut a, mut b) = (self.v[self.i + 1], self.v[self.i + 2]);
                    if !modes[0] {
                        a = self.v[a as usize]
                    }
                    if !modes[1] {
                        b = self.v[b as usize]
                    }
                    
                    if (op == 5 && a != 0) || (op == 6 && a == 0) {
                        self.i = b as usize;
                    } else {
                        self.i += 3;
                    }
                }
                _ => panic!("{}", self.v[self.i]),
            }
        }
        outputs
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let v = parse(input);
    let mut outs = vec![];
    
    for x in (0..=4).permutations(5) {
        let mut input = 0;
        for p in x {
            let mut _v = v.clone();
            let mut amp = Amplifier::new(v.clone());
            input = amp.run(vec![p, input]).unwrap();
        }
        outs.push(input);
    }
    
    Some(*outs.iter().max().unwrap() as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let v = parse(input);
    let mut ret = 0;
    
    for x in (5..=9).permutations(5) {
        let mut amps = vec![];
        let mut input = 0;
        
        for p in x {
            let mut _v = v.clone();
            let mut amp = Amplifier::new(v.clone());
            input = amp.run(vec![p, input]).unwrap();
            amps.push(amp);
        };
        
        let mut ax = 0;
        let mut e_last = input;
        loop {
            let output = amps[ax]._run(vec![input]);
            if let Some(last) = output.last() {
                input = *last;
                if ax == 4 {
                    e_last = input;
                }
            }
            if amps[ax].halted {
                // assert!(ax == 4 || output.len() > 0); <- break on any halt ig
                assert!(output.is_empty());
                break;
            } else {
                assert!(!output.is_empty())
            }
            ax = (ax + 1) % 5;
        }

        ret = ret.max(e_last)
    }
    
    Some(ret as u64)
}

// --------------------------------------------- //
advent_of_code::solution!(7);
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(43210));
    }
    
    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 4));
        assert_eq!(result, Some(139629729));
        let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 5));
        assert_eq!(result, Some(18216));
    }
}
