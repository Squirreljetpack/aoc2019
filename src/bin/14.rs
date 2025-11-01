#![allow(unused_variables, unused_macros)]
// static mut TARGET: usize = 0;
type N = i32;

fn parse(s: &str) -> () {

}

pub fn part_one(input: &str) -> Option<N> {
    let _ = parse(input);

    let ret = 0;
    Some(ret as N)
}

pub fn part_two(input: &str) -> Option<N> {
    let _ = parse(input);

    let ret = 0;
    Some(ret as N)
}

// --------------------------------------------- //
advent_of_code::solution!(14);
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
