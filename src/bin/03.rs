#![allow(unused_variables, unused_macros)]

use std::u64;

use aoc_lib::{_dbg, grid::{CartesianRTree, Manifold, PopulatedManifold}};

advent_of_code::solution!(3);

pub type P2 = [isize; 2];

fn atop2(s: &str) -> P2 {
    let (dir, n) = s.split_at(1);
    let value: isize = n.parse().expect("Invalid number");
    
    match dir {
        "R" => [value, 0],
        "L" => [-value, 0],
        "U" => [0, value],
        "D" => [0, -value],
        _ => panic!("Invalid direction"),
    }
}

fn parse(input: &str) -> (Vec<P2>, Vec<P2>) {
    let mut lines = input.lines();
    let l = lines
    .next()
    .unwrap()
    .split(',')
    .map(|s| atop2(s.trim()))
    .collect();
    let r = lines
    .next()
    .unwrap()
    .split(',')
    .map(|s| atop2(s.trim()))
    .collect();
    (l, r)
}

#[derive(Debug)]
struct M {}

fn distance(p: &P2) -> usize {
    p[0].unsigned_abs() + p[1].unsigned_abs()
}

fn length(p: &P2) -> u64 {
    (p[0].abs() + p[1].abs()) as u64
}

pub fn part_one(input: &str) -> Option<u64> {
    let (l, r) = parse(input);
    let rel = (0, 0);
    
    type Grid = CartesianRTree<[isize; 2], M>;
    
    let mut grid: Grid = CartesianRTree::new();
    for direction in l {
        let start = grid.position();
        let end = grid.shift(direction).unwrap();
        grid.store(M {}, start, end);
    }
    grid.set_position([0, 0]).unwrap();
    let mut xs = vec![];
    
    let start = std::cell::RefCell::new(grid.position());
    
    grid.travel(r.into_iter(), |obj| {
        let x = obj.bounds.min_point(&start.borrow());
        if distance(&x) != 0 {
            match xs.as_slice() {
                [] => xs.push(x),
                [.., last] => {
                    if distance(last) > distance(&x) {
                        xs.push(x)
                    }
                }
            }
        }
    }, |(_, e), _| { *start.borrow_mut() = e; true } );
    
    _dbg!(&xs);
    let ret = distance(xs.last().unwrap());
    Some(ret as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (l, r) = parse(input);
    let rel = (0, 0);
    
    type Grid = CartesianRTree<[isize; 2], (u64, [isize; 2])>;
    
    let mut grid: Grid = CartesianRTree::new();
    
    let mut total = 0;
    for direction in l {
        let start = grid.position();
        let end = grid.shift(direction).unwrap();
        grid.store((total, start), start, end);
        total += length(&direction);
    }
    grid.set_position([0, 0]).unwrap();
    
    let min = std::cell::RefCell::new(u64::MAX);
    let total = std::cell::RefCell::new(0);
    let start = std::cell::RefCell::new(grid.position());
    
    grid.travel(r.into_iter(), |obj| {
        let total = *total.borrow();
        let start = start.borrow();
        let mut min = min.borrow_mut();
        
        if total != 0 {
            let x = obj.bounds.min_point(&start);
            
            let (o_t, o_s) = obj.data;
            _dbg!(obj, x);
            let total_length = total + length(&[x[0] - start[0], x[1] - start[1]]) + length(&[x[0] - o_s[0], x[1] - o_s[1]]) + o_t;
            if total_length < *min {
                *min = total_length;
            }
        }
    }, |(_, e), direction| { 
        let mut total = total.borrow_mut();
        let mut start = start.borrow_mut();
        let min = *min.borrow();

        *total += length(&direction);
        *start = e;
        *total <= min
    } );
    
    let ret = min.borrow().clone();
    Some(ret)
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
