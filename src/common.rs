pub use anyhow::{anyhow, bail, Context as _, Error};
use std::default::Default;
use std::cmp::{Ord, Ordering};

pub type Result<T = (), E = Error> = std::result::Result<T, E>;

pub fn default<T: Default>() -> T {
    T::default()
}

pub fn read_input(filename: &str) -> Result<Vec<String>> {
    use std::fs::File;
    use std::io::{BufRead, BufReader};

    let path = format!("inputs/{}", filename);
    let f = File::open(&path).with_context(|| format!("failed to open {}", path))?;

    BufReader::new(f)
        .lines()
        .collect::<Result<_, _>>()
        .with_context(|| format!("error while reading {}", path))
}

pub fn cmp<T: Ord>(lhs: T, rhs: T) -> Ordering {
    Ord::cmp(&lhs, &rhs)
}
