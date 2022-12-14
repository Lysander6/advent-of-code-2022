use std::{
    collections::{HashMap, HashSet, VecDeque},
    str::FromStr,
};

use anyhow::{anyhow, Context};
use common::{get_arg, read_file_to_string};
use itertools::Itertools;
use rand::{seq::SliceRandom, Rng};
use rayon::prelude::*;

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
        // Wasteful, but is executed once and simplifies creation of properly
        // sized adjacency lists, etc.
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

fn compute_shortest_paths(adjacency_lists: &Vec<Vec<usize>>) -> Vec<Vec<Option<u32>>> {
    let number_of_nodes = adjacency_lists.len();
    let mut path_lengths = vec![vec![None; number_of_nodes]; number_of_nodes];

    for source_node in 0..number_of_nodes {
        let mut q = VecDeque::from([(source_node, 0u32)]);
        while let Some((node, path_len)) = q.pop_front() {
            path_lengths[source_node][node] = Some(path_len);

            for &adjacent_node in &adjacency_lists[node] {
                if path_lengths[source_node][adjacent_node] == None {
                    q.push_back((adjacent_node, path_len + 1));
                }
            }
        }
    }

    path_lengths
}

fn bruteforce(
    shortest_paths: &Vec<Vec<Option<u32>>>,
    flow_rates: &Vec<u32>,
    start_node: usize,
) -> u32 {
    let number_of_nodes = shortest_paths.len();
    let unopened_valves: HashSet<usize> =
        HashSet::from_iter((0..number_of_nodes).filter(|&node| flow_rates[node] > 0));

    let mut max_pressure_released = 0u32;

    unopened_valves
        .iter()
        .permutations(unopened_valves.len())
        .for_each(|valves| {
            let mut time_left = 30u32;
            let mut current_node = start_node;
            let mut pressure_released = 0u32;

            for &node in valves {
                if time_left == 0 {
                    break;
                }

                let t = shortest_paths[current_node][node].unwrap() + 1;
                pressure_released += time_left.saturating_sub(t) * flow_rates[node];
                time_left = time_left.saturating_sub(t);
                current_node = node;
            }

            if pressure_released > max_pressure_released {
                max_pressure_released = pressure_released;
                println!("new best: {}", max_pressure_released);
            }
        });

    max_pressure_released
}

fn find_optimal_moves(
    shortest_paths: &Vec<Vec<Option<u32>>>,
    flow_rates: &Vec<u32>,
    start_node: usize,
    valves: &HashSet<usize>,
    time_left: u32,
) -> u32 {
    let mut rng = rand::thread_rng();

    let mut time_left = time_left;
    let mut unopened_valves: HashSet<usize> = valves.clone();
    let mut current_node = start_node;

    let mut pressure_released = 0u32;

    while time_left > 0 {
        let best_candidate = unopened_valves
            .iter()
            .filter_map(|&node| {
                let flow_rate = flow_rates[node];
                let Some(distance) = shortest_paths[current_node][node] else {
                    return None;
                };

                // Compute pressure flow lost on every unopened valve for distance + 1 minutes
                let flow_lost_on_every_other_unopened_valve =
                    unopened_valves.iter().fold(0u32, |acc, &other_node| {
                        if other_node == node {
                            acc
                        } else {
                            acc + ((distance + 1 + shortest_paths[node][other_node].unwrap() + 1)
                                   / rng.gen_range(1..=30)) // 1..=30 for part 1, 1..=3 for part 2 ??\_(???)_/??
                                * flow_rates[other_node]
                        }
                    });

                // We really don't care if any of this goes below 0?
                let flow_gained = time_left.saturating_sub(distance + 1) * flow_rate;
                let score = flow_gained.saturating_sub(flow_lost_on_every_other_unopened_valve);

                Some((score, node, flow_gained))
            })
            .max_by_key(|&(v, _, _)| v);

        if best_candidate.is_none() {
            return pressure_released;
        }

        let (_, best_node, flow_gained) = best_candidate.unwrap();

        pressure_released += flow_gained;
        time_left = time_left.saturating_sub(shortest_paths[current_node][best_node].unwrap() + 1);
        unopened_valves.remove(&best_node);
        current_node = best_node;
    }

    pressure_released
}

fn part_2(
    shortest_paths: &Vec<Vec<Option<u32>>>,
    flow_rates: &Vec<u32>,
    label_to_idx: &HashMap<String, usize>,
) -> u32 {
    let mut rng = rand::thread_rng();
    let mut valves = (0..shortest_paths.len())
        .filter(|&node| flow_rates[node] > 0)
        .collect::<Vec<_>>();

    valves.shuffle(&mut rng);

    let mut best = 0;

    for s in 1..valves.len() {
        let valves = valves.clone();
        let (left, right) = valves.split_at(s);

        for _ in 0..100 {
            let pressure_released_1 = find_optimal_moves(
                &shortest_paths,
                &flow_rates,
                label_to_idx["AA"],
                &HashSet::from_iter(left.iter().cloned()),
                26,
            );
            let pressure_released_2 = find_optimal_moves(
                &shortest_paths,
                &flow_rates,
                label_to_idx["AA"],
                &HashSet::from_iter(right.iter().cloned()),
                26,
            );

            let pressure_released = pressure_released_1 + pressure_released_2;

            if pressure_released > best {
                best = pressure_released;
            }
        }
    }

    best
}

