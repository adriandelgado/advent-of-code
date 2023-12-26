#![allow(clippy::cast_possible_truncation, clippy::cast_precision_loss)]

use faer::{mat, prelude::*, Mat};
use itertools::Itertools;
use winnow::{ascii::dec_int, combinator::separated_pair, PResult, Parser};

pub fn part1(input: &str) -> usize {
    input
        .lines()
        .map(|line| pos_vel.parse(line.as_bytes()).unwrap())
        .tuple_combinations()
        .filter_map(|(pv_0, pv_1)| check_intersection_2d(pv_0, pv_1))
        .count()
}

pub fn part2(input: &str) -> i64 {
    input
        .lines()
        .map(|line| pos_vel.parse(line.as_bytes()).unwrap())
        .tuples()
        .find_map(|((p_0, v_0), (p_1, v_1), (p_2, v_2))| {
            let (dp_01, dp_02, dp_12) =
                (sub_arr3(p_0, p_1), sub_arr3(p_0, p_2), sub_arr3(p_1, p_2));
            let (dv_01, dv_02, dv_12) =
                (sub_arr3(v_0, v_1), sub_arr3(v_0, v_2), sub_arr3(v_1, v_2));
            let (cross_v01, cross_v02, cross_v12) =
                (cross(v_0, v_1), cross(v_0, v_2), cross(v_1, v_2));
            let cross_pv_01 = cross(dp_01, dv_01);
            let cross_pv_02 = cross(dp_02, dv_02);
            let cross_pv_12 = cross(dp_12, dv_12);

            let dp_01 = Mat::from_fn(3, 1, |i, _| dp_01[i] as f64);
            let dp_02 = Mat::from_fn(3, 1, |i, _| dp_02[i] as f64);
            let dp_12 = Mat::from_fn(3, 1, |i, _| dp_12[i] as f64);

            let cross_v01 = faer::mat::from_column_major_slice::<f64>(&cross_v01, 3, 1);
            let cross_v02 = faer::mat::from_column_major_slice::<f64>(&cross_v02, 3, 1);
            let cross_v12 = faer::mat::from_column_major_slice::<f64>(&cross_v12, 3, 1);

            let cross_pv_01 = faer::mat::from_column_major_slice::<f64>(&cross_pv_01, 3, 1)
                * mat![[1.0, 0.0, 0.0]];
            let cross_pv_02 = faer::mat::from_column_major_slice::<f64>(&cross_pv_02, 3, 1)
                * mat![[0.0, 1.0, 0.0]];
            let cross_pv_12 = faer::mat::from_column_major_slice::<f64>(&cross_pv_12, 3, 1)
                * mat![[0.0, 0.0, 1.0]];
            let sq_mat = cross_pv_01 + cross_pv_02 + cross_pv_12;
            let coefs = mat![
                [(dp_01.transpose() * cross_v01)[(0, 0)]],
                [(dp_02.transpose() * cross_v02)[(0, 0)]],
                [(dp_12.transpose() * cross_v12)[(0, 0)]]
            ];

            if sq_mat.determinant().abs() < 0.1 {
                return None;
            }

            let rock_velocity = sq_mat.partial_piv_lu().solve_transpose(coefs);

            let velocity_matrix = Mat::from_fn(3, 2, |r, c| [v_1, v_0][c][r] as f64)
                - &rock_velocity * mat![[1.0, 1.0]];

            let rock_time_1 = velocity_matrix.qr().solve_lstsq(dp_01)[(0, 0)];

            Some(
                velocity_matrix
                    .col_as_slice(0)
                    .iter()
                    .map(|v| v * rock_time_1)
                    .zip(p_1)
                    .map(|(v, p)| v.round() as i64 + p)
                    .sum(),
            )
        })
        .unwrap()
}

fn check_intersection_2d(
    ([px_0, py_0, _pz_0], [vx_0, vy_0, _vz_0]): ([i64; 3], [i64; 3]),
    ([px_1, py_1, _pz_1], [vx_1, vy_1, _vz_1]): ([i64; 3], [i64; 3]),
) -> Option<[i64; 2]> {
    let determinant = vx_0 * vy_1 - vy_0 * vx_1;

    // Check parallel
    if determinant == 0 {
        return None;
    }

    let coef_v_0 = px_0 * vy_0 - py_0 * vx_0;
    let coef_v_1 = px_1 * vy_1 - py_1 * vx_1;

    let x = (coef_v_0 / determinant * -vx_1) + (coef_v_1 / determinant * vx_0);
    let y = (coef_v_0 / determinant * -vy_1) + (coef_v_1 / determinant * vy_0);

    // Check future
    if (x - px_0) * vx_0.signum() < 0 || (x - px_1) * vx_1.signum() < 0 {
        return None;
    }

    if !(200_000_000_000_000..=400_000_000_000_000).contains(&x)
        || !(200_000_000_000_000..=400_000_000_000_000).contains(&y)
    {
        return None;
    }

    Some([x, y])
}

fn sub_arr3([x0, y0, z0]: [i64; 3], [x1, y1, z1]: [i64; 3]) -> [i64; 3] {
    [x0 - x1, y0 - y1, z0 - z1]
}

fn cross([x0, y0, z0]: [i64; 3], [x1, y1, z1]: [i64; 3]) -> [f64; 3] {
    [
        (y0 * z1 - z0 * y1) as f64,
        (z0 * x1 - x0 * z1) as f64,
        (x0 * y1 - y0 * x1) as f64,
    ]
}

fn arr3(input: &mut &[u8]) -> PResult<[i64; 3]> {
    (dec_int, b", ", dec_int, b", ", dec_int)
        .map(|(x, _, y, _, z)| [x, y, z])
        .parse_next(input)
}

fn pos_vel(input: &mut &[u8]) -> PResult<([i64; 3], [i64; 3])> {
    separated_pair(arr3, b" @ ", arr3).parse_next(input)
}
