use std::{collections::HashMap, str::FromStr};

use anyhow::{anyhow, bail, Context};
use common::{get_arg, read_file_to_string};

#[derive(Debug, PartialEq)]
enum Monkey {
    Value(i64),
    Operation(String, char, String),
}

impl FromStr for Monkey {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (_name, rest) = s
            .split_once(": ")
            .ok_or_else(|| anyhow!("Couldn't split at ': ': '{}'", s))?;

        let op_parts = rest.split(' ').collect::<Vec<_>>();

        if op_parts.len() == 1 {
            return Ok(Self::Value(op_parts[0].parse()?));
        } else if op_parts.len() == 3 {
            return Ok(Self::Operation(
                op_parts[0].to_string(),
                op_parts[1].parse()?,
                op_parts[2].to_string(),
            ));
        }

        bail!("Got unexpected number of monkey parameters");
    }
}

struct Problem {
    monkeys: HashMap<String, Monkey>,
}

impl FromStr for Problem {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let monkeys = s
            .lines()
            .map(|line| {
                let (name, _) = line
                    .split_once(": ")
                    .ok_or_else(|| anyhow!("Couldn't split at ': ': '{}'", line))?;

                Ok::<(String, Monkey), Self::Err>((name.to_string(), line.parse()?))
            })
            .collect::<Result<HashMap<String, Monkey>, _>>()?;

        Ok(Problem { monkeys })
    }
}

fn eval_monkey(monkey_name: &str, monkeys: &HashMap<String, Monkey>) -> Result<i64, anyhow::Error> {
    match &monkeys[monkey_name] {
        Monkey::Value(v) => Ok(*v),
        Monkey::Operation(left, op, right) => {
            let left = eval_monkey(&left, monkeys)?;
            let right = eval_monkey(&right, monkeys)?;

            match op {
                '+' => Ok(left + right),
                '-' => Ok(left - right),
                '*' => Ok(left * right),
                '/' => Ok(left / right),
                _ => bail!("Unknown operation '{}'", op),
            }
        }
    }
}

fn main() -> Result<(), anyhow::Error> {
    let input_file_path = get_arg(1).context("pass path to input file as first argument")?;
    let input_string = read_file_to_string(&input_file_path)?;
    let Problem { monkeys } = input_string.parse()?;

    println!("Part 1 solution: {}", eval_monkey("root", &monkeys)?);
    println!("Part 2 solution: {}", 0);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "\
root: pppw + sjmn
dbpl: 5
cczh: sllz + lgvd
zczc: 2
ptdq: humn - dvpt
dvpt: 3
lfqf: 4
humn: 5
ljgn: 2
sjmn: drzm * dbpl
sllz: 4
pppw: cczh / lfqf
lgvd: ljgn * ptdq
drzm: hmdt - zczc
hmdt: 32";

    #[test]
    fn test_input_parsing() {
        let Problem { monkeys } = TEST_INPUT.parse().unwrap();

        assert_eq!(monkeys.len(), 15);
        assert_eq!(
            monkeys["root"],
            Monkey::Operation("pppw".to_string(), '+', "sjmn".to_string())
        );
        assert_eq!(monkeys["sllz"], Monkey::Value(4));
    }

    #[test]
    fn test_eval_monkey_1() {
        let Problem { monkeys } = TEST_INPUT.parse().unwrap();

        assert_eq!(eval_monkey("humn", &monkeys).unwrap(), 5);
    }

    #[test]
    fn test_eval_monkey_2() {
        let Problem { monkeys } = TEST_INPUT.parse().unwrap();

        assert_eq!(eval_monkey("root", &monkeys).unwrap(), 152);
    }
}
