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

fn simulate_sand(map: &Vec<Vec<char>>, sand_source: (usize, usize)) -> (Vec<Vec<char>>, u64) {
    let mut map = map.clone();
    let mut sand_units_that_came_to_rest = 0;
    let x_max = map.len() - 1;
    let y_max = map[0].len() - 1;

    'simulation: loop {
        let (mut sand_x, mut sand_y) = sand_source;

        'sand: loop {
            // Check if sand is going to fall through bottom of simulation if it
            // keeps moving down
            if sand_y + 1 > y_max {
                break 'simulation;
            }

            // Check if cell below is empty
            if map[sand_x][sand_y + 1] == EMPTY {
                sand_y += 1;
                continue 'sand;
            }

            // Check if moving left would place sand outside the map
            if sand_x == 0 {
                break 'simulation;
            }

            // Check if bottom left is empty
            if map[sand_x - 1][sand_y + 1] == EMPTY {
                sand_x -= 1;
                sand_y += 1;
                continue 'sand;
            }

            // Check if moving right would place sand outside the map
            if sand_x == x_max {
                break 'simulation;
            }

            // Check if bottom right is empty
            if map[sand_x + 1][sand_y + 1] == EMPTY {
                sand_x += 1;
                sand_y += 1;
                continue 'sand;
            }

            // Settle down
            map[sand_x][sand_y] = SAND;
            sand_units_that_came_to_rest += 1;
            break 'sand; // a.k.a. `continue 'simulation;`
        }
    }

    (map, sand_units_that_came_to_rest)
}

fn add_floor(map: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut map = map.clone();

    for x in 0..map.len() {
        map[x].push(EMPTY);
        map[x].push(ROCK);
    }

    map
}

fn simulate_sand_with_endless_floor(
    map: &Vec<Vec<char>>,
    sand_source: (usize, usize),
) -> (Vec<Vec<char>>, u64) {
    // Add padding columns to allow for sand to fill gaps at edges of map
    let y_len = map[0].len();
    let mut new_map = vec![vec![EMPTY; y_len]];
    new_map.append(&mut map.clone());
    new_map.push(vec![EMPTY; y_len]);

    // Adjust sand source position, moved due to added padding columns
    let sand_source = (sand_source.0 + 1, sand_source.1);

    // Add floor
    let mut map = add_floor(&new_map);

    let mut sand_units_that_came_to_rest_inside_map = 0;
    let x_max = map.len() - 1;

    'simulation: loop {
        let (mut sand_x, mut sand_y) = sand_source;

        'sand: loop {
            // Check if cell below is empty
            if map[sand_x][sand_y + 1] == EMPTY {
                sand_y += 1;
                continue 'sand;
            }

            // Check if moving left would place sand outside the map
            if sand_x == 0 {
                // Could we possibly move right?
                // TODO: could we avoid this duplication?
                if sand_x + 1 <= x_max && map[sand_x + 1][sand_y + 1] == EMPTY {
                    sand_x += 1;
                    sand_y += 1;
                    continue 'sand;
                }
                break 'sand;
            }

            // Check if bottom left is empty
            if map[sand_x - 1][sand_y + 1] == EMPTY {
                sand_x -= 1;
                sand_y += 1;
                continue 'sand;
            }

            // Check if moving right would place sand outside the map
            if sand_x == x_max {
                break 'sand;
            }

            // Check if bottom right is empty
            if map[sand_x + 1][sand_y + 1] == EMPTY {
                sand_x += 1;
                sand_y += 1;
                continue 'sand;
            }

            // Sand didn't move anymore
            break 'sand;
        }

        // Settle down
        map[sand_x][sand_y] = SAND;
        sand_units_that_came_to_rest_inside_map += 1;

        // Check if sand settled down at sand source
        if sand_x == sand_source.0 && sand_y == sand_source.1 {
            break 'simulation;
        }
    }

    // Check how high did sand get on first and last columns of map - the sand
    // outside the map should make two big triangles with sum(1..column height)
    // (exclusive, as column inside map will already be counted) units of sand
    let left_col_sand_height = map[0].iter().position(|c| *c == SAND).unwrap();
    let right_col_sand_height = map[map.len() - 1].iter().position(|c| *c == SAND).unwrap();

    let left_sand = (1..left_col_sand_height as u64).sum::<u64>();
    let right_sand = (1..right_col_sand_height as u64).sum::<u64>();

    (
        map,
        sand_units_that_came_to_rest_inside_map + left_sand + right_sand,
    )
}

fn main() -> Result<(), anyhow::Error> {
    let input_file_path = get_arg(1).context("pass path to input file as first argument")?;
    let input_string = read_file_to_string(&input_file_path)?;
    let Problem {
        map, sand_source, ..
    } = input_string.parse()?;

    print_map(&map);

    println!("-----------------------------------------------------------------------");

    let (filled_map, pt1_sand_count) = simulate_sand(&map, sand_source);

    print_map(&filled_map);

    let (filled_map, pt2_sand_count) = simulate_sand_with_endless_floor(&map, sand_source);

    print_map(&filled_map);

    println!("Part 1 solution: {}", pt1_sand_count);
    println!("Part 2 solution: {}", pt2_sand_count);

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
            sand_source,
        } = TEST_INPUT.parse().unwrap();

        assert_eq!(sand_source, (6, 0));
        assert_eq!(map.len(), 10);
        assert_eq!(map[0].len(), 10);
    }

    #[test]
    fn test_simulate_sand() {
        let Problem {
            map, sand_source, ..
        } = TEST_INPUT.parse().unwrap();
        let sand_count = simulate_sand(&map, sand_source).1;

        assert_eq!(sand_count, 24);
    }

    #[test]
    fn test_add_floor() {
        let Problem { map, .. } = TEST_INPUT.parse().unwrap();
        let map = add_floor(&map);

        assert_eq!(map.len(), 10); // unchanged
        assert_eq!(map[0].len(), 12);
    }

    #[test]
    fn test_simulate_sand_with_endless_floor() {
        let Problem {
            map, sand_source, ..
        } = TEST_INPUT.parse().unwrap();
        let (map, sand_count) = simulate_sand_with_endless_floor(&map, sand_source);

        print_map(&map);

        assert_eq!(sand_count, 93);
    }
}
