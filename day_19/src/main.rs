use std::str::FromStr;

use anyhow::{anyhow, Context};
use common::{get_arg, read_file_to_string};

#[derive(Debug, PartialEq, Eq)]
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

    // TODO: These should also be kept as `Ore`, `Clay`, etc. for type-safety
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

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Ore(u64);

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Clay(u64);

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Obsidian(u64);

#[derive(Debug)]
struct Blueprint {
    id: usize,
    ore_robot_cost: Ore,
    clay_robot_cost: Ore,
    obsidian_robot_cost: (Ore, Clay),
    geode_robot_cost: (Ore, Obsidian),
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

        let obsidian_robot_cost_ore = Ore(obsidian_robot_cost_ore.parse()?);

        let rest = rest.trim_start_matches("ore and ");
        let (obsidian_robot_cost_clay, rest) = rest
            .split_once(' ')
            .ok_or_else(|| anyhow!("Couldn't split at ' ': '{}'", rest))?;

        let obsidian_robot_cost_clay = Clay(obsidian_robot_cost_clay.parse()?);
        let obsidian_robot_cost = (obsidian_robot_cost_ore, obsidian_robot_cost_clay);

        let rest = rest.trim_start_matches("clay. Each geode robot costs ");
        let (geode_robot_cost_ore, rest) = rest
            .split_once(' ')
            .ok_or_else(|| anyhow!("Couldn't split at ' ': '{}'", rest))?;

        let geode_robot_cost_ore = Ore(geode_robot_cost_ore.parse()?);

        let rest = rest.trim_start_matches("ore and ");
        let (geode_robot_cost_obsidian, _) = rest
            .split_once(' ')
            .ok_or_else(|| anyhow!("Couldn't split at ' ': '{}'", rest))?;

        let geode_robot_cost_obsidian = Obsidian(geode_robot_cost_obsidian.parse()?);
        let geode_robot_cost = (geode_robot_cost_ore, geode_robot_cost_obsidian);

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

    let Blueprint {
        ref ore_robot_cost,
        ref clay_robot_cost,
        ref obsidian_robot_cost,
        ref geode_robot_cost,
        ..
    } = blueprint;

    let Simulation {
        ore_count,
        clay_count,
        obsidian_count,
        ..
    } = *simulation;

    if ore_count >= ore_robot_cost.0 {
        available_actions.push(BuyOreRobot);
    }

    if ore_count >= clay_robot_cost.0 {
        available_actions.push(BuyClayRobot);
    }

    let (obsidian_robot_cost_ore, obsidian_robot_cost_clay) = obsidian_robot_cost;
    if ore_count >= obsidian_robot_cost_ore.0 && clay_count >= obsidian_robot_cost_clay.0 {
        available_actions.push(BuyObsidianRobot);
    }

    let (geode_robot_cost_ore, geode_robot_cost_obsidian) = geode_robot_cost;
    if ore_count >= geode_robot_cost_ore.0 && obsidian_count >= geode_robot_cost_obsidian.0 {
        available_actions.push(BuyGeodeRobot);
    }

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
        assert_eq!(blueprints[0].obsidian_robot_cost, (Ore(3), Clay(14)));
        assert_eq!(blueprints[0].geode_robot_cost, (Ore(2), Obsidian(7)));
        assert_eq!(blueprints[1].ore_robot_cost, Ore(2));
        assert_eq!(blueprints[1].clay_robot_cost, Ore(3));
        assert_eq!(blueprints[1].obsidian_robot_cost, (Ore(3), Clay(8)));
        assert_eq!(blueprints[1].geode_robot_cost, (Ore(3), Obsidian(12)));
    }

    #[test]
    fn test_get_available_actions() {
        let blueprints = parse_blueprints(TEST_INPUT).unwrap();
        let actions = get_available_actions(&blueprints[0], &Simulation::default());

        assert_eq!(actions, vec![Action::Noop]);
    }
}
