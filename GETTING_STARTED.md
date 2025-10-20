A modified AOC rust template

For PRIVATE repositories ONLY (uncomment `data/inputs/*` and `data/puzzles/*` in .gitignore if you want to make it public)

# Commands

Requires [cargo-alias-exec](https://crates.io/crates/cargo-alias-exec) and [aoc-cli](https://crates.io/crates/aoc-cli).
The following aliases apply to the earliest incomplete day if no day is given. A day is considered complete when the last line of `data/puzzles/{day}.md` begins with `#` (i.e. `#complete or #skip`). `submit_p2` will mark the day as complete.

### init

- Check out [README.md](./README.md)
- Set the year in config.toml
- To retrieve the session cookie, press F12 anywhere on the Advent of Code website to open your browser developer tools (Cmd+Shift+J). Look in Cookies under the Application or Storage tab, and copy out the session cookie value
- `pbpaste > ~/.adventofcode.session`
- In your github repo settings/secrets/actions, set these secrets:
	- `AOC_USER_ID`: Go to [this page](https://adventofcode.com/settings) and copy your user id. It's the number behind the `#` symbol in the first name option.
	- `AOC_SESSION`: ^
- create your aoc_lib crate in a sibling directory, or remove it from the dependencies

### scaffold

- `cargo start` (`c @s`)
	- `cargo refresh [n]` can force redownload, although it shouldn't be needed.

### solve

- _write your solution_
- test (from your editor or with `c @t`)
- you may want to ask gemini to add examples for day $n

### submit

- `c submit_p1`: (`c @s1/@s2`)
- solve + benchmark: `c @s3`
- git push # workflow updates the leaderboard

# Links

- https://adventofcode.com
- https://github.com/fspoettel/advent-of-code-rust - the base template with some modifications:
	- simplified ci compared to the base project: only README stars are updated on push, reduced repository secrets needed.
	- added cargo aliases for workflows
	- a set of initial dependencies for solutions
	- minor fixes