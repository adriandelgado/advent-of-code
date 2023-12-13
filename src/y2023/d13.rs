use std::ops::Range;

pub fn part1(input: &str) -> usize {
    input.split_inclusive("\n\n").map(pattern_notes::<0>).sum()
}

pub fn part2(input: &str) -> usize {
    input.split_inclusive("\n\n").map(pattern_notes::<1>).sum()
}

fn pattern_notes<const DISTANCE: usize>(pattern: &str) -> usize {
    let pattern = Grid::from_str(pattern).unwrap();
    let mut rows_above = 0;
    let mut cols_before = 0;

    for row_idx in 1..pattern.col_len {
        let end = (2 * row_idx).min(pattern.col_len);
        let start = 2 * row_idx - end;

        if std::iter::zip(
            pattern.rows_slice(start..row_idx).rev().flatten(),
            pattern.rows_slice(row_idx..end).flatten(),
        )
        .filter(|(l, r)| l != r)
        .count()
            == DISTANCE
        {
            rows_above = row_idx;
            break;
        }
    }

    for col_idx in 1..pattern.row_len {
        let end = (2 * col_idx).min(pattern.row_len);
        let start = 2 * col_idx - end;

        if std::iter::zip(
            pattern
                .all_rows()
                .flat_map(|row| row[start..col_idx].iter().rev()),
            pattern.all_rows().flat_map(|row| &row[col_idx..end]),
        )
        .filter(|(l, r)| l != r)
        .count()
            == DISTANCE
        {
            cols_before = col_idx;
            break;
        }
    }

    rows_above * 100 + cols_before
}

struct Grid<'a> {
    inner: &'a [u8],
    row_len: usize,
    col_len: usize,
}

impl<'a> Grid<'a> {
    fn from_str(input: &'a str) -> Option<Self> {
        let row_len = input.find('\n')?;
        Some(Self {
            inner: input.as_bytes(),
            row_len,
            col_len: input.len() / (row_len + 1),
        })
    }

    fn rows_slice(&self, range: Range<usize>) -> impl DoubleEndedIterator<Item = &[u8]> + '_ {
        self.inner
            .chunks_exact(self.row_len + 1)
            .map(|row_lf| &row_lf[..self.row_len])
            .skip(range.start)
            .take(range.end - range.start)
    }

    fn all_rows(&self) -> impl Iterator<Item = &[u8]> + '_ {
        self.inner
            .chunks_exact(self.row_len + 1)
            .map(|row_lf| &row_lf[..self.row_len])
    }
}