fn main() -> Result<(), anyhow::Error> {
    let input_file_path = get_arg(1).context("pass path to input file as first argument")?;
    let input_string = read_file_to_string(&input_file_path)?;
    let p: Problem = input_string.parse()?;

    let path_lengths = compute_shortest_paths(&p.adjacency_lists);
    let valves = HashSet::from_iter((0..path_lengths.len()).filter(|&node| p.flow_rates[node] > 0));

    let mut best = 0;

    // Run a couple of times
    for _ in 0..10000 {
        let result = find_optimal_moves(
            &path_lengths,
            &p.flow_rates,
            p.label_to_idx["AA"],
            &valves,
            30,
        );
        if result > best {
            best = result;
        }
    }

    println!("Part 1 solution: {}", best);

    for i in 0..1 {
        let r = (0..100)
            .into_par_iter()
            .map(|_| part_2(&path_lengths, &p.flow_rates, &p.label_to_idx))
            .max();

        let r = r.unwrap();
        if r > best {
            best = r;
            println!("new best: {}", best);
        }

        println!("done {} iterations", i * 1000);
    }

    println!("Part 2 solution: {}", best);

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

    #[test]
    fn test_compute_shortest_paths() {
        let p: Problem = TEST_INPUT.parse().unwrap();
        let path_lengths = compute_shortest_paths(&p.adjacency_lists);

        assert_eq!(
            path_lengths[p.label_to_idx["AA"]][p.label_to_idx["AA"]],
            Some(0)
        );
        assert_eq!(
            path_lengths[p.label_to_idx["AA"]][p.label_to_idx["DD"]],
            Some(1)
        );
        assert_eq!(
            path_lengths[p.label_to_idx["AA"]][p.label_to_idx["II"]],
            Some(1)
        );
        assert_eq!(
            path_lengths[p.label_to_idx["AA"]][p.label_to_idx["BB"]],
            Some(1)
        );
        assert_eq!(
            path_lengths[p.label_to_idx["AA"]][p.label_to_idx["JJ"]],
            Some(2)
        );
        assert_eq!(
            path_lengths[p.label_to_idx["AA"]][p.label_to_idx["EE"]],
            Some(2)
        );
        assert_eq!(
            path_lengths[p.label_to_idx["AA"]][p.label_to_idx["FF"]],
            Some(3)
        );
        assert_eq!(
            path_lengths[p.label_to_idx["AA"]][p.label_to_idx["GG"]],
            Some(4)
        );

        assert_eq!(
            path_lengths[p.label_to_idx["DD"]][p.label_to_idx["CC"]],
            Some(1)
        );

        assert_eq!(
            path_lengths[p.label_to_idx["FF"]][p.label_to_idx["HH"]],
            Some(2)
        );
    }

    #[test]
    fn test_find_optimal_moves() {
        let p: Problem = TEST_INPUT.parse().unwrap();
        let path_lengths = compute_shortest_paths(&p.adjacency_lists);
        let valves =
            HashSet::from_iter((0..path_lengths.len()).filter(|&node| p.flow_rates[node] > 0));

        let mut best = 0;

        for _ in 0..5000 {
            let pressure_released = find_optimal_moves(
                &path_lengths,
                &p.flow_rates,
                p.label_to_idx["AA"],
                &valves,
                30,
            );

            if pressure_released > best {
                best = pressure_released;
            }
        }

        assert_eq!(best, 1651);
    }

    #[test]
    fn test_find_optimal_moves_2() {
        let p: Problem = TEST_INPUT.parse().unwrap();
        let path_lengths = compute_shortest_paths(&p.adjacency_lists);
        let valves: HashSet<usize> =
            HashSet::from_iter((0..path_lengths.len()).filter(|&node| p.flow_rates[node] > 0));

        let mut best = 0;

        for s in 1..(valves.len() - 1) {
            let valves = valves.clone().into_iter().collect::<Vec<_>>();
            let (left, right) = valves.split_at(s);

            for _ in 0..10000 {
                let pressure_released_1 = find_optimal_moves(
                    &path_lengths,
                    &p.flow_rates,
                    p.label_to_idx["AA"],
                    &HashSet::from_iter(left.iter().cloned()),
                    26,
                );
                let pressure_released_2 = find_optimal_moves(
                    &path_lengths,
                    &p.flow_rates,
                    p.label_to_idx["AA"],
                    &HashSet::from_iter(right.iter().cloned()),
                    26,
                );

                let pressure_released = pressure_released_1 + pressure_released_2;

                if pressure_released > best {
                    best = pressure_released;
                }
            }
        }

        assert_eq!(best, 1707);
    }
}
