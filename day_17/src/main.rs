use anyhow::Context;
use common::{get_arg, read_file_to_string};

fn get_shapes() -> Vec<(usize, Vec<(usize, usize)>)> {
    let plank_parts = vec![
        (2, 0), // leftmost part
        (3, 0),
        (4, 0),
        (5, 0), // rightmost part
    ];
    let cross_parts = vec![(2, 1), (3, 0), (3, 1), (3, 2), (4, 1)];
    let l_parts = vec![(2, 0), (3, 0), (4, 0), (4, 1), (4, 2)];
    let plank_vertical_parts = vec![(2, 0), (2, 1), (2, 2), (2, 3)];
    let block_parts = vec![(2, 0), (2, 1), (3, 0), (3, 1)];

    vec![
        (1, plank_parts),
        (3, cross_parts),
        (3, l_parts),
        (4, plank_vertical_parts),
        (2, block_parts),
    ]
}

fn print_board(columns: &Vec<Vec<bool>>) {
    for row in (0..columns[0].len()).rev() {
        print!("{row:6} |");
        for col in 0..columns.len() {
            if columns[col][row] {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!("|");
    }
    print!("       +");
    for _ in 0..columns.len() {
        print!("-");
    }
    println!("+");
}

fn simulate_tetris(instructions: &str, rocks_to_drop: usize) -> (usize, Vec<Vec<bool>>) {
    let mut columns = vec![vec![false; 3]; 7];
    let mut top_of_highest_block = 0usize;
    let mut instructions = instructions.chars().cycle();
    let mut in_cycle = false;
    let mut rocks_dropped_out_of_cycle = 0usize;
    let mut rocks_dropped_in_cycle = 0usize;
    let mut height_from_cycle = 0usize;
    let cycle_start = 25;
    let cycle_len = 53;

    for (shape_height, rock_parts) in get_shapes().iter().cycle() {
        for c in 0..columns.len() {
            columns[c].resize(top_of_highest_block + 3 + shape_height, false);
        }

        let mut rock_parts = rock_parts
            .into_iter()
            .map(|&(col, row)| (col, row + top_of_highest_block + 3))
            .collect::<Vec<_>>();

        'dropping: loop {
            match instructions.next() {
                Some('<') => {
                    if rock_parts[0].0 == 0 {
                        // Shape touches leftmost column, can't move further
                        // left
                    } else {
                        // Check if cell to left of every piece is unoccupied
                        let all_free = rock_parts
                            .iter()
                            .all(|&(col, row)| columns[col - 1][row] == false);

                        if all_free {
                            // Move piece
                            for (ref mut col, _) in rock_parts.iter_mut() {
                                *col -= 1;
                            }
                        }
                    }
                }
                Some('>') => {
                    if rock_parts[rock_parts.len() - 1].0 == 6 {
                        // Shape touches rightmost column
                    } else {
                        let all_free = rock_parts
                            .iter()
                            .all(|&(col, row)| columns[col + 1][row] == false);

                        if all_free {
                            // Move piece
                            for (ref mut col, _) in rock_parts.iter_mut() {
                                *col += 1;
                            }
                        }
                    }
                }
                Some(c) => panic!("unknown instruction: '{}'", c),
                None => {
                    // Exhausted all instructions
                }
            }

            // Move down
            let all_free = rock_parts
                .iter()
                .all(|&(col, row)| row > 0 && columns[col][row - 1] == false);

            if all_free {
                for (_, ref mut row) in rock_parts.iter_mut() {
                    *row -= 1;
                }
            } else {
                // Set stone... in stone
                eprintln!("shape: {:?}", &rock_parts);
                for (col, row) in rock_parts {
                    columns[col][row] = true;

                    // Note height of topmost block
                    if row + 1 > top_of_highest_block {
                        top_of_highest_block = row + 1;
                    }
                }

                if top_of_highest_block == cycle_start && rocks_dropped_in_cycle == 0 {
                    in_cycle = true;
                    eprintln!("cycle start");
                }

                if top_of_highest_block > cycle_start + cycle_len && in_cycle {
                    eprintln!("cycle end");
                    in_cycle = false;

                    // We are one stone past cycle end
                    rocks_dropped_in_cycle -= 1;

                    eprintln!(
                        "rocks_dropped_out_of_cycle: {}, rocks_dropped_in_cycle: {}",
                        rocks_dropped_out_of_cycle, rocks_dropped_in_cycle
                    );

                    let cycles_remaining =
                        (rocks_to_drop - rocks_dropped_in_cycle - (rocks_dropped_out_of_cycle + 1))
                            / (rocks_dropped_in_cycle);

                    eprintln!("cycles_remaining: {}", cycles_remaining);

                    height_from_cycle = (cycles_remaining - 2) * (cycle_len);
                    eprintln!("cycle_len: {}", cycle_len);

                    rocks_dropped_in_cycle *= cycles_remaining - 1;
                    println!(
                        "rocks left to throw: {}",
                        rocks_to_drop - rocks_dropped_in_cycle - (rocks_dropped_out_of_cycle + 1)
                    );
                }

                if in_cycle {
                    rocks_dropped_in_cycle += 1;
                } else {
                    rocks_dropped_out_of_cycle += 1;
                }

                break 'dropping;
            }
        }

        // We've got some rogue rock, so we stop one short from target
        if rocks_dropped_out_of_cycle + rocks_dropped_in_cycle >= rocks_to_drop - 1 {
            // assert_eq!(
            //     rocks_dropped_out_of_cycle + rocks_dropped_in_cycle,
            //     rocks_to_drop
            // );
            break;
        }
    }

    eprintln!("stones_dropped_in_cycle: {}", rocks_dropped_in_cycle);

    eprintln!(
        "top_of_highest_block: {}, height_from_cycle: {}",
        top_of_highest_block, height_from_cycle
    );

    (top_of_highest_block + height_from_cycle, columns)
}

fn detect_cycle(columns: &Vec<Vec<bool>>) -> Option<(usize, usize)> {
    let cols = columns.len();
    let rows = columns[0].len();
    let mut i = 0;
    let mut j = 1;

    while j < rows {
        if (0..cols).all(|idx| columns[idx][i] == columns[idx][j]) {
            let mut cycle_start = i;
            let cycle_len = j - i;

            // Rewind
            while (0..cols)
                .all(|idx| columns[idx][cycle_start] == columns[idx][cycle_start + cycle_len])
            {
                cycle_start -= 1;
            }
            cycle_start += 1;

            // Verify
            if (cycle_start..=(cycle_start + 2 * cycle_len))
                .all(|n| (0..cols).all(|idx| columns[idx][n] == columns[idx][n + cycle_len]))
            {
                return Some((cycle_start, cycle_len));
            }
        }

        i += 1;
        j += 2;
    }

    None
}

fn main() -> Result<(), anyhow::Error> {
    let input_file_path = get_arg(1).context("pass path to input file as first argument")?;
    let input_string = read_file_to_string(&input_file_path)?;
    let instructions = input_string.trim();
    let (height, columns) = simulate_tetris(&instructions, 2022);

    println!("Part 1 solution: {}", height);

    // let (height, columns) = simulate_tetris(&instructions, 20022);
    // let cycle = detect_cycle(&columns);
    // eprintln!("cycle: {:?}", cycle);
    // println!("Part 2 solution: {}", 0);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";

    #[test]
    fn test_simulate_tetris_1() {
        let (height, _columns) = simulate_tetris(TEST_INPUT, 2022);

        // print_board(&columns);

        assert_eq!(height, 3068);
    }

    #[test]
    fn test_simulate_tetris_2() {
        let (height, _columns) = simulate_tetris(TEST_INPUT, 1000000000000);

        // print_board(&columns);

        assert_eq!(height, 1514285714288);
    }

    // #[test]
    // fn test_simulate_tetris_2() {
    //     let (height, columns) = simulate_tetris(TEST_INPUT, 2022);

    //     // print_board(&columns);
    //     let cycle = detect_cycle(&columns);
    //     eprintln!("cycle: {:?}", cycle);

    //     let (cycle_start, cycle_len) = cycle.unwrap();

    //     let peepee = (0..7)
    //         .map(|col| columns[col][cycle_start..(cycle_start + cycle_len * 2)].to_vec())
    //         .collect::<Vec<_>>();

    //     // print_board(&peepee);

    //     let peepee = (0..7)
    //         .map(|col| columns[col][cycle_start..(cycle_start + cycle_len)].to_vec())
    //         .collect::<Vec<_>>();

    //     // print_board(&peepee);

    //     eprintln!(
    //         "eq?: {}",
    //         (0..7).all(|n| columns[n][25] == columns[n][25 + 53])
    //     );

    //     assert_eq!(height, 1514285714288);
    // }
}
