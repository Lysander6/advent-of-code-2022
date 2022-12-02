use std::str::FromStr;

use anyhow::{anyhow, bail, Context};
use common::{get_arg, read_file_to_string};

#[derive(Clone, Debug, PartialEq)]
enum Shape {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl FromStr for Shape {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(Shape::Rock),
            "B" | "Y" => Ok(Shape::Paper),
            "C" | "Z" => Ok(Shape::Scissors),
            other => bail!("unknown shape: {}", other),
        }
    }
}

fn parse_game(s: &str) -> Result<(Shape, Shape), anyhow::Error> {
    let (left, right) = s
        .split_once(' ')
        .ok_or_else(|| anyhow!("couldn't split '{}'", s))?;

    Ok((left.parse()?, right.parse()?))
}

#[derive(Debug)]
struct Problem {
    games: Vec<(Shape, Shape)>,
}

impl FromStr for Problem {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let games = s.lines().map(parse_game).collect::<Result<_, _>>()?;

        Ok(Problem { games })
    }
}

fn score_game(opponent_shape: &Shape, player_shape: &Shape) -> u64 {
    let outcome_score = match (player_shape, opponent_shape) {
        (Shape::Rock, Shape::Rock) => 3,
        (Shape::Rock, Shape::Paper) => 0,
        (Shape::Rock, Shape::Scissors) => 6,
        (Shape::Paper, Shape::Rock) => 6,
        (Shape::Paper, Shape::Paper) => 3,
        (Shape::Paper, Shape::Scissors) => 0,
        (Shape::Scissors, Shape::Rock) => 0,
        (Shape::Scissors, Shape::Paper) => 6,
        (Shape::Scissors, Shape::Scissors) => 3,
    };
    let shape_score = player_shape.clone() as u64;

    shape_score + outcome_score
}

fn main() -> Result<(), anyhow::Error> {
    let input_file_path = get_arg(1).context("pass path to input file as first argument")?;
    let input_string = read_file_to_string(&input_file_path)?;
    let Problem { games } = input_string.parse()?;

    let games_score: u64 = games
        .iter()
        .map(|(opponent_shape, player_shape)| score_game(opponent_shape, player_shape)).sum();

    println!("Part 1 solution: {}", games_score);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shape_from_string() {
        assert_eq!("A".parse::<Shape>().unwrap(), Shape::Rock);
        assert_eq!("B".parse::<Shape>().unwrap(), Shape::Paper);
        assert_eq!("C".parse::<Shape>().unwrap(), Shape::Scissors);
        assert_eq!("X".parse::<Shape>().unwrap(), Shape::Rock);
        assert_eq!("Y".parse::<Shape>().unwrap(), Shape::Paper);
        assert_eq!("Z".parse::<Shape>().unwrap(), Shape::Scissors);
    }

    #[test]
    fn test_score_game() {
        assert_eq!(score_game(Shape::Rock, Shape::Paper), 8);
        assert_eq!(score_game(Shape::Paper, Shape::Rock), 1);
        assert_eq!(score_game(Shape::Scissors, Shape::Scissors), 6);
    }

    const TEST_INPUT: &str = "\
A Y
B X
C Z";

    #[test]
    fn test_parse_problem() {
        let Problem { games } = TEST_INPUT.parse().unwrap();

        assert_eq!(
            games,
            vec![
                (Shape::Rock, Shape::Paper),
                (Shape::Paper, Shape::Rock),
                (Shape::Scissors, Shape::Scissors),
            ],
        )
    }
}
