use bstr::ByteSlice;

pub fn part1(input: &str) -> usize {
    input
        .split("\n\n")
        .map(|part| pattern_notes_1(part.as_bytes()))
        .sum()
}

fn pattern_notes_1(part: &[u8]) -> usize {
    let grid: Vec<_> = part.lines().collect();

    let mut num_rows = 0;
    let mut num_cols = 0;

    for row_idx in 1..grid.len() {
        let start = (2 * row_idx).saturating_sub(grid.len());
        let end = (2 * row_idx).min(grid.len());

        if itertools::equal(&grid[start..row_idx], grid[row_idx..end].iter().rev()) {
            num_rows = row_idx;
        }
    }

    for col_idx in 1..grid[0].len() {
        let start = (2 * col_idx).saturating_sub(grid[0].len());
        let end = (2 * col_idx).min(grid[0].len());

        if itertools::equal(
            grid.iter().flat_map(|row| row[start..col_idx].iter()),
            grid.iter().flat_map(|row| row[col_idx..end].iter().rev()),
        ) {
            num_cols = col_idx;
        }
    }

    num_rows * 100 + num_cols
}

pub fn part2(input: &str) -> usize {
    input
        .split("\n\n")
        .map(|part| pattern_notes_2(part.as_bytes()))
        .sum()
}

fn pattern_notes_2(part: &[u8]) -> usize {
    let grid: Vec<_> = part.lines().collect();

    let mut num_rows = 0;
    let mut num_cols = 0;

    for row_idx in 1..grid.len() {
        let start = (2 * row_idx).saturating_sub(grid.len());
        let end = (2 * row_idx).min(grid.len());

        if std::iter::zip(
            grid[start..row_idx].iter().copied().flatten(),
            grid[row_idx..end].iter().rev().copied().flatten(),
        )
        .filter(|(l, r)| l != r)
        .count()
            == 1
        {
            num_rows = row_idx;
        }
    }

    for col_idx in 1..grid[0].len() {
        let start = (2 * col_idx).saturating_sub(grid[0].len());
        let end = (2 * col_idx).min(grid[0].len());

        if std::iter::zip(
            grid.iter().flat_map(|row| row[start..col_idx].iter()),
            grid.iter().flat_map(|row| row[col_idx..end].iter().rev()),
        )
        .filter(|(l, r)| l != r)
        .count()
            == 1
        {
            num_cols = col_idx;
        }
    }

    num_rows * 100 + num_cols
}
