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
            .ok_or_else(|| anyhow!("expected input divided by single empty line"))?;

        let mut stacks_iter = stacks_raw.lines().rev();

        let stacks_count = stacks_iter
            .next()
            .ok_or_else(|| anyhow!("empty stacks description"))?
            .split_whitespace()
            .count();

        let mut stacks = vec![vec![]; stacks_count + 1]; // NOTE: 0-th stack

        for stack_line in stacks_iter {
            for (stack_number, chunk) in
                stack_line.chars().collect::<Vec<_>>().chunks(4).enumerate()
            {
                if let Some(c) = chunk.get(1) {
                    if *c != ' ' {
                        stacks[stack_number + 1].push(*c);
                    }
                }
            }
        }

        let instructions = instructions_raw
            .lines()
            .map(|l| {
                l.parse::<Instruction>()
                    .with_context(|| anyhow!("trying to parse Instruction from '{}'", l))
            })
            .collect::<Result<_, _>>()?;

        Ok(Problem {
            stacks,
            instructions,
        })
    }
}

fn run_instructions_on_stacks(
    stacks: &Vec<Vec<char>>,
    instructions: &Vec<Instruction>,
) -> Vec<Vec<char>> {
    let mut stacks = stacks.clone();

    for instruction in instructions {
        let Instruction { num, from, to } = instruction;

        for _ in 0..*num {
            let v = stacks[*from].pop().unwrap();
            stacks[*to].push(v);
        }
    }

    stacks
}

fn run_instructions_on_stacks_2(
    stacks: &Vec<Vec<char>>,
    instructions: &Vec<Instruction>,
) -> Vec<Vec<char>> {
    let mut stacks = stacks.clone();

    for instruction in instructions {
        let Instruction { num, from, to } = instruction;

        let from_stack_len = stacks[*from].len();
        let mut v = stacks[*from].drain((from_stack_len - num)..).collect();
        stacks[*to].append(&mut v);
    }

    stacks
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
        read_tops_of_stacks(&run_instructions_on_stacks(&stacks, &instructions)).trim()
    );
    println!(
        "Part 2 solution: {}",
        read_tops_of_stacks(&run_instructions_on_stacks_2(&stacks, &instructions)).trim()
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
    fn test_run_instructions_on_stacks() {
        let Problem {
            stacks,
            instructions,
        } = TEST_INPUT.parse().unwrap();

        let stacks = run_instructions_on_stacks(&stacks, &instructions);

        assert_eq!(
            stacks,
            vec![vec![], vec!['C'], vec!['M'], vec!['P', 'D', 'N', 'Z']]
        );
    }

    #[test]
    fn test_run_instructions_on_stacks_2() {
        let Problem {
            stacks,
            instructions,
        } = TEST_INPUT.parse().unwrap();

        let stacks = run_instructions_on_stacks_2(&stacks, &instructions);

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
