use std::iter;

use atoi::FromRadix10;
use bstr::ByteSlice;

pub fn part1(input: &str) -> u32 {
    let (numbers, boards) = parse_file(input);

    let mut curr_winner_board = &boards[0];
    let mut curr_winner_marked = Default::default();
    let mut curr_turns_to_win = numbers.len();

    for board in &boards {
        let (turns, marked) = turns_to_win(&numbers, board);
        if turns < curr_turns_to_win {
            curr_turns_to_win = turns;
            curr_winner_board = board;
            curr_winner_marked = marked;
        }
    }

    let sum_unmarked = sum_unmarked(curr_winner_board, &curr_winner_marked);

    sum_unmarked * u32::from(numbers[curr_turns_to_win])
}

pub fn part2(input: &str) -> u32 {
    let (numbers, boards) = parse_file(input);

    let mut curr_loser_board = &boards[0];
    let mut curr_loser_marked = Default::default();
    let mut curr_turns_to_lose = 0;

    for board in &boards {
        let (turns, marked) = turns_to_win(&numbers, board);
        if turns > curr_turns_to_lose {
            curr_turns_to_lose = turns;
            curr_loser_board = board;
            curr_loser_marked = marked;
        }
    }

    let sum_unmarked = sum_unmarked(curr_loser_board, &curr_loser_marked);

    sum_unmarked * u32::from(numbers[curr_turns_to_lose])
}

fn sum_unmarked(winner: &Board, marked: &[[bool; 5]; 5]) -> u32 {
    iter::zip(winner, marked)
        .map(|(row_w, row_m)| -> u32 {
            iter::zip(row_w, row_m)
                .filter(|(_, &m)| !m)
                .map(|(marked_cell, _)| u32::from(*marked_cell))
                .sum()
        })
        .sum()
}

fn turns_to_win(numbers: &[u8], board: &Board) -> (usize, [[bool; 5]; 5]) {
    let mut marked: [[bool; 5]; 5] = Default::default();
    for (turn, num) in numbers.iter().enumerate() {
        for (row_idx, row) in board.iter().enumerate() {
            for (col_idx, cell) in row.iter().enumerate() {
                if num == cell {
                    marked[row_idx][col_idx] = true;
                }
                if has_won(&marked) {
                    return (turn, marked);
                }
            }
        }
    }
    (numbers.len(), marked)
}

fn has_won(board: &[[bool; 5]; 5]) -> bool {
    for row in board {
        if *row == [true; 5] {
            return true;
        }
    }
    for col in 0..5 {
        if board.iter().all(|r| r[col]) {
            return true;
        }
    }
    false
}

type Board = [[u8; 5]; 5];

fn parse_file(input: &str) -> (Vec<u8>, Vec<Board>) {
    let (numbers_str, boards_str) = input.as_bytes().split_once_str("\n\n").unwrap();

    let numbers = numbers_str
        .split_str(",")
        .map(|num| u8::from_radix_10(num).0)
        .collect();

    let boards = boards_str
        .trim_end()
        .split_str("\n\n")
        .map(parse_board)
        .collect();

    (numbers, boards)
}

fn parse_board(input: &[u8]) -> Board {
    let mut output = Board::default();
    for (idx, num) in input.chunks(3).enumerate() {
        let (row, col) = (idx / 5, idx % 5);
        output[row][col] = u8::from_radix_10(num.trim_start()).0;
    }
    output
}
