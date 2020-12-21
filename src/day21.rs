use crate::common::*;
use std::iter::FromIterator;
use std::str::FromStr;

#[derive(Debug)]
struct Food {
    ingredients: Vec<String>,
    allergens: Vec<String>,
}

impl FromStr for Food {
    type Err = anyhow::Error;

    fn from_str(line: &str) -> Result<Self> {
        let m = find("^([a-z ]*) [(]contains ([a-z, ]+)[)]$", line)
            .ok_or_else(|| anyhow!("invalid line {:?}", line))?;

        Ok(Self {
            ingredients: m[1].split(' ').map(str::to_string).collect(),
            allergens: m[2].split(", ").map(str::to_string).collect(),
        })
    }
}

fn parse_input(lines: &[String]) -> Result<Vec<Food>> {
    map(lines, |s| s.parse()).collect()
}

fn find_allergens(foods: &[Food]) -> Result<HashMap<String, String>> {
    let mut output: HashMap<String, String> = default();
    let mut options: HashMap<String, HashSet<String>> = default();

    for food in foods {
        let ingredients = HashSet::from_iter(food.ingredients.clone());

        for allergen in &food.allergens {
            options
                .entry(allergen.to_string())
                .and_modify(|s| *s = &*s & &ingredients)
                .or_insert_with(|| ingredients.clone());
        }
    }

    while options.len() > 0 {
        let allergen = options
            .keys()
            .filter(|&k| options[k].len() == 1)
            .next()
            .ok_or_else(|| anyhow!("no allergens found"))?
            .to_string();
        let ingredient = options.remove(&allergen).unwrap().drain().next().unwrap();

        for v in options.values_mut() {
            v.remove(&ingredient);
        }

        output.insert(ingredient, allergen);
    }

    Ok(output)
}

pub fn run() -> Result {
    let foods = parse_input(&read_input("day21")?)?;

    let ing2all = find_allergens(&foods)?;

    let mut count = 0;
    for food in &foods {
        for ing in &food.ingredients {
            if !ing2all.contains_key(ing) {
                count += 1;
            }
        }
    }

    println!("part A: {}", count);

    let list = ing2all
        .iter()
        .sorted_by(|(_, a1), (_, a2)| a1.cmp(a2))
        .map(|(ing, _)| ing)
        .join(",");

    println!("pat B: {}", list);


    Ok(())
}
