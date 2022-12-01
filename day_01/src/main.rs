use std::str::FromStr;

use anyhow::Context;
use common::{get_arg, read_file_to_string};

#[derive(Debug)]
struct Problem {
    elven_inventories: Vec<Vec<u64>>,
}

impl FromStr for Problem {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let elven_inventories = s
            .split("\n\n")
            .map(|elven_inventory_raw| {
                elven_inventory_raw
                    .lines()
                    .map(|line| {
                        line.parse::<u64>()
                            .with_context(|| format!("couldn't parse u64 from line: {}", line))
                            .unwrap()
                    })
                    .collect()
            })
            .collect();
        Ok(Self { elven_inventories })
    }
}

fn find_max_sum(inventories: &[Vec<u64>]) -> u64 {
    let mut max_sum = 0;

    for inventory in inventories {
        let sum = inventory.iter().sum();

        if sum > max_sum {
            max_sum = sum;
        }
    }

    max_sum
}

fn main() -> Result<(), anyhow::Error> {
    let input_file_path = get_arg(1, "pass path to input file as first argument");
    let input_string = read_file_to_string(&input_file_path)?;
    let Problem { elven_inventories } = input_string.parse()?;

    println!("Part 1 solution: {}", find_max_sum(&elven_inventories));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "\
1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

    #[test]
    fn test_input_parsing() {
        let Problem { elven_inventories } = TEST_INPUT.parse().unwrap();

        assert_eq!(
            elven_inventories,
            vec![
                vec![1000, 2000, 3000],
                vec![4000],
                vec![5000, 6000],
                vec![7000, 8000, 9000],
                vec![10000]
            ]
        )
    }

    #[test]
    fn test_find_max_sum() {
        let Problem { elven_inventories } = TEST_INPUT.parse().unwrap();

        assert_eq!(find_max_sum(&elven_inventories), 24000)
    }
}
