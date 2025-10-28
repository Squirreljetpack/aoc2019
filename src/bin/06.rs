#![allow(unused_variables, unused_macros)]
static mut TARGET: &str = "COM";

use std::collections::HashMap;

use aoc_lib::{_dbg, dfs::tree::Node};

fn parse(s: &str) -> HashMap<&str, Vec<&str>> {
    let mut ret: HashMap<&str, Vec<&str>> = HashMap::new(); // y annotation needed?
    
    for line in s.lines() {
        let (k, v) = line.split_once(")").unwrap();
        ret.entry(k.into()).or_default().push(v.into());
    }
    ret
}

pub fn part_one(input: &str) -> Option<u64> {
    let map = parse(input);
    let tree = Node::from_edges(&map, &unsafe { TARGET });
    let mut total = 0;
    tree.walk_simple(
        0,
        &mut total,
        &|d, total, _| { 
            *total += d + 1;
            Some(d + 1)
        },
    );
    
    Some(total as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let map = parse(input);
    let tree = Node::from_edges(&map, &unsafe { TARGET });
    
    let mut paths = (vec![], vec![]);
    let _ = tree.walk(
        vec![],
        &mut paths,
        &|hist, paths, node| {
            if node.elem == "YOU" {
                paths.0 = hist.clone();
                if paths.1.is_empty() {
                    Ok(None)
                } else {
                    Err(())
                }
            } else if node.elem == "SAN" {
                paths.1 = hist.clone();
                if paths.0.is_empty() {
                    Ok(None)
                } else {
                    Err(())
                }
            } else {
                let mut h = hist.clone();
                h.push(node.elem);
                Ok(Some(h))
            }
        },
        &|_, _| {}
    );

    _dbg!(&paths);
    let (y, s) = (paths.0, paths.1);
    let common = y.iter().zip(s.iter()).take_while(|(a, b)| a == b).count();
    
    let ret = y.len() + s.len() - 2 * common;
    Some(ret as u64)
}

// --------------------------------------------- //
advent_of_code::solution!(6);
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(42));
    }
    
    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 1));
        assert_eq!(result, Some(4));
    }
}
