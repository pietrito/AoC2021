fn parse_input(input: &str) -> (Vec<u32>, Vec<[(u32, bool); 25]>) {
    let mut lines = input.lines();

    let draws = lines
        .next()
        .unwrap()
        .split(',')
        .map(|n| n.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();

    // Read empty line
    lines.next();

    // Vector of loaded boards
    let mut boards: Vec<[(u32, bool); 25]> = Vec::new();

    // Start of new board
    while let Some(mut line) = lines.next() {
        // Currently loaded board
        let mut board = [(0u32, false); 25];
        // Read next 5 lines as whole board
        for i in 0..5 {
            // Parse line's five values and put into board
            let five = line
                .split_whitespace()
                .map(|n| (n.parse::<u32>().unwrap(), false))
                .collect::<Vec<(u32, bool)>>();
            board[i * 5..(i * 5 + 5)].copy_from_slice(&five[0..5]);
            // Read next line, possibly empty line
            line = lines.next().unwrap_or("");
        }
        // Push read board
        boards.push(board);
    }

    (draws, boards)
}

/**
 * Returns `true` if the board has a winning row or column. Return `false` otherwise
 */
fn is_board_done(board: &[(u32, bool); 25]) -> bool {
    // Check rows
    if board.chunks(5).any(|row| row.iter().all(|v| v.1))
                // Check columns
                || (0..5).any(|col| board.iter().skip(col).step_by(5).all(|v| v.1))
    {
        return true;
    }

    false
}

/**
 * Calculates the sum of all unmarked cells of the board.
 */
fn sum_unmarked(board: &[(u32, bool); 25]) -> u32 {
    board.iter().filter(|v| !v.1).map(|v| v.0).sum::<u32>()
}

/// To guarantee victory against the giant squid, figure out which board will win first.
/// What will your final score be if you choose that board?
pub fn part1(input: &str) -> String {
    let (draws, mut boards) = parse_input(input);

    for draw in draws {
        // Do the move on each board
        boards.iter_mut().for_each(move |board| {
            board.iter_mut().for_each(move |b| {
                if b.0 == draw {
                    b.1 = true;
                }
            })
        });

        // Check if a board just won
        for board in &boards {
            if is_board_done(board) {
                // Sum of all unmarked multiplied by the last draw value
                return (sum_unmarked(board) * draw).to_string();
            }
        }
    }

    // None of the board won after all the draws
    panic!("Not a single board won.");
}

/// Figure out which board will win last. Once it wins, what would its final score be?
pub fn part2(input: &str) -> String {
    let (draws, mut boards) = parse_input(input);

    for draw in draws {
        // Do the move on each board
        boards.iter_mut().for_each(move |board| {
            board.iter_mut().for_each(move |b| {
                if b.0 == draw {
                    b.1 = true;
                }
            })
        });

        let winners = boards
            .iter()
            .enumerate()
            .filter_map(|(i, &b)| if is_board_done(&b) { Some(i) } else { None })
            // Important: Reverse the winners so that we can remove them without changing the
            // previous ones
            .rev()
            .collect::<Vec<_>>();

        for winner in winners {
            if boards.len() == 1 && is_board_done(&boards[0]) {
                return (sum_unmarked(&boards[0]) * draw).to_string();
            }
            // println!("Removing board {}/{}.", winner, boards.len());
            boards.remove(winner);
        }
    }

    // None of the board won after all the draws
    unreachable!("Failed finding last winning board.");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = r#"
7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7
            "#
        .trim();

        assert_eq!(part1(input), "4512".to_string());
    }

    #[test]
    fn test_part2() {
        let input = r#"
7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7
            "#
        .trim();

        assert_eq!(part2(input), "1924".to_string());
    }
}
