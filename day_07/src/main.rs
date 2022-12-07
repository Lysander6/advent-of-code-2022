use std::collections::HashMap;

use anyhow::Context;
use common::{get_arg, read_file_to_string};

fn dir_walk(commands: &[&str]) -> HashMap<String, u64> {
    let mut cwd: Vec<String> = vec![];
    let mut dirs: HashMap<String, u64> = HashMap::new();

    for &command in commands {
        if command.starts_with("$ cd") {
            match &command[5..] {
                "/" => {
                    cwd = vec!["".to_string()];
                }
                ".." => {
                    cwd.pop();
                }
                other => {
                    cwd.push(other.to_string());
                }
            }
        } else if command.starts_with("$ ls") || command.starts_with("dir") {
            continue;
        } else {
            let (size, _file_name) = command.split_once(' ').unwrap();
            let size = size.parse::<u64>().unwrap();

            // update totals up the tree
            for i in 0..cwd.len() {
                let key = cwd[0..=i].join("/").to_string();

                dirs.entry(key).and_modify(|a| *a += size).or_insert(size);
            }
        }
    }

    dirs
}

fn sum_sizes_of_small_directories(dirs: &HashMap<String, u64>) -> u64 {
    dirs.iter()
        .filter_map(|(_, &size)| if size <= 100000 { Some(size) } else { None })
        .sum()
}

fn find_smallest_directory_that_frees_up_enough_space(dirs: &HashMap<String, u64>) -> u64 {
    let total_disk_space = 70000000;
    let current_free_space = total_disk_space - dirs[""];
    let space_needed_to_be_freed = 30000000 - current_free_space;

    let mut sizes = dirs
        .iter()
        .filter_map(|(_, &size)| {
            if size >= space_needed_to_be_freed {
                Some(size)
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    sizes.sort();

    sizes[0]
}

fn main() -> Result<(), anyhow::Error> {
    let input_file_path = get_arg(1).context("pass path to input file as first argument")?;
    let input_string = read_file_to_string(&input_file_path)?;

    let dirs = dir_walk(&input_string.lines().collect::<Vec<_>>());

    println!("Part 1 solution: {}", sum_sizes_of_small_directories(&dirs));
    println!(
        "Part 2 solution: {}",
        find_smallest_directory_that_frees_up_enough_space(&dirs)
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "\
$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";

    #[test]
    fn test_dir_walk() {
        let dirs = dir_walk(&TEST_INPUT.lines().collect::<Vec<_>>());

        assert_eq!(
            dirs,
            HashMap::from([
                ("".to_string(), 48381165),
                ("/a".to_string(), 94853),
                ("/a/e".to_string(), 584),
                ("/d".to_string(), 24933642),
            ])
        );
    }

    #[test]
    fn test_sum_sizes_of_small_directories() {
        let dirs = dir_walk(&TEST_INPUT.lines().collect::<Vec<_>>());
        let sum = sum_sizes_of_small_directories(&dirs);

        assert_eq!(sum, 95437);
    }

    #[test]
    fn test_find_smallest_directory_that_frees_up_enough_space() {
        let dirs = dir_walk(&TEST_INPUT.lines().collect::<Vec<_>>());
        let size = find_smallest_directory_that_frees_up_enough_space(&dirs);

        assert_eq!(size, 24933642);
    }
}
