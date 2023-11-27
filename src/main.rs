use std::{fs, io, time::Instant};

use arboard::Clipboard;
use clap::Parser;
use color_eyre::{eyre::WrapErr, Result};

/// Solve an Advent of Code problem
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Year of the problem
    #[arg(value_parser = clap::value_parser!(u16).range(2015..=2022))]
    year: u16,

    /// Day of the problem
    #[arg(value_parser = clap::value_parser!(u8).range(1..=25))]
    day: u8,

    /// Part of the problem
    #[arg(value_parser = clap::value_parser!(u8).range(1..=2))]
    part: u8,
}

fn main() -> Result<()> {
    // Nice and colorful errors
    color_eyre::install()?;
    // Get session cookie from env file
    dotenvy::dotenv()?;

    let Args { year, day, part } = Args::parse();

    for file in FILES {
        fs::create_dir_all(file)?;
    }

    let input = get_problem(year, day)?;

    let result = match year {
        2015 => timing_fn(|| aoc::y2015::solve(day, part, &input)),
        2021 => timing_fn(|| aoc::y2021::solve(day, part, &input)),
        _ => unimplemented!(),
    };

    println!("{result}");

    Clipboard::new()?.set_text(result.to_string())?;

    Ok(())
}

fn get_problem(year: u16, day: u8) -> Result<String> {
    let file_path = format!("./files/{year}/day{day:02}.txt");

    match fs::read_to_string(&file_path) {
        // If file exists, return the file
        Ok(aoc_problem) => return Ok(aoc_problem),
        // If os_error, return the error
        Err(e) if e.kind() != io::ErrorKind::NotFound => {
            return Err(e).wrap_err("couldn't access cached file")
        }
        // If file not found, continue
        Err(_) => {
            // TODO: use tracing
            eprintln!("file not in cache");
        }
    };

    let session_cookie = std::env::var("AOC_SESSION").wrap_err("AOC_SESSION not found")?;
    let user_agent = std::env::var("USER_AGENT").wrap_err("USER_AGENT not found")?;

    let cookie = ureq::Cookie::new("session", session_cookie);

    let response = ureq::get(&format!("https://adventofcode.com/{year}/day/{day}/input"))
        .set("Cookie", &cookie.to_string())
        .set("User-Agent", &user_agent)
        .call()
        .wrap_err("maybe too soon")?
        .into_string()?;

    // Cache the file
    fs::write(file_path, &response)?;

    Ok(response)
}

fn timing_fn<T, F>(f: F) -> T
where
    F: Fn() -> T,
{
    let now = Instant::now();
    let ret = f();
    eprintln!("Finished in {:?}", now.elapsed());
    ret
}

const FILES: [&str; 9] = [
    "./files/2015",
    "./files/2016",
    "./files/2017",
    "./files/2018",
    "./files/2019",
    "./files/2020",
    "./files/2021",
    "./files/2022",
    "./files/2023",
];
