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

fn find_common_item(runsack_items: &str) -> Result<usize, anyhow::Error> {
    let mut seen_items = [false; 53];

    let (left, right) = split_in_half(runsack_items);

    for item_priority in left.chars().map(char_to_priority) {
        seen_items[item_priority?] = true;
    }

    for item_priority in right.chars().map(char_to_priority) {
        let item_priority = item_priority?;
        if seen_items[item_priority] {
            return Ok(item_priority);
        }
    }

    unreachable!("assuming every rucksack has common item between compartments");
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
        assert_eq!(find_common_item("vJrwpWtwJgWrhcsFMMfFFhFp").unwrap(), 16);
        assert_eq!(
            find_common_item("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL").unwrap(),
            38
        );
        assert_eq!(find_common_item("PmmdzqPrVvPwwTWBwg").unwrap(), 42);
        assert_eq!(
            find_common_item("wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn").unwrap(),
            22
        );
    }
}
