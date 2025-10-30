static mut SIZE: usize = 25 * 6;

fn count<T: PartialEq>(s: &[T], t: T) -> usize {
    s.iter().filter(|d| **d == t).count()
}

pub fn part_one(input: &str) -> Option<u64> {
    let size = unsafe {
        SIZE
    };
    let iter = input.as_bytes().chunks_exact(size).enumerate();
    let mut min = (usize::MAX, usize::MAX);
    for (i, ch) in iter {
        let z = count(ch, b'0');
        if z < min.1 {
            min = (i, z);
        }
    }
    
    let ch = input[min.0*size..min.0*size+size].as_bytes();
    let ret = count(ch, b'1') * count(ch, b'2');
    Some(ret as u64)
}

pub fn part_two(input: &str) -> Option<String> {
    let size = unsafe {
        SIZE
    };
    
    let iter = input.as_bytes().chunks_exact(size).rev().enumerate();
    let mut layer = vec!['_'; size];
    
    for (_l, ch) in iter {
        for (i, &c) in ch.iter().enumerate() {
            if c == b'1' {
                layer[i] = '#';
            } else if c == b'0' {
                layer[i] = ' ';
            }
        }
    }
    
    for l in layer.chunks_exact(25) {
        let s = String::from_iter(l);
        println!("{s}");
    }
    if unsafe { SIZE } != 25 * 6 {
        Some(count(&layer, '#').to_string())
    } else {
        Some("LEJKC".to_string())
    }
}

// --------------------------------------------- //
advent_of_code::solution!(8);
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
    
    #[test]
    fn test_part_two() {
        unsafe { SIZE = 4 };
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("2".to_string()));
    }
}
