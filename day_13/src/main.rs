use std::{cmp::Ordering, str::FromStr};

use anyhow::{anyhow, Context};
use common::{get_arg, read_file_to_string};

#[derive(Clone, Debug, PartialEq, Eq, Ord)]
enum Packet<T>
where
    T: PartialEq + Eq + PartialOrd + Ord,
    Packet<T>: PartialOrd,
{
    Val(T),
    Nested(Vec<Packet<T>>),
}

impl FromStr for Packet<u8> {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim(); // Trim any whitespace at the both ends

        // If `s` starts with `[` then we are dealing with nested packet,
        // otherwise it's literal value
        if s.starts_with('[') {
            // Exit early on empty nested packet
            if s == "[]" {
                return Ok(Self::Nested(vec![]));
            }

            // Trim outermost braces
            let s = &s[1..(s.len() - 1)];

            // Collect indices by which we will split incoming nested packet
            let mut nesting_level = 0;
            let non_nested_commas =
                s.char_indices()
                    .filter_map(|(i, c)| match (nesting_level, c) {
                        // Only collect indices of commas between packets only
                        // at the top-most level - not any nested ones (which
                        // will be processed by recursive calls to `parse`)
                        (0, ',') => {
                            Some(i + 1) // index right after comma
                        }
                        (_, '[') => {
                            nesting_level += 1;
                            None
                        }
                        (_, ']') => {
                            nesting_level -= 1;
                            None
                        }
                        _ => None,
                    });

            // Pad comma indices with 0 and `s.len() + 1` so we can iterate over
            // whole array in consistent manner
            let mut split_at_indices = vec![0];
            split_at_indices.extend(non_nested_commas);
            split_at_indices.push(s.len() + 1);

            let packets = split_at_indices
                .windows(2)
                .map(|idxs| {
                    let range = idxs[0]..(idxs[1] - 1);
                    s[range].parse()
                })
                .collect::<Result<_, _>>()?;

            Ok(Self::Nested(packets))
        } else {
            // Literal value
            Ok(Self::Val(
                s.parse::<u8>()
                    .with_context(|| format!("parsing u8 from '{}'", s))?,
            ))
        }
    }
}

impl PartialOrd for Packet<u8> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        use Packet::*;

        match (self, other) {
            (Val(_), Nested(v)) => {
                let left = vec![self.clone()];

                left.partial_cmp(v)
            }
            (Nested(v), Val(_)) => {
                let right = vec![other.clone()];

                v.partial_cmp(&right)
            }
            (Val(a), Val(b)) => a.partial_cmp(b),
            (Nested(a), Nested(b)) => a.partial_cmp(b),
        }
    }
}

#[derive(Debug)]
struct Problem {
    packet_pairs: Vec<(Packet<u8>, Packet<u8>)>,
}

impl FromStr for Problem {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let packet_pairs: Vec<(Packet<u8>, Packet<u8>)> = s
            .split("\n\n")
            .map(|pair| {
                let (left, right) = pair
                    .split_once('\n')
                    .ok_or_else(|| anyhow!("couldn't split at newline: {}", pair))?;

                Ok::<(Packet<u8>, Packet<u8>), Self::Err>((left.parse()?, right.parse()?))
            })
            .collect::<Result<_, _>>()?;

        Ok(Problem { packet_pairs })
    }
}

fn find_indices_of_packets_in_correct_order(packet_pairs: &[(Packet<u8>, Packet<u8>)]) -> Vec<u64> {
    (1..)
        .zip(packet_pairs)
        .filter_map(|(idx, (a, b))| if a <= b { Some(idx) } else { None })
        .collect()
}

