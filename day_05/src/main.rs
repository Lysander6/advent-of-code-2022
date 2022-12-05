use std::str::FromStr;

use anyhow::{anyhow, Context};
use common::{get_arg, read_file_to_string};

#[derive(Debug, PartialEq)]
struct Instruction {
    num: usize,
    from: usize,
    to: usize,
}

impl FromStr for Instruction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split_whitespace().collect::<Vec<_>>();
        let [_, num, _, from, _, to]: [&str; 6] = parts
            .try_into()
            .map_err(|_v| anyhow!("couldn't split '{}' into expected six parts", s))?;

        Ok(Instruction {
            num: num.parse()?,
            from: from.parse()?,
            to: to.parse()?,
        })
    }
}

#[derive(Debug)]
struct Problem {
    stacks: Vec<Vec<char>>,
    instructions: Vec<Instruction>,
}

impl FromStr for Problem {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (stacks_raw, instructions_raw) = s
            .split_once("\n\n")
            .ok_or_else(|| anyhow!("expected input separated by single empty line"))?;

        // Process stacks from the bottom-up
        let mut stacks_iter = stacks_raw.lines().rev();

        // Use first (bottom-most) stacks description line (labels) to determine
        // their count
        let stacks_count = stacks_iter
            .next()
            .ok_or_else(|| anyhow!("empty stacks description"))?
            .split_whitespace()
            .count();

        // Allocate `stacks_count + 1` vectors to accommodate `stacks_count`
        // 1-indexed stacks and a dummy 0-th stack, to not bother with
        // re-indexing stacks. 0-th stack will simply never be touched when
        // executing instructions, and is an issue only when reading tops of
        // stacks to retrieve solution - a nuisance we can live with
        let mut stacks = vec![vec![]; stacks_count + 1];

        // Consider rest of stacks description input in (at most) 4-character
        // long chunks. We will then encounter three kinds of input: (1) `[X] `
        // - an item on stack, (2) `    ` - no item, (3) `[X]` - an item on
        // stack in last column of input (note no space at the end), same as (1)
        // for our purposes.
        for stack_line in stacks_iter {
            let chars = stack_line.chars().collect::<Vec<_>>();
            for (stack_number, chunk) in chars.chunks(4).enumerate() {
                if let Some(&c) = chunk.get(1) {
                    if c != ' ' {
                        stacks[stack_number + 1].push(c);
                    }
                }
            }
        }

        let instructions = instructions_raw
            .lines()
            .map(|l| {
                l.parse()
                    .with_context(|| format!("parsing Instruction from '{}'", l))
            })
            .collect::<Result<_, _>>()?;

        Ok(Problem {
            stacks,
            instructions,
        })
    }
}

/// Executes instructions on stacks using "single-item pick up" interpretation
fn run_instructions_with_single_pick_up(
    stacks: &Vec<Vec<char>>,
    instructions: &Vec<Instruction>,
) -> Result<Vec<Vec<char>>, anyhow::Error> {
    let mut stacks = stacks.clone();

    for instruction in instructions {
        let Instruction { num, from, to } = *instruction;

        // Move items one by one. Once could re-use loop-less solution from
        // [`run_instructions_with_multi_pick_up`], by just reversing the order
        // of items returned by [`std::vec::Vec::drain`], but this way the
        // intention is much clearer
        for _ in 0..num {
            let v = stacks[from]
                .pop()
                .ok_or_else(|| anyhow!("no items left on stack {}", from))?;
            stacks[to].push(v);
        }
    }

    Ok(stacks)
}

/// Executes instructions on stacks using "multi-item pick up" interpretation
fn run_instructions_with_multi_pick_up(
    stacks: &Vec<Vec<char>>,
    instructions: &Vec<Instruction>,
) -> Result<Vec<Vec<char>>, anyhow::Error> {
    let mut stacks = stacks.clone();

    for instruction in instructions {
        let Instruction { num, from, to } = *instruction;

        // move `num` items from the end of stack at once
        let idx_to_pick_up_from = stacks[from]
            .len()
            .checked_sub(num)
            .ok_or_else(|| anyhow!("stack {} has less than {} items left on it", from, num))?;
        let mut v = stacks[from].drain(idx_to_pick_up_from..).collect();
        stacks[to].append(&mut v);
    }

    Ok(stacks)
}

fn read_tops_of_stacks(stacks: &Vec<Vec<char>>) -> String {
    stacks
        .iter()
        .map(|stack| stack.last().unwrap_or(&' '))
        .collect()
}

fn main() -> Result<(), anyhow::Error> {
    let input_file_path = get_arg(1).context("pass path to input file as first argument")?;
    let input_string = read_file_to_string(&input_file_path)?;
    let Problem {
        stacks,
        instructions,
    } = input_string.parse()?;

    println!(
        "Part 1 solution: {}",
        read_tops_of_stacks(&run_instructions_with_single_pick_up(
            &stacks,
            &instructions
        )?)
        .trim()
    );
    println!(
        "Part 2 solution: {}",
        read_tops_of_stacks(&run_instructions_with_multi_pick_up(
            &stacks,
            &instructions
        )?)
        .trim()
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "    [D]
[N] [C]
[Z] [M] [P]
 1   2   3

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

    #[test]
    fn test_problem_parsing() {
        let Problem {
            stacks,
            instructions,
        } = TEST_INPUT.parse().unwrap();

        assert_eq!(
            stacks,
            vec![vec![], vec!['Z', 'N'], vec!['M', 'C', 'D'], vec!['P']]
        );
        assert_eq!(
            instructions,
            vec![
                Instruction {
                    num: 1,
                    from: 2,
                    to: 1
                },
                Instruction {
                    num: 3,
                    from: 1,
                    to: 3
                },
                Instruction {
                    num: 2,
                    from: 2,
                    to: 1
                },
                Instruction {
                    num: 1,
                    from: 1,
                    to: 2
                }
            ]
        );
    }

    #[test]
    fn test_run_instructions_with_single_pick_up() {
        let Problem {
            stacks,
            instructions,
        } = TEST_INPUT.parse().unwrap();

        let stacks = run_instructions_with_single_pick_up(&stacks, &instructions).unwrap();

        assert_eq!(
            stacks,
            vec![vec![], vec!['C'], vec!['M'], vec!['P', 'D', 'N', 'Z']]
        );
    }

    #[test]
    fn test_run_instructions_with_multi_pick_up() {
        let Problem {
            stacks,
            instructions,
        } = TEST_INPUT.parse().unwrap();

        let stacks = run_instructions_with_multi_pick_up(&stacks, &instructions).unwrap();

        assert_eq!(
            stacks,
            vec![vec![], vec!['M'], vec!['C'], vec!['P', 'Z', 'N', 'D']]
        );
    }

    #[test]
    fn test_read_tops_of_stacks() {
        let stacks = vec![vec![], vec!['C'], vec!['M'], vec!['P', 'D', 'N', 'Z']];

        assert_eq!(read_tops_of_stacks(&stacks), " CMZ".to_string())
    }
}
