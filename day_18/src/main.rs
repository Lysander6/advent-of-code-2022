use std::str::FromStr;

use anyhow::{anyhow, Context};
use common::{get_arg, read_file_to_string};

// construct 23x23x23 Boolean vec and add cubes one by one, checking if it has
// neighbors (in canonical directions) - any neighbor means -2 from visible
// sides count (one from covered side of inserted cube and one from neighboring
// cube, of which side was covered)

#[derive(Debug)]
struct Problem {
    boxes: Vec<[u8; 3]>,
}

fn get_neighbor_indices(coords: &[u8; 3]) -> [[u8; 3]; 6] {
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

fn get_surface_area(boxes: &Vec<[u8; 3]>) -> usize {
    let mut surface_area = 0i64;
    let mut grid = vec![vec![vec![false; 24]; 24]; 24];

    for b in boxes {
        println!("box: {:?}", b);
        let &[x, y, z] = b;
        grid[x as usize][y as usize][z as usize] = true;

        let neighbors = get_neighbor_indices(b)
            .into_iter()
            .filter(|&[nx, ny, nz]| grid[nx as usize][ny as usize][nz as usize])
            .count() as i64;

        surface_area += 6 - neighbors * 2;
    }

    surface_area as usize
}

impl FromStr for Problem {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let boxes = s
            .lines()
            .map(|line| {
                let a = line
                    .split(',')
                    // Shift coordinates by 1, so we don't need to mind if we
                    // are looking for neighbors of box at 0-th index
                    .map(|n| n.parse::<u8>().and_then(|n| Ok(n + 1)))
                    .collect::<Result<Vec<_>, _>>()
                    .with_context(|| format!("parsing '{}'", line))?
                    .try_into()
                    .map_err(|_v| anyhow!("couldn't split '{}' into expected three parts", line))?;

                Ok::<[u8; 3], Self::Err>(a)
            })
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Problem { boxes })
    }
}

fn main() -> Result<(), anyhow::Error> {
    let input_file_path = get_arg(1).context("pass path to input file as first argument")?;
    let input_string = read_file_to_string(&input_file_path)?;
    let Problem { boxes } = input_string.parse()?;

    println!("Part 1 solution: {}", get_surface_area(&boxes));
    println!("Part 2 solution: {}", 0);

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
        assert_eq!(boxes[0], [3, 3, 3]);
        assert_eq!(boxes[12], [3, 4, 6]);
        // assert_eq!(boxes[0], [2, 2, 2]);
        // assert_eq!(boxes[12], [2, 3, 5]);
    }

    #[test]
    fn test_get_surface_area_1() {
        let boxes = vec![[1, 1, 1], [2, 1, 1]];

        assert_eq!(get_surface_area(&boxes), 10);
    }

    #[test]
    fn test_get_surface_area_2() {
        let Problem { boxes } = TEST_INPUT.parse().unwrap();

        assert_eq!(get_surface_area(&boxes), 64);
    }
}
