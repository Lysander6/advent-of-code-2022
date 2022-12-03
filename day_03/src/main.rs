use anyhow::{bail, Context};
use common::{get_arg, read_file_to_string};

fn char_to_priority(c: char) -> Result<usize, anyhow::Error> {
    match c {
        'a'..='z' => Ok((c as usize) - ('a' as usize) + 1),
        'A'..='Z' => Ok((c as usize) - ('A' as usize) + 27),
        _ => bail!("can't convert {} to priority", c),
    }
}

fn split_in_half(s: &str) -> (&str, &str) {
    // assumes `s` of even length
    let half = s.len() / 2;

    (&s[0..half], &s[half..])
}

fn find_common_item_type(rucksack_items: &str) -> Result<usize, anyhow::Error> {
    let mut seen_items = [false; 53];

    let (left, right) = split_in_half(rucksack_items);

    for item_priority in left.chars().map(char_to_priority) {
        seen_items[item_priority?] = true;
    }

    for item_priority in right.chars().map(char_to_priority) {
        let item_priority = item_priority?;
        if seen_items[item_priority] {
            return Ok(item_priority);
        }
    }

    unreachable!("assuming every rucksack has common item type between compartments");
}

fn sum_common_item_types(inventories_raw: &str) -> Result<usize, anyhow::Error> {
    let inventories = inventories_raw.lines();
    inventories.map(find_common_item_type).sum()
}

fn find_common_item_type_between_three_rucksacks(
    rucksack_a: &str,
    rucksack_b: &str,
    rucksack_c: &str,
) -> Result<usize, anyhow::Error> {
    let mut seen_in_a = [false; 53];
    let mut seen_in_a_and_b = [false; 53];

    for item_priority in rucksack_a.chars().map(char_to_priority) {
        seen_in_a[item_priority?] = true;
    }

    for item_priority in rucksack_b.chars().map(char_to_priority) {
        let item_priority = item_priority?;
        if seen_in_a[item_priority] {
            seen_in_a_and_b[item_priority] = true;
        }
    }

    for item_priority in rucksack_c.chars().map(char_to_priority) {
        let item_priority = item_priority?;
        if seen_in_a_and_b[item_priority] {
            return Ok(item_priority);
        }
    }

    unreachable!(
        "assuming every three consecutive rucksacks have common item type shared between them"
    );
}

fn sum_group_badges(inventories_raw: &str) -> Result<usize, anyhow::Error> {
    inventories_raw
        .lines()
        .collect::<Vec<_>>()
        .chunks_exact(3)
        .map(|chunk| match chunk {
            &[a, b, c] => find_common_item_type_between_three_rucksacks(a, b, c),
            _ => unreachable!("`chunks_exact` gives exactly 3-item long slices"),
        })
        .sum()
}

fn main() -> Result<(), anyhow::Error> {
    let input_file_path = get_arg(1).context("pass path to input file as first argument")?;
    let input_string = read_file_to_string(&input_file_path)?;

    let part_1_solution = sum_common_item_types(&input_string)?;
    let part_2_solution = sum_group_badges(&input_string)?;

    println!("Part 1 solution: {}", part_1_solution);
    println!("Part 2 solution: {}", part_2_solution);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_char_to_priority() {
        assert_eq!(char_to_priority('a').unwrap(), 1);
        assert_eq!(char_to_priority('z').unwrap(), 26);
        assert_eq!(char_to_priority('A').unwrap(), 27);
        assert_eq!(char_to_priority('Z').unwrap(), 52);
        assert_eq!(char_to_priority('p').unwrap(), 16);
        assert_eq!(char_to_priority('L').unwrap(), 38);
    }

    #[test]
    fn test_split_in_half() {
        assert_eq!(
            split_in_half("vJrwpWtwJgWrhcsFMMfFFhFp"),
            ("vJrwpWtwJgWr", "hcsFMMfFFhFp")
        );
        assert_eq!(
            split_in_half("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL"),
            ("jqHRNqRjqzjGDLGL", "rsFMfFZSrLrFZsSL")
        );
    }

    #[test]
    fn test_find_common_item() {
        assert_eq!(
            find_common_item_type("vJrwpWtwJgWrhcsFMMfFFhFp").unwrap(),
            16
        );
        assert_eq!(
            find_common_item_type("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL").unwrap(),
            38
        );
        assert_eq!(find_common_item_type("PmmdzqPrVvPwwTWBwg").unwrap(), 42);
        assert_eq!(
            find_common_item_type("wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn").unwrap(),
            22
        );
        assert_eq!(find_common_item_type("ttgJtRGJQctTZtZT").unwrap(), 20);
        assert_eq!(
            find_common_item_type("CrZsJsPPZsGzwwsLwLmpwMDw").unwrap(),
            19
        );
    }

    const TEST_INPUT: &str = "\
vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn test_sum_common_item_types() {
        assert_eq!(sum_common_item_types(&TEST_INPUT).unwrap(), 157);
    }

    #[test]
    fn test_find_common_item_type_between_three_rucksacks() {
        assert_eq!(
            find_common_item_type_between_three_rucksacks(
                "vJrwpWtwJgWrhcsFMMfFFhFp",
                "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL",
                "PmmdzqPrVvPwwTWBwg"
            )
            .unwrap(),
            18
        );
        assert_eq!(
            find_common_item_type_between_three_rucksacks(
                "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn",
                "ttgJtRGJQctTZtZT",
                "CrZsJsPPZsGzwwsLwLmpwMDw"
            )
            .unwrap(),
            52
        );
    }

    #[test]
    fn test_sum_group_badges() {
        assert_eq!(sum_group_badges(&TEST_INPUT).unwrap(), 70);
    }
}
