mod common;
mod day01;

use common::*;
use std::env;

fn main() -> Result {
    let funs = [day01::run];

    let mut args = env::args();
    let binary = args.next().unwrap_or_default();
    let day = args.next().unwrap_or_default();
    //let rest: Vec<_> = args.collect();

    if let Ok(x) = day.parse::<usize>() {
        if x > 0 && x <= funs.len() {
            (funs[x - 1])()
        } else {
            bail!("day must be a number between 1 and {}", funs.len() + 1);
        }
    } else {
        bail!("usage: {} [day]", binary);
    }
}
