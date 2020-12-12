use crate::common::*;

fn parse_input(lines: &[String]) -> Result<Vec<(char, i32)>> {
    lines
        .iter()
        .map(|line| {
            let c = line.chars().next().unwrap_or_default();
            let i = line[1..]
                .parse()
                .with_context(|| format!("while parsing line {:?}", line))?;

            Ok((c, i))
        })
        .collect()
}

fn rotate((mut x, mut y): (i32, i32), angle: i32) -> (i32, i32) {
    match i32::rem_euclid(angle, 360) {
        0 => (x, y),
        90 => (y, -x),
        180 => (-x, -y),
        270 => (-y, x),
        _ => panic!("invalid angle"),
    }
}

fn execute(instr: &[(char, i32)]) -> (i32, i32) {
    let (mut x, mut y) = (0, 0);
    let mut dir = (1, 0);

    for &(c, v) in instr {
        match c {
            'N' => y += v,
            'S' => y -= v,
            'E' => x += v,
            'W' => x -= v,
            'R' => dir = rotate(dir, v),
            'L' => dir = rotate(dir, -v),
            'F' => {
                let (dx, dy) = dir;
                x += dx * v;
                y += dy * v;
            }
            _ => println!("unknown command: {}", c),
        }
    }

    (x, y)
}

fn execute_real(instr: &[(char, i32)]) -> (i32, i32) {
    let (mut sx, mut sy) = (0, 0);
    let (mut wx, mut wy) = (10, 1);

    for &(c, v) in instr {
        match c {
            'N' => wy += v,
            'S' => wy -= v,
            'E' => wx += v,
            'W' => wx -= v,
            'R' => {
                let (x, y) = rotate((wx, wy), v);
                wx = x;
                wy = y;
            }
            'L' => {
                let (x, y) = rotate((wx, wy), -v);
                wx = x;
                wy = y;
            }
            'F' => {
                sx += wx * v;
                sy += wy * v;
            }
            _ => println!("unknown command: {}", c),
        }
    }

    (sx, sy)
}

pub fn run() -> Result {
    let instr = parse_input(&read_input("day12")?)?;

    let (x, y) = execute(&instr);
    println!("part A: ({}, {}) -> {}", x, y, x.abs() + y.abs());

    let (x, y) = execute_real(&instr);
    println!("part B: ({}, {}) -> {}", x, y, x.abs() + y.abs());

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let instr = vec![
            "F10".to_string(),
            "N3".to_string(),
            "F7".to_string(),
            "R90".to_string(),
            "F11".to_string(),
        ];

        let instr = parse_input(&instr).unwrap();

        let (x, y) = execute(&instr);
        assert_eq!((x, y), (17, -8));

        let (x, y) = execute_real(&instr);
        assert_eq!((x, y), (214, -72));
    }
}
