mod common;
mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod gbcode;

use common::*;
use std::env;

fn main() -> Result {
    let funs = [
        day01::run,
        day02::run,
        day03::run,
        day04::run,
        day05::run,
        day06::run,
        day07::run,
        day08::run,
    ];

    let mut args = env::args();
    let binary = args.next().unwrap_or_default();
    let day = args.next().unwrap_or_default();
    //let rest: Vec<_> = args.collect();

    if let Ok(x) = day.parse::<usize>() {
        if x > 0 && x <= funs.len() {
            (funs[x - 1])()
        } else {
            bail!("day must be a number between 1 and {}", funs.len());
        }
    } else {
        bail!("usage: {} [day]", binary);
    }
}
