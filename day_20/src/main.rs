use std::collections::VecDeque;

use anyhow::{anyhow, Context};
use common::{get_arg, read_file_to_string};

fn parse_input(s: &str) -> Result<Vec<i64>, anyhow::Error> {
    s.lines()
        .map(|n| {
            n.parse::<i64>()
                .with_context(|| format!("Parsing i64 from '{}'", n))
        })
        .collect()
}

fn mix_numbers(ns: &Vec<i64>) -> Vec<i64> {
    let len = ns.len();

    let mut q = ns.into_iter().map(|&n| (n, false)).collect::<VecDeque<_>>();

    let mut left_to_mix = len;
    let mut i = 0;

    while left_to_mix > 0 {
        let item = q[i];

        if item.1 == true {
            // Item was already moved once
            i += 1;
            continue;
        }

        q.remove(i);
        let mut new_idx = ((item.0).rem_euclid(len as i64 - 1) as usize + i).rem_euclid(len - 1);
        if new_idx == 0 {
            new_idx = len - 1;
        }

        q.insert(new_idx, (item.0, true));
        left_to_mix -= 1;
    }

    q.into_iter().map(|a| a.0).collect()
}

fn find_grove_coords(ns: &Vec<i64>) -> Option<(i64, i64, i64)> {
    let Some(zero_idx) = ns.iter().position(|&n| n == 0) else {
        return None;
    };

    Some((
        ns[(zero_idx + 1000) % ns.len()],
        ns[(zero_idx + 2000) % ns.len()],
        ns[(zero_idx + 3000) % ns.len()],
    ))
}

fn main() -> Result<(), anyhow::Error> {
    let input_file_path = get_arg(1).context("pass path to input file as first argument")?;
    let input_string = read_file_to_string(&input_file_path)?;
    let ns = parse_input(&input_string)?;
    let mixed = mix_numbers(&ns);
    let grove_coords = find_grove_coords(&ns)
        .ok_or_else(|| anyhow!("Couldn't find grove coordinates from '{:?}'", mixed))?;

    println!(
        "Part 1 solution: {}",
        grove_coords.0 + grove_coords.1 + grove_coords.2
    );
    println!("Part 2 solution: {}", 0);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "\
1
2
-3
3
-2
0
4";

    #[test]
    fn test_parse_input() {
        let ns = parse_input(&TEST_INPUT).unwrap();

        assert_eq!(ns, vec![1, 2, -3, 3, -2, 0, 4]);
    }

    #[test]
    fn test_rem_euclid() {
        assert_eq!((-7i64).rem_euclid(50), 43);
    }

    #[test]
    fn test_mix_numbers() {
        let ns = parse_input(&TEST_INPUT).unwrap();
        let result = mix_numbers(&ns);

        assert_eq!(result, vec![1, 2, -3, 4, 0, 3, -2]);
    }

    #[test]
    fn test_find_grove_coords() {
        let ns = vec![1, 2, -3, 4, 0, 3, -2];
        let result = find_grove_coords(&ns);

        assert_eq!(result, Some((4, -3, 2)));
    }
}
