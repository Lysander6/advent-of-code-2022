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

    for (shape_height, rock_parts) in get_shapes().iter().cycle().take(rocks_to_drop) {
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
                for (col, row) in rock_parts {
                    columns[col][row] = true;

                    // Note height of topmost block
                    if row + 1 > top_of_highest_block {
                        top_of_highest_block = row + 1;
                    }
                }

                break 'dropping;
            }
        }
    }

    (top_of_highest_block, columns)
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

    let (height, columns) = simulate_tetris(&instructions, 20022);
    let cycle = detect_cycle(&columns);
    eprintln!("cycle: {:?}", cycle);
    println!("Part 2 solution: {}", 0);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";

    #[test]
    fn test_simulate_tetris_1() {
        let (height, columns) = simulate_tetris(TEST_INPUT, 2022);

        // print_board(&columns);

        assert_eq!(height, 3068);
    }

    #[test]
    fn test_simulate_tetris_2() {
        let (height, columns) = simulate_tetris(TEST_INPUT, 2022);

        // print_board(&columns);
        let cycle = detect_cycle(&columns);
        eprintln!("cycle: {:?}", cycle);

        eprintln!(
            "eq?: {}",
            (0..7).all(|n| columns[n][25] == columns[n][25 + 53])
        );

        assert_eq!(height, 1514285714288);
    }
}
