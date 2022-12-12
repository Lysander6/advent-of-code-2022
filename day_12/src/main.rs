use std::{collections::VecDeque, str::FromStr};

use anyhow::{anyhow, bail, Context};
use common::{get_arg, read_file_to_string};

const START: u8 = 'a' as u8 - 1;
const END: u8 = 'z' as u8 + 1;

#[derive(Debug)]
struct Problem {
    map: Vec<Vec<u8>>,
}

impl FromStr for Problem {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut map: Vec<Vec<u8>> = s.lines().map(|l| l.as_bytes().to_vec()).collect();

        // Turn 'S' and 'E' into respectively numbers one lower/higher than
        // 'a'/'z' (lowest and highest points on the map)
        let start = find_named_point(&map, 'S' as u8)
            .ok_or_else(|| anyhow!("couldn't find starting point coordinates"))?;
        map[start.0][start.1] = START;

        let end = find_named_point(&map, 'E' as u8)
            .ok_or_else(|| anyhow!("couldn't find end point coordinates"))?;
        map[end.0][end.1] = END;

        Ok(Problem { map })
    }
}

fn find_named_point(map: &[Vec<u8>], point: u8) -> Option<(usize, usize)> {
    for x in 0..map.len() {
        // Assumes every row has the same length
        for y in 0..map[0].len() {
            if map[x][y] == point {
                return Some((x, y));
            }
        }
    }

    None
}

fn get_adjacent_points(x: usize, y: usize, x_dim: usize, y_dim: usize) -> Vec<(usize, usize)> {
    let mut v = Vec::with_capacity(4);

    if x > 0 {
        v.push((x - 1, y));
    }
    if x < x_dim - 1 {
        v.push((x + 1, y));
    }
    if y > 0 {
        v.push((x, y - 1));
    }
    if y < y_dim - 1 {
        v.push((x, y + 1));
    }

    v
}

/// Finds shortest path from point labeled `start_point_label` to point labeled
/// `end_point_label` and returns number of steps needed to get to one from the
/// other (i.e. path, if it includes both ends, will have length one greater
/// than number of steps)
fn find_shortest_path(
    map: &[Vec<u8>],
    start_point_label: u8,
    end_point_label: u8,
) -> Result<u32, anyhow::Error> {
    let end_coords = find_named_point(&map, end_point_label)
        .ok_or_else(|| anyhow!("couldn't find ending point coordinates"))?;

    let x_dim = map.len();
    let y_dim = map[0].len();

    let mut path_lengths: Vec<Vec<Option<u32>>> = vec![vec![None; map[0].len()]; map.len()];

    // We start from the end point
    let mut q = VecDeque::from([(end_coords.0, end_coords.1, 0u32)]);
    path_lengths[end_coords.0][end_coords.1] = Some(0);

    while let Some((p_x, p_y, p_path_len)) = q.pop_front() {
        // We are done if we reached point with label `start_point_label`
        if map[p_x][p_y] == start_point_label {
            // We only care about length of the path, so simply return it
            return Ok(p_path_len);
        }

        // Get all adjacent points
        let adjacent_points = get_adjacent_points(p_x, p_y, x_dim, y_dim);

        // Keep only these that can access point `p`
        let p_height = map[p_x][p_y];
        let unexplored_points_that_can_access_p = adjacent_points
            .into_iter()
            // We can access point `p` from points not lower than `p_height - 1`
            // (so also from the ones that are higher than `p`)
            .filter(|(i_x, i_y)| {
                map[*i_x][*i_y] >= p_height - 1 && path_lengths[*i_x][*i_y] == None
            })
            .collect::<Vec<_>>();

        // Note their path lengths and queue for further examination
        for (x, y) in unexplored_points_that_can_access_p {
            path_lengths[x][y] = Some(p_path_len + 1);
            q.push_back((x, y, p_path_len + 1));
        }
    }

    bail!("couldn't find path to {}", start_point_label as char)
}

fn main() -> Result<(), anyhow::Error> {
    let input_file_path = get_arg(1).context("pass path to input file as first argument")?;
    let input_string = read_file_to_string(&input_file_path)?;
    let Problem { map } = input_string.parse()?;

    let shortest_path = find_shortest_path(&map, START, END)?;
    println!("Part 1 solution: {}", shortest_path);

    let shortest_path = find_shortest_path(&map, 'a' as u8, END)?;
    println!("Part 2 solution: {}", shortest_path);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "\
Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";

    #[test]
    fn test_input_parsing() {
        let Problem { map } = TEST_INPUT.parse().unwrap();

        assert_eq!(map.len(), 5);
        assert_eq!(map[0].len(), 8);
        assert_eq!(map[0][0], START);
        assert_eq!(map[2][5], END);
    }

    #[test]
    fn test_find_named_point() {
        let Problem { map } = TEST_INPUT.parse().unwrap();

        assert_eq!(find_named_point(&map, START), Some((0, 0)));
        assert_eq!(find_named_point(&map, END), Some((2, 5)));
        assert_eq!(find_named_point(&map, 255), None);
    }

    #[test]
    fn test_find_shortest_path() {
        let Problem { map } = TEST_INPUT.parse().unwrap();
        let shortest_path = find_shortest_path(&map, START, END).unwrap();

        assert_eq!(shortest_path, 31);
    }

    #[test]
    fn test_find_shortest_path_from_elevation() {
        let Problem { map } = TEST_INPUT.parse().unwrap();
        let shortest_path = find_shortest_path(&map, 'a' as u8, END).unwrap();

        assert_eq!(shortest_path, 29);
    }
}
