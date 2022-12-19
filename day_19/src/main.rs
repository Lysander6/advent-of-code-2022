use std::str::FromStr;

use anyhow::{anyhow, Context};
use common::{get_arg, read_file_to_string};

#[derive(Debug)]
enum Action {
    Noop,
    BuyOreRobot,
    BuyClayRobot,
    BuyObsidianRobot,
    BuyGeodeRobot,
}

#[derive(Debug)]
struct Simulation {
    ore_robots_count: u64,
    clay_robots_count: u64,
    obsidian_robots_count: u64,
    geode_robots_count: u64,

    ore_count: u64,
    clay_count: u64,
    obsidian_count: u64,
    geode_count: u64,
}

impl Default for Simulation {
    fn default() -> Self {
        Self {
            ore_robots_count: 1,
            clay_robots_count: Default::default(),
            obsidian_robots_count: Default::default(),
            geode_robots_count: Default::default(),
            ore_count: Default::default(),
            clay_count: Default::default(),
            obsidian_count: Default::default(),
            geode_count: Default::default(),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Ore(u64);

#[derive(Debug, PartialEq, Eq)]
struct OreAndClay(u64, u64);

#[derive(Debug, PartialEq, Eq)]
struct OreAndObsidian(u64, u64);

#[derive(Debug)]
struct Blueprint {
    id: usize,
    ore_robot_cost: Ore,
    clay_robot_cost: Ore,
    obsidian_robot_cost: OreAndClay,
    geode_robot_cost: OreAndObsidian,
}

impl FromStr for Blueprint {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim_start_matches("Blueprint ");
        let (id, rest) = s
            .split_once(':')
            .ok_or_else(|| anyhow!("Couldn't split at ':': '{}'", s))?;

        let id = id
            .parse()
            .with_context(|| format!("Parsing id from '{}'", s))?;

        let rest = rest.trim_start_matches(" Each ore robot costs ");
        let (ore_robot_cost, rest) = rest
            .split_once(' ')
            .ok_or_else(|| anyhow!("Couldn't split at ' ': '{}'", rest))?;

        let ore_robot_cost = Ore(ore_robot_cost.parse()?);

        let rest = rest.trim_start_matches("ore. Each clay robot costs ");
        let (clay_robot_cost, rest) = rest
            .split_once(' ')
            .ok_or_else(|| anyhow!("Couldn't split at ' ': '{}'", rest))?;

        let clay_robot_cost = Ore(clay_robot_cost.parse()?);

        let rest = rest.trim_start_matches("ore. Each obsidian robot costs ");
        let (obsidian_robot_cost_ore, rest) = rest
            .split_once(' ')
            .ok_or_else(|| anyhow!("Couldn't split at ' ': '{}'", rest))?;

        let rest = rest.trim_start_matches("ore and ");
        let (obsidian_robot_cost_clay, rest) = rest
            .split_once(' ')
            .ok_or_else(|| anyhow!("Couldn't split at ' ': '{}'", rest))?;

        let obsidian_robot_cost = OreAndClay(
            obsidian_robot_cost_ore.parse()?,
            obsidian_robot_cost_clay.parse()?,
        );

        let rest = rest.trim_start_matches("clay. Each geode robot costs ");
        let (geode_robot_cost_ore, rest) = rest
            .split_once(' ')
            .ok_or_else(|| anyhow!("Couldn't split at ' ': '{}'", rest))?;

        let rest = rest.trim_start_matches("ore and ");
        let (geode_robot_cost_obsidian, _) = rest
            .split_once(' ')
            .ok_or_else(|| anyhow!("Couldn't split at ' ': '{}'", rest))?;

        let geode_robot_cost = OreAndObsidian(
            geode_robot_cost_ore.parse()?,
            geode_robot_cost_obsidian.parse()?,
        );

        Ok(Blueprint {
            id,
            ore_robot_cost,
            clay_robot_cost,
            obsidian_robot_cost,
            geode_robot_cost,
        })
    }
}

fn parse_blueprints(s: &str) -> Result<Vec<Blueprint>, anyhow::Error> {
    s.lines().map(|line| line.parse()).collect()
}

fn get_available_actions(blueprint: &Blueprint, simulation: &Simulation) -> Vec<Action> {
    use Action::*;

    let mut available_actions = vec![Noop];

    available_actions
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
Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.";

    #[test]
    fn test_parse_blueprints() {
        let blueprints = parse_blueprints(TEST_INPUT).unwrap();

        assert_eq!(blueprints.len(), 2);
        assert_eq!(blueprints[0].ore_robot_cost, Ore(4));
        assert_eq!(blueprints[0].clay_robot_cost, Ore(2));
        assert_eq!(blueprints[0].obsidian_robot_cost, OreAndClay(3, 14));
        assert_eq!(blueprints[0].geode_robot_cost, OreAndObsidian(2, 7));
        assert_eq!(blueprints[1].ore_robot_cost, Ore(2));
        assert_eq!(blueprints[1].clay_robot_cost, Ore(3));
        assert_eq!(blueprints[1].obsidian_robot_cost, OreAndClay(3, 8));
        assert_eq!(blueprints[1].geode_robot_cost, OreAndObsidian(3, 12));
    }
}
