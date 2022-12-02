use std::str::FromStr;

use anyhow::Context;
use common::{get_arg, read_file_to_string};

#[derive(Debug)]
struct Problem {
    elven_inventories: Vec<Vec<u64>>,
}

fn parse_inventory(s: &str) -> Result<Vec<u64>, anyhow::Error> {
    s.lines()
        .map(|line| {
            line.parse::<u64>()
                .with_context(|| format!("couldn't parse u64 from line: {}", line))
        })
        .collect()
}

impl FromStr for Problem {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let elven_inventories: Vec<Vec<u64>> = s
            .split("\n\n")
            .map(parse_inventory)
            .collect::<Result<_, _>>()?;

        Ok(Self { elven_inventories })
    }
}

fn sum_inventories(inventories: &[Vec<u64>]) -> Vec<u64> {
    inventories
        .iter()
        .map(|inv| inv.iter().sum::<u64>())
        .collect()
}

fn find_top_n(inventories: &[Vec<u64>], num: usize) -> Vec<u64> {
    let mut inventory_sums = sum_inventories(&inventories);
    inventory_sums.sort();

    inventory_sums.into_iter().rev().take(num).collect()
}

fn main() -> Result<(), anyhow::Error> {
    let input_file_path = get_arg(1).context("pass path to input file as first argument")?;
    let input_string = read_file_to_string(&input_file_path)?;
    let Problem { elven_inventories } = input_string.parse()?;

    let top_inventories = find_top_n(&elven_inventories, 3);

    println!("Part 1 solution: {}", top_inventories[0]);
    println!("Part 2 solution: {}", top_inventories.iter().sum::<u64>());

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

        assert_eq!(find_top_n(&elven_inventories, 1)[0], 24000)
    }

    #[test]
    fn test_find_top_3_sum() {
        let Problem { elven_inventories } = TEST_INPUT.parse().unwrap();

        assert_eq!(find_top_n(&elven_inventories, 3).iter().sum::<u64>(), 45000)
    }
}
