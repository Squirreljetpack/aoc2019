This document outlines the process for creating example files for a given day's puzzle.

## Process

To add examples for a given day `n`:

THROUGHOUT THE WHOLE PROCESS, DO NOT SEARCH ONLINE
DO NOT SOLVE THE PUZZLE

In the following, `DD` is the zero-padded day number (e.g., `01`, `02`)

1. **Identify Examples:** Your file is `data/puzzles/DD.md`. Do not attempt to list for the file, it has been gitignored, just read it and abort if the file doesn't exist. Carefully read the puzzle description for Part 1 (and Part 2 if existing in the file) to find all provided examples. These include the input and the expected output.
2. **Create Example Files:** For each example, create a new file in the `data/examples/` directory.
3. **Name the Files:** Name the files using the format `DD-i.txt`, where `i` is a sequential integer starting from 1 for each distinct example.
4. **Add Content:** Populate each file with its corresponding input data.
5. **Update Tests:** In the solution file for the day, located at `src/bin/DD.rs`, update the `#[cfg(test)]` block following this pattern:

```
    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part("examples", DAY, 1));
        assert_eq!(result, Some(0));
        let result = part_one(&advent_of_code::template::read_file_part("examples", DAY, 2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 10));
        assert_eq!(result, Some(1));
        let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 11));
        assert_eq!(result, Some(5));
    }
```

- If part 2 was not processed, do not add tests for it!
- If the current solution cannot test an example, (i.e. the puzzle asks for the value of "a", while the example only has a value for "b"), make the minimal modifications to the example file to make it testable.
