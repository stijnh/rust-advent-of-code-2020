mod common;
mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;
mod day21;
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
        day09::run,
        day10::run,
        day11::run,
        day12::run,
        day13::run,
        day14::run,
        day15::run,
        day16::run,
        day17::run,
        day18::run,
        day19::run,
        day20::run,
        day21::run,
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
