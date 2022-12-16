use std::{collections::HashMap, str::FromStr};

use anyhow::{anyhow, Context};
use common::{get_arg, read_file_to_string};

// TODO: Priority queue scored by time_left * flow_rate - distance_to_valve - 1
// (time to turn valve)

#[derive(Debug)]
struct Problem {
    label_to_idx: HashMap<String, usize>,
    adjacency_lists: Vec<Vec<usize>>,
    flow_rates: Vec<u32>,
    // shortest_paths: Vec<Vec<Option<u32>>>,
}

impl FromStr for Problem {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Wasteful, but is executed once and simplifies determining sizes of
        // adjacency lists, etc.
        let node_count = s.lines().count();

        let mut label_to_idx = HashMap::with_capacity(node_count);
        let mut adjacency_lists = vec![vec![]; node_count];
        let mut flow_rates = vec![0; node_count];
        // let shortest_paths = vec![vec![None; node_count]; node_count];

        let mut _next_free_idx: usize = 0;
        let mut get_next_free_index = || {
            let temp = _next_free_idx;
            _next_free_idx += 1;
            temp
        };

        for line in s.lines() {
            let line = line.trim_start_matches("Valve ");
            let (label, rest) = line
                .split_once(' ')
                .ok_or_else(|| anyhow!("couldn't split on space: '{}'", line))?;

            let label = label.to_string();
            let idx = *label_to_idx
                .entry(label)
                .or_insert_with(&mut get_next_free_index);

            let rest = rest.trim_start_matches("has flow rate=");
            let (flow_rate, rest) = rest
                .split_once(';')
                .ok_or_else(|| anyhow!("couldn't split on space: '{}'", line))?;

            flow_rates[idx] = flow_rate.parse()?;

            let rest = rest
                .trim_start_matches(" tunnels lead to valves ")
                .trim_start_matches(" tunnel leads to valve ");
            let adjacent_nodes = rest
                .split(", ")
                .map(|other| {
                    let other_label = other.to_string();
                    let other_idx = *label_to_idx
                        .entry(other_label)
                        .or_insert_with(&mut get_next_free_index);

                    other_idx
                })
                .collect::<Vec<_>>();

            adjacency_lists[idx] = adjacent_nodes;
        }

        Ok(Problem {
            label_to_idx,
            adjacency_lists,
            flow_rates,
            // shortest_paths,
        })
    }
}

fn main() -> Result<(), anyhow::Error> {
    let input_file_path = get_arg(1).context("pass path to input file as first argument")?;
    let input_string = read_file_to_string(&input_file_path)?;
    let p: Problem = input_string.parse()?;

    println!("Part 1 solution: {}", 0);
    println!("Part 2 solution: {}", 0);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "\
Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II";

    #[test]
    fn test_parsing_problem() {
        let p: Problem = TEST_INPUT.parse().unwrap();

        assert_eq!(p.label_to_idx.len(), 10);
        assert_eq!(p.label_to_idx["AA"], 0);
        assert_eq!(p.label_to_idx["JJ"], 9);

        assert_eq!(
            p.adjacency_lists[p.label_to_idx["AA"]],
            vec![
                p.label_to_idx["DD"],
                p.label_to_idx["II"],
                p.label_to_idx["BB"],
            ],
        );

        assert_eq!(
            p.adjacency_lists[p.label_to_idx["JJ"]],
            vec![p.label_to_idx["II"],],
        );

        assert_eq!(p.flow_rates[p.label_to_idx["AA"]], 0);
        assert_eq!(p.flow_rates[p.label_to_idx["BB"]], 13);
        assert_eq!(p.flow_rates[p.label_to_idx["HH"]], 22);
        assert_eq!(p.flow_rates[p.label_to_idx["JJ"]], 21);
    }
}
