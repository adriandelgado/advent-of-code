use ndarray::{array, s, Array2};
use winnow::{
    ascii::{alpha1, dec_int},
    combinator::preceded,
    PResult, Parser,
};

const TOTAL_INGREDIENTS: i64 = 100;

pub fn part1(input: &str) -> i64 {
    let ingredients: Vec<[i64; 5]> = input
        .lines()
        .map(|line| parse_line.parse(line).unwrap())
        .collect();

    let mut maximum = 0;

    let arr = Array2::from(ingredients).reversed_axes();
    for a in 0..=TOTAL_INGREDIENTS {
        for b in 0..=(TOTAL_INGREDIENTS - a) {
            for c in 0..=(TOTAL_INGREDIENTS - (a + b)) {
                let d = 100 - (a + b + c);
                let arr_100 = array![a, b, c, d];

                let dot = arr.dot(&arr_100);

                #[allow(clippy::reversed_empty_ranges)]
                let product = dot
                    .slice_move(s![0..-1])
                    .mapv(|n| if n.is_negative() { 0 } else { n })
                    .product();

                maximum = maximum.max(product);
            }
        }
    }

    maximum
}

pub fn part2(input: &str) -> i64 {
    let ingredients: Vec<[i64; 5]> = input
        .lines()
        .map(|line| parse_line.parse(line).unwrap())
        .collect();

    let mut maximum = 0;

    let arr = Array2::from(ingredients).reversed_axes();
    for a in 0..=TOTAL_INGREDIENTS {
        for b in 0..=(TOTAL_INGREDIENTS - a) {
            for c in 0..=(TOTAL_INGREDIENTS - (a + b)) {
                let d = 100 - (a + b + c);
                let arr_100 = array![a, b, c, d];

                let dot = arr.dot(&arr_100);

                // part 2
                if dot[4] != 500 {
                    continue;
                };

                #[allow(clippy::reversed_empty_ranges)]
                let product = dot
                    .slice_move(s![0..-1])
                    .mapv(|n| if n.is_negative() { 0 } else { n })
                    .product();

                maximum = maximum.max(product);
            }
        }
    }

    maximum
}

fn parse_line(input: &mut &str) -> PResult<Ingredient> {
    preceded(
        alpha1,
        (
            preceded(": capacity ", dec_int),
            preceded(", durability ", dec_int),
            preceded(", flavor ", dec_int),
            preceded(", texture ", dec_int),
            preceded(", calories ", dec_int),
        ),
    )
    .output_into()
    .parse_next(input)
}

type Ingredient = [i64; 5];
