use structopt::clap::arg_enum;
use structopt::StructOpt;

arg_enum! {
    #[derive(Debug)]
    enum Step {
        A,
        B,
    }
}

#[derive(Debug, StructOpt)]
/// Advent of Code solutions.
///
/// Run without options to check all solutions.
struct Opt {
    /// Day: number from 1 to 25
    #[structopt(short, long, requires("step"))]
    day: Option<usize>,

    /// Step
    #[structopt(short, long, requires("day"), possible_values = &Step::variants(), case_insensitive = true)]
    step: Option<Step>,
}

fn main() {
    let opt = Opt::from_args();

    let num = opt.day.map(|n| n - 1);

    if let Some((step, num)) = opt.step.zip(num) {
        let result = match step {
            Step::A => aoc::SOLUTIONS[2 * num](aoc::FILES[num]),
            Step::B => aoc::SOLUTIONS[2 * num + 1](aoc::FILES[num]),
        };
        println!("{}", result);
    } else {
        for i in 0..aoc::SOLUTIONS.len() {
            let result = aoc::SOLUTIONS[i](aoc::FILES[i / 2]);
            println!("{}", result);
        }
    }
}
