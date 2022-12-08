use std::str::FromStr;

use anyhow::{anyhow, Context};
use common::{get_arg, read_file_to_string};

#[derive(Debug)]
struct Problem {
    trees: Vec<Vec<u8>>,
}

impl FromStr for Problem {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let trees = s
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| {
                        c.to_digit(10)
                            .map(|d| u8::try_from(d).unwrap())
                            .ok_or_else(|| anyhow!("couldn't parse digit from '{}'", c))
                    })
                    .collect()
            })
            .collect::<Result<_, _>>()?;

        Ok(Problem { trees })
    }
}

fn visible_trees_map(trees: &Vec<Vec<u8>>) -> Vec<Vec<bool>> {
    let rows_count = trees.len();
    let columns_count = trees[0].len();

    // Use simple Boolean mask, memory is cheap
    let mut visible_trees: Vec<Vec<bool>> = vec![vec![false; columns_count]; rows_count];

    // Visible from the top
    for col in 0..columns_count {
        let mut latest_visible_tree_height = trees[0][col];
        visible_trees[0][col] = true;
        for row in 1..rows_count {
            if latest_visible_tree_height < trees[row][col] {
                visible_trees[row][col] = true;
                latest_visible_tree_height = trees[row][col];
            }
            // Do not immediately break loop, as there might be more visible
            // (higher) trees down the row, but *do* break if we reached highest
            // possible tree (9) to not perform unnecessary work
            if latest_visible_tree_height == 9 {
                break;
            }
        }
    }

    // Visible from the right
    for row in 0..rows_count {
        let mut latest_visible_tree_height = trees[row][columns_count - 1];
        visible_trees[row][columns_count - 1] = true;
        for col in (0..(columns_count - 1)).rev() {
            if latest_visible_tree_height < trees[row][col] {
                visible_trees[row][col] = true;
                latest_visible_tree_height = trees[row][col];
            }
            if latest_visible_tree_height == 9 {
                break;
            }
        }
    }

    // Visible from the bottom
    for col in 0..columns_count {
        let mut latest_visible_tree_height = trees[rows_count - 1][col];
        visible_trees[rows_count - 1][col] = true;
        for row in (0..(rows_count - 1)).rev() {
            if latest_visible_tree_height < trees[row][col] {
                visible_trees[row][col] = true;
                latest_visible_tree_height = trees[row][col];
            }
            if latest_visible_tree_height == 9 {
                break;
            }
        }
    }

    // Visible from the left
    for row in 0..rows_count {
        let mut latest_visible_tree_height = trees[row][0];
        visible_trees[row][0] = true;
        for col in 0..columns_count {
            if latest_visible_tree_height < trees[row][col] {
                visible_trees[row][col] = true;
                latest_visible_tree_height = trees[row][col];
            }
            if latest_visible_tree_height == 9 {
                break;
            }
        }
    }

    visible_trees
}

fn count_visible_trees(visible_trees: &Vec<Vec<bool>>) -> u64 {
    visible_trees
        .iter()
        .map(|v| {
            v.iter()
                .filter_map(|&b| if b { Some(1) } else { None })
                .sum::<u64>()
        })
        .sum()
}

fn main() -> Result<(), anyhow::Error> {
    let input_file_path = get_arg(1).context("pass path to input file as first argument")?;
    let input_string = read_file_to_string(&input_file_path)?;
    let Problem { trees } = input_string.parse()?;
    let visible_trees = visible_trees_map(&trees);

    println!("Part 1 solution: {}", count_visible_trees(&visible_trees));
    println!("Part 2 solution: {}", 0);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "\
30373
25512
65332
33549
35390";

    #[test]
    fn test_parsing_input() {
        let Problem { trees } = TEST_INPUT.parse().unwrap();

        assert_eq!(
            trees,
            vec![
                vec![3, 0, 3, 7, 3],
                vec![2, 5, 5, 1, 2],
                vec![6, 5, 3, 3, 2],
                vec![3, 3, 5, 4, 9],
                vec![3, 5, 3, 9, 0],
            ]
        );
    }

    #[test]
    fn test_visible_trees_map() {
        let Problem { trees } = TEST_INPUT.parse().unwrap();
        let visible_trees = visible_trees_map(&trees);

        assert_eq!(
            visible_trees,
            vec![
                vec![true, true, true, true, true],
                vec![true, true, true, false, true],
                vec![true, true, false, true, true],
                vec![true, false, true, false, true],
                vec![true, true, true, true, true],
            ]
        );
    }

    #[test]
    fn test_count_visible_trees() {
        let Problem { trees } = TEST_INPUT.parse().unwrap();
        let visible_trees = visible_trees_map(&trees);
        let count = count_visible_trees(&visible_trees);

        assert_eq!(count, 21);
    }
}
