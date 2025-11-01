#![allow(unused_variables, unused_macros)]

use aoc_lib::{_eprintln, grid::{CartesianCollection, Collection, grids::DequeueGrid}};
// static mut TARGET: usize = 0;
type N = i32;


fn parse(s: &str) -> Vec<isize> {
    s.split(',').map(|s| s.trim().parse().expect(&format!("Failed to parse: {s}"))).collect()
}

#[derive(Clone)]
struct IntCode {
    v: Vec<isize>,
    i: usize,
    halted: bool,
    relative_base: isize,
}

impl IntCode {
    fn new(v: Vec<isize>) -> Self {
        Self {
            v,
            i: 0,
            halted: false,
            relative_base: 0,
        }
    }
    
    fn get_addr(&self, addr: usize, mode: u8) -> isize {
        let a = self.get(addr as isize);
        match mode {
            0 => self.get(a),
            1 => a,
            2 => self.get(a + self.relative_base),
            _ => panic!("Invalid mode: {}", mode),
        }
    }
    
    fn set_addr(&mut self, addr: usize, val: isize, mode: u8) {
        let a = self.get(addr as isize);
        match mode {
            0 => self.set(a, val),
            2 => self.set(a + self.relative_base, val),
            _ => panic!("Invalid mode: {}", mode),
        }
    }
    
    fn get_modes(&self) -> ([u8; 3], u8) {
        Self::parse_modes(self.v[self.i])
    }
    
    // this should really return a result
    fn run(&mut self, mut inputs: Vec<isize>) -> Vec<isize> {
        let mut outputs = vec![];
        if self.halted {
            _eprintln!("Abort: halted");
            return outputs;
        }
        
        loop {
            let (modes, op) = self.get_modes();
            match op {
                99 => {
                    self.halted = true;
                    break;
                }
                3 => {
                    if inputs.is_empty() {
                        break;
                    }
                    self.set_addr(self.i + 1, inputs.remove(0), modes[0]);
                    self.i += 2;
                }
                4 => {
                    let a = self.get_addr(self.i + 1, modes[0]);
                    outputs.push(a);
                    self.i += 2;
                }
                // add multiply lt gt
                1 | 2 | 7 | 8 => {
                    let (a, b) = (
                        self.get_addr(self.i + 1, modes[0]),
                        self.get_addr(self.i + 2, modes[1]),
                    );
                    let val = if op == 1 {
                        a + b
                    } else if op == 2 {
                        a * b
                    } else if (op == 7 && a < b) || (op == 8 && a == b) {
                        1
                    } else {
                        0
                    };
                    self.set_addr(self.i + 3, val, modes[2]);
                    self.i += 4;
                }
                // jump if
                5 | 6 => {
                    let a = self.get_addr(self.i + 1, modes[0]);
                    if (op == 5 && a != 0) || (op == 6 && a == 0) {
                        self.i = self.get_addr(self.i + 2, modes[1]) as usize;
                    } else {
                        self.i += 3;
                    }
                }
                // change relative base
                9 => {
                    let a = self.get_addr(self.i + 1, modes[0]);
                    self.relative_base += a;
                    self.i += 2;
                }
                _ => panic!("Invalid instruction: {}", self.v[self.i]),
            }
        }
        outputs
    }
}

impl IntCode {
    fn get(&self, addr: isize) -> isize {
        assert!(addr >= 0);
        *self.v.get(addr as usize).unwrap_or(&0)
    }
    
    fn set(&mut self, addr: isize, val: isize) {
        assert!(addr >= 0);
        let addr = addr as usize;
        if addr >= self.v.len() {
            let mut new_len = self.v.len().max(1);
            while addr >= new_len {
                new_len *= 2;
            }
            self.v.resize(new_len, 0);
        }
        
        self.v[addr] = val;
    }
    fn parse_modes(n: isize) -> ([u8; 3], u8) {
        let opcode = n % 100;
        let mut modes = [0; 3];
        let mut rem = n / 100;
        
        for i in 0..3 {
            modes[i] = (rem % 10) as u8;
            rem /= 10;
        }
        
        (modes, opcode as u8)
    }
}

// ----



pub fn part_one(input: &str) -> Option<N> {
    let p = parse(input);
    
    let mut grid = DequeueGrid::new();
    
    let mut ic = IntCode::new(p);
    let outs = ic.run(vec![]);
    
    for chunk in outs.chunks(3) {
        match chunk {
            &[x, y, c] =>
            {
                grid.insert([y, x], c);
            },
            _ => panic!("Unexpected: {:?}", chunk),
        }
    }
    
    let ret = grid.enumerate().filter(|(_, x)| **x == 2).count();
    
    let p = grid.draw(|(i, x)| (i, x.to_string().chars().next().unwrap()));
    _eprintln!("{p}");
    
    Some(ret as N)
}

pub fn part_two(input: &str) -> Option<N> {
    let mut p = parse(input);
    p[0] = 2;

    let mut grid = DequeueGrid::new();
    
    let mut ic = IntCode::new(p);
    let outs = ic.run(vec![]);
    let mut bx = 0;
    let mut px = 0;
    for chunk in outs.chunks(3) {
        match chunk {
            &[x, y, c] =>
            {
                grid.insert([y, x], c);
                match c {
                    4 => { bx = x },
                    3 => { px = x },
                    _ => {}
                }
            },
            _ => panic!("Unexpected: {:?}", chunk),
        }
    }
    
    let mut blocks = grid.enumerate().filter(|(_, x)| **x == 2).count();
    assert!(!ic.halted);
    
    while blocks > 0 && !ic.halted {
        let input =
            if bx > px {
                1
            } else if bx < px {
                -1
            } else {
                0
            };
        
        let outs = ic.run(vec![input]);
        
        for chunk in outs.chunks(3) {
            match chunk {
                &[x, y, c] =>
                {
                    if let Some(2) = grid.insert([y, x], c) {
                        assert!(c == 0);
                        blocks -= 1;
                    }
                    match c {
                        4 => { bx = x },
                        3 => { px = x },
                        _ => {}
                    }
                },
                _ => panic!("Unexpected: {:?}", chunk),
            }
        }
    }
    
    let ret = *grid.get([0, -1]).unwrap();

    let p = grid.draw(|(i, x)| (i, x.to_string().chars().next().unwrap()));
    _eprintln!("{p}");
    Some(ret as N)
}

// --------------------------------------------- //
advent_of_code::solution!(13);
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }
    
    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
