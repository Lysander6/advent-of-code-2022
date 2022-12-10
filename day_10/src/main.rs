use anyhow::{anyhow, bail, Context};
use common::{get_arg, read_file_to_string};

fn execute<'a>(
    instructions: impl IntoIterator<Item = &'a str>,
) -> Result<Vec<(u64, i32)>, anyhow::Error> {
    let mut register_x = 1i32;
    let mut cycle = 0u64;
    let mut signal_strengths = vec![];

    for inst in instructions {
        if (17 < cycle && cycle < 20)
            || (57 < cycle && cycle < 60)
            || (97 < cycle && cycle < 100)
            || (137 < cycle && cycle < 140)
            || (177 < cycle && cycle < 180)
            || (217 < cycle && cycle < 220)
        {
            signal_strengths.push((cycle, register_x));
        }

        match &inst[..4] {
            "noop" => {
                cycle += 1;
            }
            "addx" => {
                cycle += 2;
                let (_, v) = inst
                    .split_once(' ')
                    .ok_or_else(|| anyhow!("couldn't split '{}'", inst))?;

                let v = v
                    .parse::<i32>()
                    .with_context(|| format!("parsing '{}'", v))?;

                register_x += v;
            }
            _ => {
                bail!("unknown instruction '{}'", inst);
            }
        }
    }

    Ok(signal_strengths)
}

fn calculate_score(register_history: &[(u64, i32)]) -> Result<i32, anyhow::Error> {
    let mut score = 0;

    for cycle in [20u64, 60, 100, 140, 180, 220] {
        let &(_, register_value) = register_history
            .iter()
            .take_while(|(c, _)| *c < cycle)
            .last()
            .ok_or_else(|| anyhow!("couldn't find record for cycle '{}'", cycle))?;

        score += (cycle as i32) * register_value;
    }

    Ok(score)
}

fn main() -> Result<(), anyhow::Error> {
    let input_file_path = get_arg(1).context("pass path to input file as first argument")?;
    let input_string = read_file_to_string(&input_file_path)?;

    let register_history = execute(input_string.lines())?;

    println!("Part 1 solution: {}", calculate_score(&register_history)?);
    println!("Part 2 solution: {}", 0);

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
        let r = execute(TEST_INPUT.lines()).unwrap();
        let score = calculate_score(&r).unwrap();

        assert_eq!(score, 13140);
    }
}
