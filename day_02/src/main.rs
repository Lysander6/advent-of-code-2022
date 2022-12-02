use std::str::FromStr;

use anyhow::{anyhow, bail, Context};
use common::{get_arg, read_file_to_string};

/// Abstract representation of input, to be later type-safely interpreted as
/// either `Shape` or `GameResult`.
#[derive(Clone, Debug, PartialEq)]
enum Symbol {
    AX,
    BY,
    CZ,
}

impl FromStr for Symbol {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(Symbol::AX),
            "B" | "Y" => Ok(Symbol::BY),
            "C" | "Z" => Ok(Symbol::CZ),
            other => bail!("unknown symbol: {}", other),
        }
    }
}

/// Possible Rock-Paper-Scissors game "moves" with explicit scoring values as
/// described in the challenge.
#[derive(Clone, Debug, PartialEq)]
enum Shape {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl From<Symbol> for Shape {
    fn from(s: Symbol) -> Self {
        match s {
            Symbol::AX => Self::Rock,
            Symbol::BY => Self::Paper,
            Symbol::CZ => Self::Scissors,
        }
    }
}

#[derive(Debug, PartialEq)]
enum GameResult {
    Lose,
    Draw,
    Win,
}

impl From<Symbol> for GameResult {
    fn from(s: Symbol) -> Self {
        match s {
            Symbol::AX => Self::Lose,
            Symbol::BY => Self::Draw,
            Symbol::CZ => Self::Win,
        }
    }
}

fn parse_game(s: &str) -> Result<(Symbol, Symbol), anyhow::Error> {
    let (left, right) = s
        .split_once(' ')
        .ok_or_else(|| anyhow!("couldn't split '{}'", s))?;

    Ok((left.parse()?, right.parse()?))
}

#[derive(Debug)]
struct Problem {
    games: Vec<(Symbol, Symbol)>,
}

impl FromStr for Problem {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let games = s.lines().map(parse_game).collect::<Result<_, _>>()?;

        Ok(Problem { games })
    }
}

/// Scores a game of Rock-Paper-Scissors, awarding points for both outcome of
/// the game and `Shape` selected by the player.
fn score_game(opponent_shape: &Shape, player_shape: &Shape) -> u64 {
    use Shape::*;

    let outcome_score = match (player_shape, opponent_shape) {
        (Rock, Paper) => 0,
        (Paper, Scissors) => 0,
        (Scissors, Rock) => 0,
        (Rock, Rock) => 3,
        (Paper, Paper) => 3,
        (Scissors, Scissors) => 3,
        (Rock, Scissors) => 6,
        (Paper, Rock) => 6,
        (Scissors, Paper) => 6,
    };
    let shape_score = player_shape.clone() as u64;

    shape_score + outcome_score
}

/// Given opponent's `Shape` and desired `GameResult` returns `Shape` that, when
/// chosen by the player, will satisfy that outcome.
fn match_shape_to_desired_game_result(
    opponent_shape: &Shape,
    desired_game_result: &GameResult,
) -> Shape {
    use GameResult::*;
    use Shape::*;

    match desired_game_result {
        Lose => match opponent_shape {
            Rock => Scissors,
            Paper => Rock,
            Scissors => Paper,
        },
        Draw => opponent_shape.clone(),
        Win => match opponent_shape {
            Rock => Paper,
            Paper => Scissors,
            Scissors => Rock,
        },
    }
}

fn main() -> Result<(), anyhow::Error> {
    let input_file_path = get_arg(1).context("pass path to input file as first argument")?;
    let input_string = read_file_to_string(&input_file_path)?;
    let Problem { games } = input_string.parse()?;

    // score games using straightforward interpretation of the input
    let games_score_pt1: u64 = games
        .clone()
        .into_iter()
        .map(|(opponent_symbol, player_symbol)| {
            score_game(&opponent_symbol.into(), &player_symbol.into())
        })
        .sum();

    // score games using alternate interpretation of the input, where second
    // symbol of each pair is a desired game result
    let games_score_pt2: u64 = games
        .into_iter()
        .map(|(opponent_symbol, player_symbol)| {
            score_game(
                &opponent_symbol.clone().into(),
                &match_shape_to_desired_game_result(&opponent_symbol.into(), &player_symbol.into()),
            )
        })
        .sum();

    println!("Part 1 solution: {}", games_score_pt1);
    println!("Part 2 solution: {}", games_score_pt2);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use GameResult::*;
    use Shape::*;
    use Symbol::*;

    #[test]
    fn test_symbol_from_string() {
        assert_eq!("A".parse::<Symbol>().unwrap(), AX);
        assert_eq!("B".parse::<Symbol>().unwrap(), BY);
        assert_eq!("C".parse::<Symbol>().unwrap(), CZ);
        assert_eq!("X".parse::<Symbol>().unwrap(), AX);
        assert_eq!("Y".parse::<Symbol>().unwrap(), BY);
        assert_eq!("Z".parse::<Symbol>().unwrap(), CZ);
    }

    #[test]
    fn test_score_game() {
        assert_eq!(score_game(&Rock, &Paper), 8);
        assert_eq!(score_game(&Paper, &Rock), 1);
        assert_eq!(score_game(&Scissors, &Scissors), 6);
    }

    const TEST_INPUT: &str = "\
A Y
B X
C Z";

    #[test]
    fn test_parse_problem() {
        let Problem { games } = TEST_INPUT.parse().unwrap();

        assert_eq!(games, vec![(AX, BY), (BY, AX), (CZ, CZ)]);
    }

    #[test]
    fn test_match_shape_to_expected_game_result() {
        assert_eq!(match_shape_to_desired_game_result(&Rock, &Draw), Rock);
        assert_eq!(match_shape_to_desired_game_result(&Paper, &Lose), Rock);
        assert_eq!(match_shape_to_desired_game_result(&Scissors, &Win), Rock);
    }
}
