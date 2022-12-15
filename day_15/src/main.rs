use std::{collections::HashSet, str::FromStr};

use anyhow::{anyhow, Context};
use common::{get_arg, read_file_to_string};

// TODO: collect empty ranges - spreading out from sensor position to +/-
// closest beacon positions (in both coordinates). For part 1: find all these
// which contain y=2000000 in their y range, "sum" extends of their x ranges
// (note that they might not necessarily all overlap) and subtract beacons. Note
// that number of cells a sensor covers will shrink by 2 for every 1 cell of
// distance from inspected row

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

fn manhattan_distance(a_x: i32, a_y: i32, b_x: i32, b_y: i32) -> i32 {
    let x_dist = (a_x - b_x).abs();
    let y_dist = (a_y - b_y).abs();

    x_dist + y_dist
}

fn find_sensor_coverage_at_row(y: i32, sensor: &Sensor, beacon: &Beacon) -> Option<(i32, i32)> {
    let covered_distance = manhattan_distance(sensor.0, sensor.1, beacon.0, beacon.1);
    let sensor_to_y_distance = manhattan_distance(sensor.0, sensor.1, sensor.0, y);

    if sensor_to_y_distance > covered_distance {
        return None;
    }

    let diff = covered_distance - sensor_to_y_distance;

    Some((sensor.0 - diff, sensor.0 + diff))
}

/// Merges inclusive ranges that either overlap or neighbor each other
fn merge_ranges(mut ranges: Vec<(i32, i32)>) -> Vec<(i32, i32)> {
    if ranges.len() <= 1 {
        return ranges;
    }

    ranges.sort_by_key(|r| r.0);

    let mut merged_ranges = vec![];
    let mut coverage = ranges[0];

    for other in ranges[1..].iter() {
        if coverage.1 >= other.0 - 1 {
            // Ranges overlap or are next to each other
            coverage = (coverage.0, i32::max(coverage.1, other.1));
        } else {
            merged_ranges.push(coverage);
            coverage = *other;
        }
    }
    merged_ranges.push(coverage);

    merged_ranges
}

fn get_covered_ranges_at_row(y: i32, reports: &Vec<(Sensor, Beacon)>) -> Vec<(i32, i32)> {
    let coverages_at_y = reports
        .iter()
        .filter_map(|(sensor, beacon)| find_sensor_coverage_at_row(y, sensor, beacon))
        .collect::<Vec<_>>();

    merge_ranges(coverages_at_y)
}

fn find_coverage_for_row(y: i32, reports: &Vec<(Sensor, Beacon)>) -> u32 {
    let beacons_at_row = reports
        .iter()
        .filter(|(_, beacon)| beacon.1 == y)
        .map(|(_, b)| b);

    let merged_coverages = get_covered_ranges_at_row(y, reports);

    let covered_cells = merged_coverages
        .iter()
        .fold(0, |acc, range| acc + (range.0 - range.1).abs() + 1) as u32;

    let beacons_at_row = beacons_at_row
        .filter_map(|b| {
            if merged_coverages.iter().any(|r| r.0 <= b.0 && b.0 <= r.1) {
                Some(b.0)
            } else {
                None
            }
        })
        .collect::<HashSet<_>>()
        .into_iter()
        .count() as u32;

    covered_cells - beacons_at_row
}

fn find_distress_beacons_signal(
    range_start: i32,
    range_end: i32,
    reports: &Vec<(Sensor, Beacon)>,
) -> Option<u64> {
    for y in range_start..=range_end {
        let ranges = get_covered_ranges_at_row(y, reports);
        if ranges.len() == 2 {
            return Some(4000000 * (ranges[0].1 as u64 + 1) + y as u64);
        }
    }

    None
}

fn main() -> Result<(), anyhow::Error> {
    let input_file_path = get_arg(1).context("pass path to input file as first argument")?;
    let input_string = read_file_to_string(&input_file_path)?;

    let Problem { reports } = input_string.parse()?;

    println!(
        "Part 1 solution: {}",
        find_coverage_for_row(2000000, &reports)
    );
    println!(
        "Part 2 solution: {}",
        find_distress_beacons_signal(0, 4000000, &reports).unwrap()
    );

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

    #[test]
    fn test_find_sensor_coverage_at_row() {
        let s = Sensor(8, 7);
        let b = Beacon(2, 10);
        let y = 4;

        let result = find_sensor_coverage_at_row(y, &s, &b);

        assert_eq!(result, Some((2, 14)));
    }

    #[test]
    fn test_find_coverage_for_row() {
        let Problem { reports } = TEST_INPUT.parse().unwrap();

        let coverage = find_coverage_for_row(9, &reports);
        assert_eq!(coverage, 25);
        let coverage = find_coverage_for_row(10, &reports);
        assert_eq!(coverage, 26);
        let coverage = find_coverage_for_row(11, &reports);
        assert_eq!(coverage, 28); // Cell with Sensor counts as covered
    }

    #[test]
    fn test_find_distress_beacons_signal() {
        let Problem { reports } = TEST_INPUT.parse().unwrap();
        let signal = find_distress_beacons_signal(0, 20, &reports);

        assert_eq!(signal, Some(56000011));
    }
}
