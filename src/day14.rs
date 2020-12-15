use crate::common::*;
use recap::Recap;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug)]
enum Instr {
    Mask(u64, u64),
    Assign(u64, u64),
}

fn parse_input(lines: &[String]) -> Result<Vec<Instr>> {
    #[derive(Recap, Deserialize)]
    #[recap(regex="mask = (?P<mask>[01X]+)")]
    struct Mask { mask: String };

    #[derive(Recap, Deserialize)]
    #[recap(regex="mem\\[(?P<addr>[0-9]+)\\] = (?P<value>[0-9]+)")]
    struct Assign { addr: u64, value: u64 };
        

    let mut output = vec![];
    for line in lines {
        if let Ok(Mask { mask }) = line.parse() {
            let value = u64::from_str_radix(&mask.replace('X', "0"), 2).unwrap();
            let mask = u64::from_str_radix(&mask.replace(&['0', '1'] as &[char], "1").replace('X', "0"), 2).unwrap();
            output.push(Instr::Mask(mask, value));

        } else if let Ok(Assign { addr, value }) = line.parse() {
            output.push(Instr::Assign(addr, value));
        } else {
            bail!("failed to parse line: {}", line);
        }
    }

    Ok(output)
}

fn execute(instrs: &[Instr]) -> HashMap<u64, u64> {
    use Instr::*;
    let mut mem = HashMap::new();
    let mut mask_flags = 0;
    let mut mask_values = 0;
    
    for instr in instrs {
        match instr {
            &Mask(m, v) => {
                mask_flags = m;
                mask_values = v & mask_flags;
            },
            &Assign(addr, value) => {
                let v = (value & !mask_flags) | mask_values;
                mem.insert(addr, v);
            }
        }
    }

    mem
}

pub fn run() -> Result {
    let instrs = parse_input(&read_input("day14")?)?;
    let mem = execute(&instrs);

    let sum = sum(map(mem.values(), |&v| v));
    println!("part A: {:?}", sum);

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let program = vec![
            "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X".to_string(),
            "mem[8] = 11".to_string(),
            "mem[7] = 101".to_string(),
            "mem[8] = 0".to_string(),
        ];

        let program = parse_input(&program).unwrap();
        let mem = execute(&program);
    }
}
