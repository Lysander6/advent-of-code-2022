use std::{collections::VecDeque, str::FromStr};

use anyhow::{anyhow, Context};
use common::{get_arg, read_file_to_string};

// Construct a Boolean grid and add cubes one by one, checking if it has
// neighbors (in canonical directions) - any neighbor means -2 from visible
// sides count (one from covered side of inserted cube and one from neighboring
// cube, whose side was covered)

const GRID: usize = 25;

#[derive(Debug)]
struct Problem {
    boxes: Vec<[usize; 3]>,
}

impl FromStr for Problem {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let boxes = s
            .lines()
            .map(|line| {
                let a = line
                    .split(',')
                    // Shift coordinates, so we don't need to mind if we are
                    // looking for neighbors of box originally at 0-th index.
                    .map(|n| n.parse::<usize>().and_then(|n| Ok(n + 2)))
                    .collect::<Result<Vec<_>, _>>()
                    .with_context(|| format!("parsing '{}'", line))?
                    .try_into()
                    .map_err(|_v| anyhow!("couldn't split '{}' into expected three parts", line))?;

                Ok::<[usize; 3], Self::Err>(a)
            })
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Problem { boxes })
    }
}

fn get_neighbor_indices(coords: &[usize; 3]) -> [[usize; 3]; 6] {
    let &[x, y, z] = coords;

    [
        [x + 1, y, z],
        [x - 1, y, z],
        [x, y + 1, z],
        [x, y - 1, z],
        [x, y, z + 1],
        [x, y, z - 1],
    ]
}

fn get_surface_area(boxes: &Vec<[usize; 3]>) -> (usize, Vec<Vec<Vec<bool>>>) {
    let mut surface_area = 0i64;
    let mut grid = vec![vec![vec![false; GRID]; GRID]; GRID];

    for b in boxes {
        let &[x, y, z] = b;
        grid[x][y][z] = true;

        let neighbors = get_neighbor_indices(b)
            .into_iter()
            .filter(|&[nx, ny, nz]| grid[nx][ny][nz])
            .count() as i64;

        surface_area += 6 - neighbors * 2;
    }

    (surface_area as usize, grid)
}

fn flood_count(grid: &Vec<Vec<Vec<bool>>>) -> usize {
    let mut scheduled = vec![vec![vec![false; grid[0][0].len()]; grid[0].len()]; grid.len()];
    let mut hits = 0;

    let mut q = VecDeque::from([[1, 1, 1]]);
    scheduled[1][1][1] = true;
    while let Some(node) = q.pop_front() {
        for adjacent_node in get_neighbor_indices(&node) {
            let [nx, ny, nz] = adjacent_node;
            if 0 < nx && 0 < ny && 0 < nz && nx < GRID && ny < GRID && nz < GRID {
                if grid[nx][ny][nz] {
                    hits += 1;
                    scheduled[nx][ny][nz] = true;
                    continue;
                }

                if !scheduled[nx][ny][nz] {
                    scheduled[nx][ny][nz] = true;
                    q.push_back(adjacent_node);
                }
            }
        }
    }

    hits
}

fn main() -> Result<(), anyhow::Error> {
    let input_file_path = get_arg(1).context("pass path to input file as first argument")?;
    let input_string = read_file_to_string(&input_file_path)?;
    let Problem { boxes } = input_string.parse()?;
    let (surface_area, grid) = get_surface_area(&boxes);

    println!("Part 1 solution: {}", surface_area);
    println!("Part 2 solution: {}", flood_count(&grid));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "\
2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5";

    #[test]
    fn test_parsing_problem() {
        let Problem { boxes } = TEST_INPUT.parse().unwrap();

        assert_eq!(boxes.len(), 13);
        assert_eq!(boxes[0], [4, 4, 4]);
        assert_eq!(boxes[12], [4, 5, 7]);
        // assert_eq!(boxes[0], [2, 2, 2]);
        // assert_eq!(boxes[12], [2, 3, 5]);
    }

    #[test]
    fn test_get_surface_area_1() {
        let boxes = vec![[1, 1, 1], [2, 1, 1]];

        assert_eq!(get_surface_area(&boxes).0, 10);
    }

    #[test]
    fn test_get_surface_area_2() {
        let Problem { boxes } = TEST_INPUT.parse().unwrap();

        assert_eq!(get_surface_area(&boxes).0, 64);
    }

    #[test]
    fn test_flood_count_1() {
        let boxes = vec![[2, 2, 2], [3, 2, 2]];
        let (_, grid) = get_surface_area(&boxes);

        assert_eq!(flood_count(&grid), 10);
    }

    #[test]
    fn test_flood_count_2() {
        let Problem { boxes } = TEST_INPUT.parse().unwrap();
        let (_, grid) = get_surface_area(&boxes);

        assert_eq!(flood_count(&grid), 58);
    }
}
