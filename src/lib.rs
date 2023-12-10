use std::time::Instant;

pub mod y2015;
pub mod y2021;
pub mod y2023;

pub fn timing_fn<T, F>(f: F) -> T
where
    F: Fn() -> T,
{
    let now = Instant::now();
    let ret = f();
    eprintln!("Finished in {:?}", now.elapsed());
    ret
}
