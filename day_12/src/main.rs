use std::str::FromStr;

use anyhow::Context;
use common::{get_arg, read_file_to_string};

const START: u8 = 'S' as u8;
const END: u8 = 'E' as u8;

#[derive(Debug)]
struct Problem {
    map: Vec<Vec<u8>>,
}

impl FromStr for Problem {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let map = s.lines().map(|l| l.as_bytes().to_vec()).collect();

        Ok(Problem { map })
    }
}

fn find_named_point(map: &[Vec<u8>], point: u8) -> Option<(usize, usize)> {
    for x in 0..map.len() {
        // Assumes every row has the same length
        for y in 0..map[0].len() {
            if map[x][y] == point {
                return Some((x, y));
            }
        }
    }

    None
}

fn main() -> Result<(), anyhow::Error> {
    let input_file_path = get_arg(1).context("pass path to input file as first argument")?;
    let input_string = read_file_to_string(&input_file_path)?;

    println!("Part 1 solution: {}", 0);
    println!("Part 2 solution: {}", 0);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "\
Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";

    #[test]
    fn test_input_parsing() {
        let Problem { map } = TEST_INPUT.parse().unwrap();

        assert_eq!(map.len(), 5);
        assert_eq!(map[0].len(), 8);
        assert_eq!(map[0][0], START);
        assert_eq!(map[2][5], END);
    }

    #[test]
    fn test_find_named_point() {
        let Problem { map } = TEST_INPUT.parse().unwrap();

        assert_eq!(find_named_point(&map, START), Some((0, 0)));
        assert_eq!(find_named_point(&map, END), Some((2, 5)));
        assert_eq!(find_named_point(&map, 255), None);
    }
}
