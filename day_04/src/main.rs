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

/// Checks if either range is fully contained in the other (start- and
/// end-points inclusive).
fn assignments_fully_overlap(assignment_pair: &AssignmentPair) -> bool {
    let ([fst_start, fst_end], [snd_start, snd_end]) = assignment_pair;

    (fst_start <= snd_start && snd_end <= fst_end) || (snd_start <= fst_start && fst_end <= snd_end)
}

/// Checks if any range limit falls within limits of the other range (start- and
/// end-points inclusive).
///
/// When ranges overlap, they do so in one of three ways:
///
/// ```text
///     ↓¹
/// ....A--B..
/// ..C--D....
///
/// ..A--B....
/// ....C--D..
///     ↑₂
///
///    ↓³
/// ...A--B...
/// ...C--D...
/// ```
///
/// Therefore, ranges overlap if at least one of the following is true: (1)
/// start-point of the first range falls within second range, (2) start-point of
/// the second range falls within first range. Third case folds into either
/// first or second, when ranges are inclusive.
fn assignments_partially_overlap(assignment_pair: &AssignmentPair) -> bool {
    let ([fst_start, fst_end], [snd_start, snd_end]) = assignment_pair;

    (snd_start <= fst_start && fst_start <= snd_end)
        || (fst_start <= snd_start && snd_start <= fst_end)
}

fn count_fully_overlapping_assignments(assignment_pairs: &[AssignmentPair]) -> usize {
    assignment_pairs
        .iter()
        .filter(|a| assignments_fully_overlap(a))
        .count()
}

fn count_partially_overlapping_assignments(assignment_pairs: &[AssignmentPair]) -> usize {
    assignment_pairs
        .iter()
        .filter(|a| assignments_partially_overlap(a))
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
    println!(
        "Part 2 solution: {}",
        count_partially_overlapping_assignments(&assignment_pairs)
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

    #[test]
    fn test_count_partially_overlapping_assignments() {
        let Problem { assignment_pairs } = TEST_INPUT.parse().unwrap();

        assert_eq!(
            count_partially_overlapping_assignments(&assignment_pairs),
            4
        );
    }
}
