use crate::common::*;
use itertools::iproduct;
use std::collections::HashSet;
use std::ops::RangeInclusive;

fn find_xmass_weakness(numbers: &[usize]) -> Option<usize> {
    let mut combinations = HashSet::new();

    for (i, &x) in enumerate(numbers) {
        if i >= 25 && !combinations.contains(&x) {
            return Some(i);
        }

        for y in &numbers[..i] {
            combinations.insert(x + y);
        }
    }

    None
}

fn find_range(numbers: &[usize], needle: usize) -> Option<(usize, usize)> {
    let mut sums = vec![0; numbers.len()];

    for n in 0..numbers.len() {
        for i in 0..(numbers.len() - n) {
            sums[i] += numbers[i + n];

            if sums[i] == needle {
                return Some((i, i + n));
            }
        }
    }

    None
}

pub fn run() -> Result {
    let numbers = read_input("day09")?
        .into_iter()
        .map(|s| s.parse::<usize>())
        .collect::<Result<Vec<_>, _>>()?;

    let index = find_xmass_weakness(&numbers).ok_or(anyhow!("number not found"))?;
    println!("part A: {}", numbers[index]);

    let (begin, end) =
        find_range(&numbers[..index], numbers[index]).ok_or(anyhow!("range not found"))?;

    let min = numbers[begin..=end].iter().max().unwrap();
    let max = numbers[begin..=end].iter().min().unwrap();

    println!("part B: {:?}", min + max);

    Ok(())
}
