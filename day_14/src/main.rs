use std::str::FromStr;

use anyhow::{anyhow, Context};
use common::{get_arg, read_file_to_string};

const EMPTY: char = '.';
const ROCK: char = '#';
const SAND: char = 'o';
const SOURCE: char = '+';

#[derive(Debug)]
struct Problem {
    map: Vec<Vec<char>>,
    orig_x_range: (usize, usize),
    y_range: (usize, usize),
    sand_source: (usize, usize),
}

impl FromStr for Problem {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut x_max = 0;
        let mut x_min = 9999;
        let mut y_max = 0;
        let mut y_min = 9999;

        let mut rock_lines = s
            .lines()
            .map(|line| {
                line.split(" -> ")
                    .map(|pair| {
                        let (x, y) = pair.split_once(',').ok_or_else(|| {
                            anyhow!("couldn't split at ',': '{}' found in '{}'", pair, line)
                        })?;

                        let x = x
                            .parse::<usize>()
                            .with_context(|| format!("parsing usize from '{}'", x))?;

                        let y = y
                            .parse::<usize>()
                            .with_context(|| format!("parsing usize from '{}'", y))?;

                        // Note extend of the cave
                        if x > x_max {
                            x_max = x;
                        } else if x < x_min {
                            x_min = x;
                        }

                        if y > y_max {
                            y_max = y;
                        } else if y < y_min {
                            y_min = y;
                        }

                        Ok::<(usize, usize), Self::Err>((x, y))
                    })
                    .collect::<Result<Vec<_>, _>>()
            })
            .collect::<Result<Vec<_>, _>>()?;

        // Shift x coordinates into [0, x_max - x_min] range. Remember to update x
        // coordinate of sand source
        for line in rock_lines.iter_mut() {
            for (ref mut x, _) in line {
                *x -= x_min;
            }
        }

        // We keep y coordinates unchanged, and create map with y in the range
        // of [0, y_max], as sand can possibly stack that hight
        let mut map = vec![vec![EMPTY; y_max + 1]; x_max - x_min + 1];

        let sand_source = (500 - x_min, 0);
        map[sand_source.0][sand_source.1] = SOURCE;

        for line in rock_lines.clone() {
            for pair in line.windows(2) {
                let start = pair[0];
                let end = pair[1];

                let x_range = usize::min(start.0, end.0)..=usize::max(start.0, end.0);
                for x in x_range {
                    let y_range = usize::min(start.1, end.1)..=usize::max(start.1, end.1);
                    for y in y_range {
                        map[x][y] = ROCK;
                    }
                }
            }
        }

        Ok(Problem {
            map,
            orig_x_range: (x_min, x_max),
            y_range: (y_min, y_max),
            sand_source,
        })
    }
}

fn print_map(map: &Vec<Vec<char>>) {
    for y in 0..map[0].len() {
        for x in 0..map.len() {
            print!("{}", map[x][y]);
        }
        println!();
    }
}

fn main() -> Result<(), anyhow::Error> {
    let input_file_path = get_arg(1).context("pass path to input file as first argument")?;
    let input_string = read_file_to_string(&input_file_path)?;
    let Problem { map, .. } = input_string.parse()?;

    print_map(&map);

    println!("Part 1 solution: {}", 0);
    println!("Part 2 solution: {}", 0);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "\
498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";

    #[test]
    fn test_input_parsing() {
        let Problem {
            map,
            orig_x_range,
            y_range,
            sand_source,
        } = TEST_INPUT.parse().unwrap();

        assert_eq!(orig_x_range, (494, 503));
        assert_eq!(y_range, (4, 9));
        assert_eq!(sand_source, (6, 0));
        assert_eq!(map.len(), 10);
        assert_eq!(map[0].len(), 10);
    }
}
