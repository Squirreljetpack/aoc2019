#![allow(unused_variables, unused_macros)]
static TARGET: usize = 200;

use std::{cmp::Ordering, collections::{HashMap, HashSet}};
use aoc_lib::{_dbg, point::{Point, PointExt}};

fn parse(s: &str) -> Vec<Point<i16>> {
    // let grid: Vec<Vec<bool>> = s
    //     .lines()
    //     .map(|line| line.chars().map(|c| c == '#').collect())
    //     .collect();
    // let dimensions = [grid[0].len(), grid.len()];
    // (grid, dimensions)
    
    let mut asteroids = Vec::new();
    for (y, line) in s.lines().enumerate() {
        for (x, char) in line.chars().enumerate() {
            if char == '#' {
                asteroids.push(Point([x as i16, y as i16]));
            }
        }
    }
    asteroids
}

pub fn part_one(input: &str) -> Option<u64> {
    let points = parse(input);
    let ret = points.iter().map(|p| {
        let vis: HashSet<_> = points.iter().map(|p2| p.sub(p2).simplify().0).collect();
        vis.len()
    }).max();
    
    Some(ret.unwrap() as u64 - 1)
}

// atan2(y = x0, -y0).cmp(..)
fn cmp_atan(a: &[i16; 2], b: &[i16; 2]) -> Ordering {
    let ([x0, y0], [x1, y1]) = (*a, *b);
    (x0 < 0).cmp(&(x1 < 0)).then(
        (y1*x0-y0*x1).cmp(&0).then((y1 > 0).cmp(&(y0 > 0)))
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    let points = parse(input);
    let base = points.iter().max_by_key(|p| {
        let vis: HashSet<_> = points.iter().map(|p2| p.sub(p2).simplify().0).collect();
        vis.len()
    }).unwrap().clone();
    
    let mut around = HashMap::new();
    for p in points {
        if p != base {
            let d = p.sub(&base);
            let s = d.simplify().0;
            let s = [s[0], -s[1]]; // up is negative ><
            around.entry(s).and_modify(|x: &mut Vec<_>| x.push(p)).or_insert(vec![p]);
        }
    };
    
    let mut u: Vec<_> = around.keys().collect();
    u.sort_by(|a, b| cmp_atan(a, b));
    _dbg!(&u, &u.len());
    
    let mut min_len = 1;
    let k = if TARGET > u.len() {
        let mut total = u.len();
        min_len += 1;
        
        'outer: loop {
            for k in u.iter().filter(|k| around[**k].len() >= min_len) {
                total += 1;
                if total == TARGET {
                    break 'outer (*k).clone();
                }
            }
            min_len += 1;
        }
    } else {
        u[TARGET - 1].clone()
    };
    
    let mut v = around.remove(&k).unwrap();
    _dbg!(&v);
    let (_, p, _) = v.select_nth_unstable_by_key(min_len - 1,|p| (p[0] - base[0]).abs());
    
    let ret = p[0] * 100 + p[1];
    Some(ret as u64)
}

// --------------------------------------------- //
advent_of_code::solution!(10);
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(210));
    }
    
    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(802));
    }
}
