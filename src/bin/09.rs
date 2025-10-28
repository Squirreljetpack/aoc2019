#![allow(unused_variables, unused_macros)]
// static mut TARGET: u64 = 0;

fn parse(s: &str) -> () {

}

pub fn part_one(input: &str) -> Option<u64> {
    let _ = parse(input);

    let ret = 0;
    Some(ret as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let _ = parse(input);

    let ret = 0;
    Some(ret as u64)
}

// --------------------------------------------- //
advent_of_code::solution!(9);
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
