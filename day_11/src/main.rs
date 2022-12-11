use std::{collections::VecDeque, str::FromStr};

use anyhow::{anyhow, bail, Context};
use common::{get_arg, read_file_to_string};

#[derive(Debug)]
enum Either<L, R> {
    Left(L),
    Right(R),
}

#[derive(Debug)]
struct Old;

#[derive(Debug)]
enum Operation {
    Add(Either<Old, i32>, Either<Old, i32>),
    Multiply(Either<Old, i32>, Either<Old, i32>),
}

impl Operation {
    fn execute(&self, old: i32) -> i32 {
        use Either::*;

        match self {
            Operation::Add(a, b) => {
                let a = match a {
                    Left(_) => old,
                    Right(r) => *r,
                };
                let b = match b {
                    Left(_) => old,
                    Right(r) => *r,
                };

                a + b
            }
            Operation::Multiply(a, b) => {
                let a = match a {
                    Left(_) => old,
                    Right(r) => *r,
                };
                let b = match b {
                    Left(_) => old,
                    Right(r) => *r,
                };

                a * b
            }
        }
    }
}

impl FromStr for Operation {
    type Err = anyhow::Error;

    // "Operation: new = old + 6"
    // "Operation: new = old * old"
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim().trim_start_matches("Operation: new = ");
        let parts = s.split_whitespace().collect::<Vec<_>>();
        let [left, operator, right]: [&str; 3] = parts
            .try_into()
            .map_err(|_v| anyhow!("couldn't split '{}' into expected three parts", s))?;

        let left = match left {
            "old" => Either::Left(Old),
            v => Either::Right(v.parse::<i32>()?),
        };

        let right = match right {
            "old" => Either::Left(Old),
            v => Either::Right(v.parse::<i32>()?),
        };

        let operation = match operator {
            "+" => Self::Add(left, right),
            "*" => Self::Multiply(left, right),
            _ => bail!("unknown operator '{}' in '{}'", operator, s),
        };

        Ok(operation)
    }
}

#[derive(Debug)]
struct MonkeyTest {
    divisibility_operand: i32,
    if_true_receiver: usize,
    if_false_receiver: usize,
}

impl MonkeyTest {
    fn do_test(&self, worry_level: i32) -> usize {
        if worry_level % self.divisibility_operand == 0 {
            self.if_true_receiver
        } else {
            self.if_false_receiver
        }
    }
}

impl FromStr for MonkeyTest {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let [test, if_true, if_false]: [&str; 3] = s
            .lines()
            .collect::<Vec<_>>()
            .try_into()
            .map_err(|_v| anyhow!("couldn't split '{}' into expected three parts", s))?;

        let divisibility_operand = test
            .trim()
            .trim_start_matches("Test: divisible by ")
            .parse::<i32>()?;

        let if_true_receiver = if_true
            .trim()
            .trim_start_matches("If true: throw to monkey ")
            .parse::<usize>()?;

        let if_false_receiver = if_false
            .trim()
            .trim_start_matches("If false: throw to monkey ")
            .parse::<usize>()?;

        Ok(Self {
            divisibility_operand,
            if_true_receiver,
            if_false_receiver,
        })
    }
}

#[derive(Debug)]
struct Monkey {
    index: usize,
    items: VecDeque<i32>,
    operation: Operation,
    test: MonkeyTest,
}

impl Monkey {
    fn receive(&mut self, item: i32) {
        self.items.push_back(item);
    }

    fn pick_for_inspection(&mut self) -> Option<i32> {
        self.items.pop_front()
    }
}

impl FromStr for Monkey {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (index, rest) = s
            .split_once('\n')
            .ok_or_else(|| anyhow!("couldn't split '{}' at newline", s))?;

        let index = index
            .trim_start_matches("Monkey ")
            .trim_end_matches(":")
            .parse::<usize>()?;

        let (items, rest) = rest
            .split_once('\n')
            .ok_or_else(|| anyhow!("couldn't split '{}' at newline", rest))?;

        let items = items
            .trim()
            .trim_start_matches("Starting items: ")
            .split(", ")
            .map(|v| v.parse::<i32>())
            .collect::<Result<VecDeque<_>, _>>()?;

        let (operation, test) = rest
            .split_once('\n')
            .ok_or_else(|| anyhow!("couldn't split '{}' at newline", rest))?;

        let operation = operation.parse()?;

        let test = test.parse()?;

        Ok(Self {
            index,
            items,
            operation,
            test,
        })
    }
}

#[derive(Debug)]
struct Problem {
    monkeys: Vec<Monkey>,
}

impl FromStr for Problem {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let monkeys = s
            .split("\n\n")
            .map(|m| m.parse())
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self { monkeys })
    }
}

fn main() -> Result<(), anyhow::Error> {
    let input_file_path = get_arg(1).context("pass path to input file as first argument")?;
    let input_string = read_file_to_string(&input_file_path)?;

    println!("Part 1 solution: {}", 0);
    println!("Part 2 solution: {}", 0);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "\
Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";

    #[test]
    fn test_parsing() {
        let Problem { monkeys } = TEST_INPUT.parse().unwrap();

        assert_eq!(monkeys.len(), 4);
    }
}
