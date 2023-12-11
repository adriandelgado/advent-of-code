use std::ops::Index;

use bstr::ByteSlice;
use itertools::Itertools;

fn solve<const EXTRA: usize>(input: &str) -> usize {
    let grid = Grid::from_str(input).unwrap();
    let empty_rows = grid.empty_rows();
    let empty_cols = grid.empty_cols();
    let galaxies = grid.galaxies();

    galaxies
        .into_iter()
        .tuple_combinations()
        .map(|((row_0, col_0), (row_1, col_1))| {
            let (row_0, row_1) = sort_pair((row_0, row_1));
            let (col_0, col_1) = sort_pair((col_0, col_1));
            let extra_rows = empty_rows
                .iter()
                .filter(|&row_idx| (row_0..row_1).contains(row_idx))
                .count()
                * EXTRA;
            let extra_cols = empty_cols
                .iter()
                .filter(|&col_idx| (col_0..col_1).contains(col_idx))
                .count()
                * EXTRA;

            (row_1 - row_0) + extra_rows + (col_1 - col_0) + extra_cols
        })
        .sum()
}

pub fn part1(input: &str) -> usize {
    solve::<1>(input)
}

pub fn part2(input: &str) -> usize {
    solve::<999_999>(input)
}

fn sort_pair((x, y): (usize, usize)) -> (usize, usize) {
    if x <= y {
        (x, y)
    } else {
        (y, x)
    }
}

struct Grid<'a> {
    inner: &'a [u8],
    row_len: usize,
    col_len: usize,
}

impl<'a> Grid<'a> {
    fn from_str(input: &'a str) -> Option<Self> {
        Some(Self {
            inner: input.as_bytes(),
            row_len: input.find('\n')?,
            col_len: input.lines().count(),
        })
    }

    fn empty_rows(&self) -> Vec<usize> {
        self.inner
            .chunks_exact(self.row_len + 1)
            .positions(|row| row[..self.row_len].iter().all(|&ch| ch == b'.'))
            .collect()
    }

    fn empty_cols(&self) -> Vec<usize> {
        (0..self.col_len)
            .filter(|&col| (0..self.row_len).all(|row| self[(row, col)] == b'.'))
            .collect()
    }

    fn galaxies(&self) -> Vec<(usize, usize)> {
        self.inner
            .find_iter("#")
            .map(|idx| self.idx_to_coords(idx))
            .collect()
    }

    fn idx_to_coords(&self, idx: usize) -> (usize, usize) {
        let row = idx / (self.row_len + 1);
        let col = idx % (self.row_len + 1);
        (row, col)
    }
}

impl Index<(usize, usize)> for Grid<'_> {
    type Output = u8;

    fn index(&self, (row, col): (usize, usize)) -> &Self::Output {
        &self.inner[row * (self.row_len + 1) + col]
    }
}
