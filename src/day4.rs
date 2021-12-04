use std::collections::HashSet;

fn get_score(numbers: &[u32], board: &[[u32; 5]; 5], turns: usize) -> u32 {
    let called_numbers: HashSet<_> = numbers.into_iter().take(turns).collect();
    let numbers_in_board: HashSet<_> = board.into_iter().flatten().collect();

    let sum_of_unmarked: u32 = numbers_in_board.difference(&called_numbers).copied().sum();

    sum_of_unmarked * numbers[turns - 1]
}

fn get_position_in_board(board: &[[u32; 5]; 5], number: u32) -> Option<(usize, usize)> {
    board
        .into_iter()
        .enumerate()
        .find_map(|(x, row)| row.into_iter().position(|&n| n == number).map(|y| (x, y)))
}

fn get_board_turns(board: &[[u32; 5]; 5], numbers: &[u32]) -> usize {
    let winner_subsets = {
        let mut temp = Vec::with_capacity(10);
        for i in 0..5 {
            let row: HashSet<_> = (0..5).map(|col| (i, col)).collect();
            let col: HashSet<_> = (0..5).map(|row| (row, i)).collect();
            temp.extend([row, col]);
        }
        temp
    };
    let mut marked = HashSet::new();
    for (turn, number) in numbers.into_iter().enumerate() {
        let turn = turn + 1;
        if let Some(position) = get_position_in_board(board, *number) {
            marked.insert(position);
            for winner_subset in &winner_subsets {
                if winner_subset.is_subset(&marked) {
                    return turn;
                }
            }
        }
    }
    unreachable!()
}

fn find_winner_board_and_turns(
    numbers: &[u32],
    boards: &[[[u32; 5]; 5]],
) -> ([[u32; 5]; 5], usize) {
    let mut current_winner = 0;
    let mut winner_turns = 25;
    for (idx, board) in boards.into_iter().enumerate() {
        let turns = get_board_turns(board, numbers);
        if turns < winner_turns {
            current_winner = idx;
            winner_turns = turns;
        }
    }
    (boards[current_winner], winner_turns)
}

fn find_loser_board_and_turns(numbers: &[u32], boards: &[[[u32; 5]; 5]]) -> ([[u32; 5]; 5], usize) {
    let mut current_loser = 0;
    let mut loser_turns = 0;
    for (idx, board) in boards.into_iter().enumerate() {
        let turns = get_board_turns(board, numbers);
        if turns > loser_turns {
            current_loser = idx;
            loser_turns = turns;
        }
    }
    (boards[current_loser], loser_turns)
}

fn parse_file(file: &str) -> (Vec<u32>, Vec<[[u32; 5]; 5]>) {
    let mut sections = file.split("\n\n");
    let numbers: Vec<u32> = sections
        .nth(0)
        .unwrap()
        .split(',')
        .map(|n| n.parse().unwrap())
        .collect();
    let boards: Vec<_> = sections
        .map(|board| {
            let board = board;
            let mut arr_board: [[u32; 5]; 5] = [[0; 5]; 5];

            for i in (0..5).map(|n| n * 15) {
                for j in (0..5).map(|n| n * 3) {
                    arr_board[i / 15][j / 3] =
                        board[i..i + 14][j..j + 2].trim_start().parse().unwrap();
                }
            }

            arr_board
        })
        .collect();
    (numbers, boards)
}

#[inline]
pub fn a(file: &str) -> String {
    let (numbers, boards) = parse_file(file);
    let (board, turns) = find_winner_board_and_turns(&numbers, &boards);
    let final_score = get_score(&numbers, &board, turns);
    final_score.to_string()
}

#[inline]
pub fn b(file: &str) -> String {
    let (numbers, boards) = parse_file(file);
    let (board, turns) = find_loser_board_and_turns(&numbers, &boards);
    let final_score = get_score(&numbers, &board, turns);
    final_score.to_string()
}

#[test]
fn day4_is_correct() {
    use super::FILES;

    assert_eq!(a(FILES[3]), "45031");
    assert_eq!(b(FILES[3]), "2568");
}
