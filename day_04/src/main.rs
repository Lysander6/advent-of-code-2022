use std::str::FromStr;

use anyhow::{anyhow, Context};
use common::{get_arg, read_file_to_string};

type AssignmentPair = ([u8; 2], [u8; 2]);

#[derive(Debug)]
struct Problem {
    assignment_pairs: Vec<AssignmentPair>,
}

impl FromStr for Problem {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let assignment_pairs = s
            .lines()
            .map(|line| {
                let (fst, snd) = line
                    .split_once(',')
                    .ok_or_else(|| anyhow!("couldn't split '{}' at ','", s))?;

                let (fst_start, fst_end) = fst
                    .split_once('-')
                    .ok_or_else(|| anyhow!("couldn't split '{}' at '-'", fst))?;

                let (snd_start, snd_end) = snd
                    .split_once('-')
                    .ok_or_else(|| anyhow!("couldn't split '{}' at '-'", snd))?;

                Ok::<_, Self::Err>((
                    [fst_start.parse()?, fst_end.parse()?],
                    [snd_start.parse()?, snd_end.parse()?],
                ))
            })
            .collect::<Result<_, _>>()?;

        Ok(Problem { assignment_pairs })
    }
}

fn assignments_fully_overlap(assignment_pair: &AssignmentPair) -> bool {
    let ([fst_start, fst_end], [snd_start, snd_end]) = assignment_pair;

    (fst_start <= snd_start && snd_end <= fst_end) || (snd_start <= fst_start && fst_end <= snd_end)
}

fn count_fully_overlapping_assignments(assignment_pairs: &[AssignmentPair]) -> usize {
    assignment_pairs
        .iter()
        .filter(|a| assignments_fully_overlap(a))
        .count()
}

fn main() -> Result<(), anyhow::Error> {
    let input_file_path = get_arg(1).context("pass path to input file as first argument")?;
    let input_string = read_file_to_string(&input_file_path)?;
    let Problem { assignment_pairs } = input_string.parse()?;

    println!(
        "Part 1 solution: {}",
        count_fully_overlapping_assignments(&assignment_pairs)
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "\
2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    #[test]
    fn test_problem_parse() {
        let Problem { assignment_pairs } = TEST_INPUT.parse().unwrap();

        assert_eq!(
            assignment_pairs,
            vec![
                ([2, 4], [6, 8]),
                ([2, 3], [4, 5]),
                ([5, 7], [7, 9]),
                ([2, 8], [3, 7]),
                ([6, 6], [4, 6]),
                ([2, 6], [4, 8]),
            ]
        );
    }

    #[test]
    fn test_count_fully_overlapping_assignments() {
        let Problem { assignment_pairs } = TEST_INPUT.parse().unwrap();

        assert_eq!(count_fully_overlapping_assignments(&assignment_pairs), 2);
    }
}
