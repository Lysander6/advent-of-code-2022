use std::str::FromStr;

use anyhow::{anyhow, Context};
use common::{get_arg, read_file_to_string};

// TODO: collect empty ranges - spreading out from sensor position to +/-
// closest beacon positions (in both coordinates). For part 1: find all these
// which contain y=2000000 in their y range, "sum" extends of their x ranges
// (note that they might not necessarily all overlap).

#[derive(Debug, PartialEq, Eq)]
struct Sensor(i32, i32);

#[derive(Debug, PartialEq, Eq)]
struct Beacon(i32, i32);

#[derive(Debug)]
struct Problem {
    reports: Vec<(Sensor, Beacon)>,
}

/// Reads string like `x=-2, y=15` and returns ordered tuple of values
fn read_named_pair(s: &str) -> Result<(&str, &str), anyhow::Error> {
    let (fst, snd) = s
        .split_once(", ")
        .ok_or_else(|| anyhow!("couldn't split at ',': '{}'", s))?;

    let (_, fst) = fst
        .split_once('=')
        .ok_or_else(|| anyhow!("couldn't split at '=': '{}'", fst))?;

    let (_, snd) = snd
        .split_once('=')
        .ok_or_else(|| anyhow!("couldn't split at '=': '{}'", snd))?;

    Ok((fst, snd))
}

impl FromStr for Problem {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let reports = s
            .lines()
            .map(|line| {
                let line = line.trim().trim_start_matches("Sensor at ");
                let (sensor_coords, beacon_coors) = line
                    .split_once(": closest beacon is at ")
                    .ok_or_else(|| anyhow!("couldn't split '{}'", line))?;

                let (sensor_x, sensor_y) = read_named_pair(sensor_coords)?;
                let (beacon_x, beacon_y) = read_named_pair(beacon_coors)?;

                let sensor = Sensor(sensor_x.parse()?, sensor_y.parse()?);
                let beacon = Beacon(beacon_x.parse()?, beacon_y.parse()?);

                Ok::<(Sensor, Beacon), Self::Err>((sensor, beacon))
            })
            .collect::<Result<_, _>>()?;

        Ok(Problem { reports })
    }
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
Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3";

    #[test]
    fn test_parsing_problem() {
        let Problem { reports } = TEST_INPUT.parse().unwrap();

        assert_eq!(reports.len(), 14);
        assert_eq!(reports[0], (Sensor(2, 18), Beacon(-2, 15)));
        assert_eq!(reports[13], (Sensor(20, 1), Beacon(15, 3)));
    }
}
