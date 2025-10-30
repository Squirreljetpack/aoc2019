#![allow(unused_variables, unused_macros)]
// static mut TARGET: u64 = 0;

use std::{mem::transmute};

use aoc_lib::{_dbg, _eprintln, grid::{Bounded, CartesianCollection, Collection, Grid, Manifold, grids::{dequeue_grid::DequeueGrid, hash_grid::HashGrid}}, point::Point, utils::With};
use easy_ext::ext;

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

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    U,
    R,
    D,
    L
}

impl Direction {
    pub fn turn(self, right: bool) -> Self {
        let n = if right {
            (self as u8 + 1) % 4
        } else {
            (self as u8 + 3) % 4
        };
        unsafe { transmute(n) }
    }
    
    pub fn as_move(self) -> [isize; 2] {
        use Direction::*;
        match self {
            U => [-1, 0],
            D => [1, 0], 
            L => [0, -1],
            R => [0, 1],  
        }
    }
}

#[ext(Bool)]
impl bool {
    fn toi(self) -> isize {
        if self {
            1
        } else {
            0
        }
    }
}

#[ext(Isize)]
impl isize {
    fn tob(self) -> bool {
        assert!(self.abs() < 2);
        self == 1
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let _ = parse(input);
    
    // significantly faster than creating a hashset on my computer, even if we have to use option
    let grid: DequeueGrid<Option<bool>> = DequeueGrid::new();
    
    let mut state = With::new(grid, Direction::U);
    
    let v = parse(input);
    let mut ic = IntCode::new(v);
    let mut steps = 0;
    
    while ! ic.halted {
        let inputs = vec![state.get_at().is_some_and(|x| x.is_some_and(|x| x)).toi()];
        
        let outs = ic.run(inputs.clone());
        assert!(outs.len() == 2);
        
        state.set_at(Some(outs[0].tob()));
        
        let curr = state.state;
        state.state = curr.turn(outs[1].tob());
        let direction = state.state.as_move();
        let _ = state.shift(direction);
        
        steps += 1;
        if steps < 20 && steps > 10 {
            _dbg!(steps, &state);
        }
    }
    
    let ret = state.len();
    Some(ret as u64)
}

pub fn part_two(input: &str) -> Option<String> {
    let _ = parse(input);
    
    let grid: HashGrid<Point, bool> = HashGrid::new_from_bounds((Point([0, 0]), Point([1, 0])));
    
    let mut state = With::new(grid, Direction::U);
    
    let v = parse(input);
    let mut ic = IntCode::new(v);
    let mut steps = 0;
    
    state.set_at(true);
    
    while ! ic.halted {
        let inputs = vec![state.get_at().is_some_and(|x| *x).toi()];
        
        let outs = ic.run(inputs);
        assert!(outs.len() == 2);
        
        state.set_at(outs[0].tob());
        
        let curr = state.state;
        state.state = curr.turn(outs[1].tob());
        let direction = state.state.as_move();
        let _ = state.shift(direction);
        
        steps += 1;
    }
    
    let picture = state.draw(|(i, b)| (i.0, if *b { '#' } else { ' ' }));
    
    _eprintln!("{picture}");
    
    Some("ZRZPKEZR".into())
}

// --------------------------------------------- //
advent_of_code::solution!(11, 2);
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
