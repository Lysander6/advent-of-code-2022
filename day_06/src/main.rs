use anyhow::{anyhow, Context};
use common::{get_arg, read_file_to_string};

fn find_start_of_packet(packet: &str) -> Option<usize> {
    let idx = packet
        .chars()
        .collect::<Vec<_>>()
        .windows(4)
        .position(|window| {
            let [a, b, c, d]: [char; 4] = window.try_into().unwrap();

            a != b && a != c && a != d && b != c && b != d && c != d
        });

    idx.map(|i| i + 4)
}

fn main() -> Result<(), anyhow::Error> {
    let input_file_path = get_arg(1).context("pass path to input file as first argument")?;
    let input_string = read_file_to_string(&input_file_path)?;

    println!(
        "Part 1 solution: {}",
        find_start_of_packet(&input_string)
            .ok_or_else(|| anyhow!("couldn't find start of packet"))?
    );
    println!("Part 2 solution: {}", 0);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_start_of_packet() {
        assert_eq!(
            find_start_of_packet("mjqjpqmgbljsphdztnvjfqwrcgsmlb"),
            Some(7)
        );
        assert_eq!(
            find_start_of_packet("bvwbjplbgvbhsrlpgdmjqwftvncz"),
            Some(5)
        );
        assert_eq!(
            find_start_of_packet("nppdvjthqldpwncqszvftbrmjlhg"),
            Some(6)
        );
        assert_eq!(
            find_start_of_packet("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"),
            Some(10)
        );
        assert_eq!(
            find_start_of_packet("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"),
            Some(11)
        );
    }
}
