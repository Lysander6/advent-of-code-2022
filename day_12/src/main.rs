use std::{collections::VecDeque, str::FromStr};

use anyhow::{anyhow, Context};
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

        // TODO: might as well store start and end point coordinates

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

fn get_adjacent_indices(x: usize, y: usize, x_dim: usize, y_dim: usize) -> Vec<(usize, usize)> {
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

/// Finds shortest path and returns vector of its indices
///
/// Note: result will contain both starting and ending points, so number of
/// steps taken to reach end from start will be one less than length of returned
/// vector
fn find_shortest_path(map: &[Vec<u8>]) -> Result<Vec<(usize, usize)>, anyhow::Error> {
    let start_coords = find_named_point(&map, START)
        .ok_or_else(|| anyhow!("couldn't find starting point coordinates"))?;
    let end_coords = find_named_point(&map, END)
        .ok_or_else(|| anyhow!("couldn't find ending point coordinates"))?;

    let x_dim = map.len();
    let y_dim = map[0].len();

    let mut path_lengths: Vec<Vec<Option<u32>>> = vec![vec![None; map[0].len()]; map.len()];

    // We start from the end point
    let mut q = VecDeque::from([(end_coords.0, end_coords.1, 0u32)]);
    path_lengths[end_coords.0][end_coords.1] = Some(0);

    while let Some((p_x, p_y, p_path_len)) = q.pop_front() {
        // We are done if we reached starting point
        if p_x == start_coords.0 && p_y == start_coords.1 {
            break;
        }

        // Get all adjacent points
        let adjacent_indices = get_adjacent_indices(p_x, p_y, x_dim, y_dim);

        // Keep only these that can access point `p`
        let p_height = map[p_x][p_y];
        let unexplored_indices_that_can_access_p = adjacent_indices
            .into_iter()
            // We can access point `p` from points not lower than `p_height - 1`
            // (so also from the ones that are higher than `p`)
            .filter(|(i_x, i_y)| {
                map[*i_x][*i_y] >= p_height - 1 && path_lengths[*i_x][*i_y] == None
            })
            .collect::<Vec<_>>();

        // Note their path lengths and queue for further examination
        for &(x, y) in unexplored_indices_that_can_access_p.iter() {
            path_lengths[x][y] = Some(p_path_len + 1);
            q.push_back((x, y, p_path_len + 1));
        }
    }

    let mut walk_point = start_coords;
    let mut shortest_path = vec![walk_point];

    while walk_point != end_coords {
        let (shortest_path_point, _) =
            get_adjacent_indices(walk_point.0, walk_point.1, x_dim, y_dim)
                .into_iter()
                .filter_map(|p| {
                    if let Some(p_length) = path_lengths[p.0][p.1] {
                        // When walking shortest path we still need to check if
                        // adjacent point is accessible from current point
                        if map[walk_point.0][walk_point.1] >= map[p.0][p.1] - 1 {
                            Some((p, p_length))
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                })
                .min_by_key(|&(_, path_length)| path_length)
                .unwrap();

        shortest_path.push(shortest_path_point);
        walk_point = shortest_path_point;
    }

    Ok(shortest_path)
}

fn main() -> Result<(), anyhow::Error> {
    let input_file_path = get_arg(1).context("pass path to input file as first argument")?;
    let input_string = read_file_to_string(&input_file_path)?;
    let Problem { map } = input_string.parse()?;

    let shortest_path = find_shortest_path(&map)?;

    println!("Part 1 solution: {}", shortest_path.len() - 1);
    println!("Part 2 solution: {}", 0);

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
        let shortest_path = find_shortest_path(&map).unwrap();

        assert_eq!(shortest_path.len() - 1, 31);
    }
}
