use std::str::FromStr;

use anyhow::{anyhow, bail, Context};
use common::{get_arg, read_file_to_string};

#[derive(Debug)]
enum Instruction {
    Noop,
    Addx(i32),
}

impl FromStr for Instruction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match &s[..4] {
            "noop" => Ok(Self::Noop),
            "addx" => {
                let (_, v) = s
                    .split_once(' ')
                    .ok_or_else(|| anyhow!("couldn't split '{}'", s))?;

                let v = v
                    .parse::<i32>()
                    .with_context(|| format!("parsing '{}'", v))?;

                Ok(Self::Addx(v))
            }
            _ => {
                bail!("unknown instruction '{}'", s);
            }
        }
    }
}

#[derive(Debug)]
struct Problem {
    instructions: Vec<Instruction>,
}

impl FromStr for Problem {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let instructions = s.lines().map(|l| l.parse()).collect::<Result<_, _>>()?;

        Ok(Problem { instructions })
    }
}

fn execute<'a>(instructions: impl IntoIterator<Item = &'a Instruction>) -> Vec<i32> {
    let mut register_x = 1i32;
    let mut register_history = vec![register_x];

    for inst in instructions {
        register_history.push(register_x);

        match inst {
            Instruction::Noop => {}
            Instruction::Addx(v) => {
                register_history.push(register_x);
                register_x += v;
            }
        }
    }

    register_history
}

fn calculate_score(register_history: &[i32]) -> Result<i32, anyhow::Error> {
    let mut score = 0;

    for cycle in [20i32, 60, 100, 140, 180, 220] {
        score += cycle * register_history[cycle as usize];
    }

    Ok(score)
}

fn print_crt(register_history: &[i32]) {
    for (i, x) in register_history[1..].iter().enumerate() {
        let i = (i as i32) % 40;
        if i == 0 {
            print!("\n")
        }
        if i - 2 < *x && *x < i + 2 {
            print!("#");
        } else {
            print!(".");
        }
    }
}

fn main() -> Result<(), anyhow::Error> {
    let input_file_path = get_arg(1).context("pass path to input file as first argument")?;
    let input_string = read_file_to_string(&input_file_path)?;
    let Problem { instructions } = input_string.parse()?;

    let register_history = execute(&instructions);

    println!("Part 1 solution: {}", calculate_score(&register_history)?);
    println!("Part 2 solution:");

    print_crt(&register_history);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "\
addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";

    #[test]
    fn test_execute_1() {
        let Problem { instructions } = TEST_INPUT.parse().unwrap();
        let register_history = execute(&instructions);
        let score = calculate_score(&register_history).unwrap();

        assert_eq!(score, 13140);
    }
}
