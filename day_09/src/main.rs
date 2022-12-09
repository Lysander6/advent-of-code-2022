use std::{collections::HashSet, str::FromStr};

use anyhow::{anyhow, bail, Context};
use common::{get_arg, read_file_to_string};

#[derive(Debug, PartialEq)]
enum Move {
    Left(usize),
    Right(usize),
    Up(usize),
    Down(usize),
}

impl FromStr for Move {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (direction, magnitude) = s
            .split_once(' ')
            .with_context(|| anyhow!("splitting '{}'", s))?;

        let magnitude = magnitude.parse::<usize>()?;

        match direction {
            "L" => Ok(Move::Left(magnitude)),
            "R" => Ok(Move::Right(magnitude)),
            "U" => Ok(Move::Up(magnitude)),
            "D" => Ok(Move::Down(magnitude)),
            _ => bail!("unknown direction '{}'", direction),
        }
    }
}

#[derive(Debug)]
struct Problem {
    moves: Vec<Move>,
}

impl FromStr for Problem {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let moves = s
            .lines()
            .map(|l| l.parse())
            .collect::<Result<Vec<Move>, _>>()?;

        Ok(Problem { moves })
    }
}

fn simulate_rope(moves: &Vec<Move>) -> HashSet<(i32, i32)> {
    let mut head = [0, 0];
    let mut tail = [0, 0];
    let mut visited_positions = HashSet::from([(0, 0)]);

    for m in moves {
        let (v, &times) = match m {
            Move::Left(times) => ([-1, 0], times),
            Move::Right(times) => ([1, 0], times),
            Move::Up(times) => ([0, 1], times),
            Move::Down(times) => ([0, -1], times),
        };

        for _ in 0..times {
            head[0] += v[0];
            head[1] += v[1];

            let dx: i32 = head[0] - tail[0];
            let dy: i32 = head[1] - tail[1];
            let dx_abs = dx.abs();
            let dy_abs = dy.abs();

            if dx_abs > 1 || dy_abs > 1 {
                if dx_abs > dy_abs {
                    tail[0] += v[0];
                    tail[1] = head[1];
                } else {
                    tail[0] = head[0];
                    tail[1] += v[1];
                }

                visited_positions.insert((tail[0], tail[1]));
            }
        }
    }

    visited_positions
}

fn main() -> Result<(), anyhow::Error> {
    let input_file_path = get_arg(1).context("pass path to input file as first argument")?;
    let input_string = read_file_to_string(&input_file_path)?;
    let Problem { moves } = input_string.parse()?;
    let visited_positions = simulate_rope(&moves);

    println!("Part 1 solution: {}", visited_positions.iter().count());
    println!("Part 2 solution: {}", 0);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use Move::*;

    const TEST_INPUT: &str = "\
R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

    #[test]
    fn test_parse_input() {
        let Problem { moves } = TEST_INPUT.parse().unwrap();

        assert_eq!(
            moves,
            vec![
                Right(4),
                Up(4),
                Left(3),
                Down(1),
                Right(4),
                Down(1),
                Left(5),
                Right(2)
            ]
        );
    }

    #[test]
    fn test_simulate_rope_1() {
        let Problem { moves } = TEST_INPUT.parse().unwrap();
        let visited_positions = simulate_rope(&moves);

        assert_eq!(visited_positions.iter().count(), 13);
    }
}