fn find_decoder_key(packet_pairs: &Vec<(Packet<u8>, Packet<u8>)>) -> Result<u64, anyhow::Error> {
    let mut packets = packet_pairs
        .clone()
        .into_iter()
        .flat_map(|pair| [pair.0, pair.1])
        .collect::<Vec<_>>();

    let divider_packet_1: Packet<u8> = "[[2]]".parse()?;
    let divider_packet_2: Packet<u8> = "[[6]]".parse()?;

    packets.push(divider_packet_1.clone());
    packets.push(divider_packet_2.clone());

    packets.sort();

    let divider_packet_1_idx = packets
        .iter()
        .position(|p| *p == divider_packet_1)
        .ok_or_else(|| anyhow!("couldn't find first divider packet"))?;
    let divider_packet_2_idx = packets
        .iter()
        .position(|p| *p == divider_packet_2)
        .ok_or_else(|| anyhow!("couldn't find second divider packet"))?;

    Ok((divider_packet_1_idx as u64 + 1) * (divider_packet_2_idx as u64 + 1))
}

fn main() -> Result<(), anyhow::Error> {
    let input_file_path = get_arg(1).context("pass path to input file as first argument")?;
    let input_string = read_file_to_string(&input_file_path)?;

    let Problem { packet_pairs } = input_string.parse()?;

    let indices_in_correct_order = find_indices_of_packets_in_correct_order(&packet_pairs);

    println!(
        "Part 1 solution: {}",
        indices_in_correct_order.into_iter().sum::<u64>()
    );

    println!("Part 2 solution: {}", find_decoder_key(&packet_pairs)?);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use Packet::*;

    const TEST_INPUT: &str = "\
[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]";

    #[test]
    fn test_ordering_of_vecs() {
        let a = vec![1, 1, 3, 1, 1];
        let b = vec![1, 1, 5, 1, 1];

        assert!(a < b);

        let a = vec![2, 3, 4];
        let b = vec![4];

        assert!(a < b);

        let a = vec![9];
        let b = vec![8, 7, 6];

        assert!(a > b);

        let a = vec![4, 4];
        let b = vec![4, 4];

        assert!(a <= b);

        let a = vec![7, 7, 7, 7];
        let b = vec![7, 7, 7];

        assert!(a > b);

        let a = vec![];
        let b = vec![3];

        assert!(a < b);
    }

    #[test]
    fn test_packet_parsing_1() {
        let s = "[1,1,3,1,1]";
        let result = s.parse::<Packet<u8>>().unwrap();

        assert_eq!(result, Nested(vec![Val(1), Val(1), Val(3), Val(1), Val(1)]));
    }

    #[test]
    fn test_packet_parsing_2() {
        let s = "[[4,4],4,4]";
        let result = s.parse::<Packet<u8>>().unwrap();

        assert_eq!(
            result,
            Nested(vec![Nested(vec![Val(4), Val(4)]), Val(4), Val(4)])
        );
    }

    #[test]
    fn test_packet_parsing_3() {
        let s = "[[10],[7]]";
        let result = s.parse::<Packet<u8>>().unwrap();

        assert_eq!(
            result,
            Nested(vec![Nested(vec![Val(10)]), Nested(vec![Val(7)])])
        );
    }

    #[test]
    fn test_packet_parsing_4() {
        let s = "[[7,1,[8,9,[6,8],7,8],[3,2],2],[[5,[4,6,10,3,7],[5,6,10,7],3,[7,5,7,10,2]],[[4,5,10,6,10],[],7],[7,[9],[10,9,9]],4],[0,[5,3,[9,8]],[4,5,6,0,0],7],[2,1,[[2],[],[6,10],[8],[6,6,10,4,7]]]]";
        let result = s.parse::<Packet<u8>>();

        assert!(result.is_ok());
    }

    #[test]
    fn test_find_indices_of_packets_in_correct_order() {
        let Problem { packet_pairs } = TEST_INPUT.parse().unwrap();
        let result = find_indices_of_packets_in_correct_order(&packet_pairs);

        assert_eq!(result.into_iter().sum::<u64>(), 13);
    }

    #[test]
    fn test_find_decoder_key() {
        let Problem { packet_pairs } = TEST_INPUT.parse().unwrap();
        let result = find_decoder_key(&packet_pairs).unwrap();

        assert_eq!(result, 140);
    }
}
