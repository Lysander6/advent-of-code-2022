use std::{collections::HashSet, str::FromStr};

use anyhow::{bail, Context};
use common::{get_arg, read_file_to_string};

#[derive(Clone, Debug, PartialEq)]
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
            .with_context(|| format!("splitting '{}'", s))?;

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

impl Into<([i32; 2], usize)> for &Move {
    fn into(self) -> ([i32; 2], usize) {
        match self {
            Move::Left(times) => ([-1, 0], *times),
            Move::Right(times) => ([1, 0], *times),
            Move::Up(times) => ([0, 1], *times),
            Move::Down(times) => ([0, -1], *times),
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

fn simulate_rope(moves: &Vec<Move>, length: usize) -> HashSet<(i32, i32)> {
    let mut rope = vec![[0, 0]; length];
    let mut visited_positions = HashSet::from([(0, 0)]);

    for m in moves {
        let (v, times) = m.into();

        for _ in 0..times {
            rope[0][0] += v[0];
            rope[0][1] += v[1];

            for tail_idx in 1..rope.len() {
                let head = rope[tail_idx - 1];
                let dx: i32 = head[0] - rope[tail_idx][0];
                let dy: i32 = head[1] - rope[tail_idx][1];

                if dx.abs() > 1 || dy.abs() > 1 {
                    rope[tail_idx][0] += dx.signum();
                    rope[tail_idx][1] += dy.signum();

                    if tail_idx == rope.len() - 1 {
                        visited_positions.insert((rope[tail_idx][0], rope[tail_idx][1]));
                    }
                }
            }
        }
    }

    visited_positions
}

fn main() -> Result<(), anyhow::Error> {
    let input_file_path = get_arg(1).context("pass path to input file as first argument")?;
    let input_string = read_file_to_string(&input_file_path)?;
    let Problem { moves } = input_string.parse()?;
    let visited_positions = simulate_rope(&moves, 2);

    println!("Part 1 solution: {}", visited_positions.len());

    let visited_positions = simulate_rope(&moves, 10);
    println!("Part 2 solution: {}", visited_positions.len());

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

    const TEST_INPUT_LARGE: &str = "\
R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";

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
        let visited_positions = simulate_rope(&moves, 2);

        assert_eq!(visited_positions.len(), 13);
    }

    #[test]
    fn test_simulate_long_rope_1() {
        let Problem { moves } = TEST_INPUT.parse().unwrap();
        let visited_positions = simulate_rope(&moves, 10);

        assert_eq!(visited_positions.len(), 1);
    }

    #[test]
    fn test_simulate_long_rope_2() {
        let Problem { moves } = TEST_INPUT_LARGE.parse().unwrap();
        let visited_positions = simulate_rope(&moves, 10);

        assert_eq!(visited_positions.len(), 36);
    }
}
