#![allow(
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    clippy::cast_sign_loss
)]

use std::ops::Index;

pub fn part1(input: &str) -> u16 {
    let grid = Grid::from_str(input).unwrap();
    let (start_row, start_col) = grid.find(b'S').unwrap();
    let (mut last_row, mut last_col) = grid.get_s_neighbors((start_row, start_col)).next().unwrap();
    let (mut curr_row, mut curr_col) = (start_row, start_col);

    let mut lenght = 0;
    while !(curr_row == start_row && curr_col == start_col) || lenght == 0 {
        let (next_row, next_col) = grid.get_neighbor((curr_row, curr_col), (last_row, last_col));
        (last_row, last_col) = (curr_row, curr_col);
        (curr_row, curr_col) = (next_row, next_col);
        lenght += 1;
    }
    lenght / 2
}

pub fn part2(input: &str) -> i16 {
    let grid = Grid::from_str(input).unwrap();
    let (start_row, start_col) = grid.find(b'S').unwrap();
    let (mut last_row, mut last_col) = grid.get_s_neighbors((start_row, start_col)).next().unwrap();
    let (mut curr_row, mut curr_col) = (start_row, start_col);

    let mut lenght = 0;
    let mut twice_area = 0;
    while !(curr_row == start_row && curr_col == start_col) || lenght == 0 {
        let (next_row, next_col) = grid.get_neighbor((curr_row, curr_col), (last_row, last_col));
        (last_row, last_col) = (curr_row, curr_col);
        (curr_row, curr_col) = (next_row, next_col);
        lenght += 1;
        twice_area += (last_col + curr_col) * (last_row - curr_row);
    }
    (twice_area.abs() - lenght) / 2 + 1
}

struct Grid<'a> {
    inner: &'a [u8],
    modulo: i16,
    col_len: i16,
}

impl<'a> Grid<'a> {
    fn from_str(input: &'a str) -> Option<Self> {
        Some(Self {
            inner: input.as_bytes(),
            modulo: input.find('\n')? as i16 + 1,
            col_len: input.lines().count() as i16,
        })
    }

    fn find(&self, haystack: u8) -> Option<(i16, i16)> {
        let idx = self.inner.iter().position(|&pipe| pipe == haystack)? as i16;
        let row = idx / self.modulo;
        let col = idx % self.modulo;
        Some((row, col))
    }

    fn get_s_neighbors(
        &self,
        (curr_row, curr_col): (i16, i16),
    ) -> impl Iterator<Item = (i16, i16)> + '_ {
        [(-1, 0), (0, -1), (1, 0), (0, 1)]
            .into_iter()
            .filter(move |&(d_row, d_col)| {
                curr_row + d_row >= 0
                    && curr_row + d_row < self.col_len
                    && curr_col + d_col >= 0
                    && curr_col + d_col < self.modulo - 1
            })
            .map(move |(d_row, d_col)| ((d_row, d_col), self[(curr_row + d_row, curr_col + d_col)]))
            .filter(|neighbor| {
                matches!(
                    neighbor,
                    ((-1, 0), b'|' | b'7' | b'F')
                        | ((0, -1), b'-' | b'L' | b'F')
                        | ((1, 0), b'|' | b'L' | b'J')
                        | ((0, 1), b'-' | b'J' | b'7')
                )
            })
            .map(move |((d_row, d_col), _)| (curr_row + d_row, curr_col + d_col))
    }

    fn get_neighbor(
        &self,
        (curr_row, curr_col): (i16, i16),
        (last_row, last_col): (i16, i16),
    ) -> (i16, i16) {
        let last_d_row = last_row - curr_row;
        let last_d_col = last_col - curr_col;
        match self[(curr_row, curr_col)] {
            b'|' | b'-' => (curr_row - last_d_row, curr_col - last_d_col),
            b'L' | b'7' => (curr_row - last_d_col, curr_col - last_d_row),
            b'J' | b'F' => (curr_row + last_d_col, curr_col + last_d_row),
            // (curr_row, curr_col) -> S
            _ => self
                .get_s_neighbors((curr_row, curr_col))
                .find(|&pipe| pipe != (last_row, last_col))
                .unwrap(),
        }
    }
}

impl Index<(i16, i16)> for Grid<'_> {
    type Output = u8;

    fn index(&self, (row, col): (i16, i16)) -> &Self::Output {
        &self.inner[(row * self.modulo + col) as usize]
    }
}
