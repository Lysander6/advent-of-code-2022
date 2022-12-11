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
        // Ignore header with index, as monkeys are passed in order, starting
        // from index 0 anyway
        let (_index, rest) = s
            .split_once('\n')
            .ok_or_else(|| anyhow!("couldn't split '{}' at newline", s))?;

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

fn do_your_business(monkeys: &mut [Monkey], rounds: usize, worry_decay: bool) -> Vec<u64> {
    let mut inspected_items = vec![0u64; monkeys.len()];

    for _ in 0..rounds {
        for i in 0..monkeys.len() {
            let monkey = &mut monkeys[i];

            let mut items_to_throw: Vec<(usize, i32)> = vec![];

            while let Some(item) = monkey.pick_for_inspection() {
                inspected_items[i] += 1;

                // Monkey inspects item
                let item = monkey.operation.execute(item);

                // Worry level shrinks after inspection
                let item = if worry_decay { item / 3 } else { item };

                // Throw item to another monkey
                let monkey_idx_to_throw_item_to = monkey.test.do_test(item);
                items_to_throw.push((monkey_idx_to_throw_item_to, item));
            }

            // Distribute thrown items
            for (idx, item) in items_to_throw {
                monkeys[idx].receive(item);
            }
        }
    }

    inspected_items
}

fn score_monkey_business(inspected_items: &[u64]) -> u64 {
    let mut inspected_items = inspected_items.to_owned();
    inspected_items.sort_by(|a, b| b.cmp(a));

    inspected_items
        .into_iter()
        .take(2)
        .reduce(|a, b| a * b)
        .unwrap_or(0)
}

fn main() -> Result<(), anyhow::Error> {
    let input_file_path = get_arg(1).context("pass path to input file as first argument")?;
    let input_string = read_file_to_string(&input_file_path)?;

    let Problem { mut monkeys } = input_string.parse()?;
    let inspected_items = do_your_business(&mut monkeys, 20, true);

    println!(
        "Part 1 solution: {}",
        score_monkey_business(&inspected_items)
    );
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

    #[test]
    fn test_do_your_business() {
        let Problem { mut monkeys } = TEST_INPUT.parse().unwrap();
        do_your_business(&mut monkeys, 20, true);

        assert_eq!(monkeys[0].items, vec![10, 12, 14, 26, 34]);
        assert_eq!(monkeys[1].items, vec![245, 93, 53, 199, 115]);
        assert_eq!(monkeys[2].items, vec![]);
        assert_eq!(monkeys[3].items, vec![]);
    }

    #[test]
    fn test_score_monkey_business() {
        let inspected_items = vec![101, 95, 7, 105];
        let score = score_monkey_business(&inspected_items);

        assert_eq!(score, 10605);
    }
}
