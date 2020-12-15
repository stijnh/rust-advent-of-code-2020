use crate::common::*;

fn parse_input(line: &str) -> Result<Vec<usize>> {
    line.split(',').map(|s| Ok(s.parse()?)).collect()
}

fn play_for_n_rounds(n: usize, initial_numbers: &[usize]) -> usize {
    let mut spoken = HashMap::default();
    let mut now_spoken = initial_numbers[0];

    for turn in 0..n {
        let next_spoken = if let Some(&v) = initial_numbers.get(turn + 1) {
            v
        } else if let Some(&prev_turn) = spoken.get(&now_spoken) {
            turn - prev_turn
        } else {
            0
        };

        spoken.insert(now_spoken, turn);
        now_spoken = next_spoken;
    }

    now_spoken
}

pub fn run() -> Result {
    let numbers = parse_input(&read_input("day15")?[0])?;

    let result = play_for_n_rounds(2020 - 1, &numbers);
    println!("part A: {}", result);

    let result = play_for_n_rounds(30_000_000 - 1, &numbers);
    println!("part B: {}", result);

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let input = [0, 3, 6];
        let expected_output = [0, 3, 6, 0, 3, 3, 1, 0, 4, 0];

        let gotten_output = (0..10)
            .map(|i| play_for_n_rounds(i, &input))
            .collect::<Vec<_>>();

        assert_eq!(expected_output, &*gotten_output);
    }
}
