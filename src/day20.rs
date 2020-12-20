use crate::common::*;
use ndarray::prelude::*;

fn parse_input(lines: &[String]) -> Result<HashMap<usize, Array2<char>>> {
    let mut output = HashMap::default();
    let mut lines = lines.iter().map(|s| &**s);

    while let Some(line) = lines.next() {
        let index = find("^Tile ([0-9]+):$", line)
            .and_then(|c| c[1].parse::<usize>().ok())
            .ok_or(anyhow!("invalid line: {:?}", line))?;

        let mut map = vec![];
        for _ in 0..10 {
            let line = lines.next().unwrap_or_default();

            if line.len() != 10 {
                bail!("invalid line: {:?}", line);
            }

            map.extend(line.chars());
        }

        let map = Array::from(map).into_shape((10, 10))?;

        output.insert(index, map);
        lines.next();
    }

    Ok(output)
}

fn all_transformations<'a>(
    array: ArrayView2<'a, char>,
) -> impl Iterator<Item = ArrayView2<'a, char>> {
    (0..8).map(move |i| {
        let mut m = array;

        if i & 0x1 == 0 {
            m = m.reversed_axes()
        }

        if i & 0x2 == 0 {
            m = m.slice_move(s![..;-1, ..]);
        }

        if i & 0x4 == 0 {
            m = m.slice_move(s![.., ..;-1]);
        }

        m
    })
}

fn rearrange_maps(maps: &mut HashMap<usize, Array2<char>>) -> Array2<usize> {
    let keys: Vec<_> = maps.keys().copied().collect();
    let mut neighbors = HashMap::<usize, [Option<usize>; 4]>::default();
    let mut visited = HashSet::default();
    let mut queue = vec![keys[0]];

    while let Some(i) = queue.pop() {
        if !visited.insert(i) {
            continue;
        }

        println!("visit {:?}", i);
        let this = maps[&i].clone();
        let neighbors = neighbors.entry(i).or_default();

        for side in 0..4 {
            let mut neighbor = None;

            for &j in &keys {
                if i != j {
                    let that = maps.get_mut(&j).unwrap();
                    let mut iter = all_transformations(that.view());

                    while let Some(b) = iter.next() {
                        let a = this.view();

                        let matches = match side {
                            0 => a.row(0) == b.row(9),
                            1 => a.column(9) == b.column(0),
                            2 => a.row(9) == b.row(0),
                            3 => a.column(0) == b.column(9),
                            _ => unreachable!(),
                        };

                        if matches {
                            drop(iter);
                            *that = b.to_owned();
                            neighbor = Some(j);
                            queue.push(j);
                            break;
                        }
                    }
                }
            }

            neighbors[side] = neighbor;
        }
    }

    let mut grid = Array2::<usize>::zeros((12, 12));
    grid[[0, 0]] = neighbors
        .keys()
        .copied()
        .filter(|i| neighbors[i][0] == None && neighbors[i][3] == None)
        .next()
        .unwrap();

    for i in 0..12 {
        for j in 0..12 {
            let neighbors = neighbors[&grid[[i, j]]];

            if let Some(x) = neighbors[0] {
                grid[[i - 1, j]] = x;
            }

            if let Some(x) = neighbors[1] {
                grid[[i, j + 1]] = x;
            }

            if let Some(x) = neighbors[2] {
                grid[[i + 1, j]] = x;
            }

            if let Some(x) = neighbors[3] {
                grid[[i, j - 1]] = x;
            }
        }
    }

    grid
}

fn reassemble_maps(maps: &HashMap<usize, Array2<char>>, grid: ArrayView2<usize>) -> Array2<char> {
    let mut result = Array2::<char>::from_elem((96, 96), '0');

    for i in 0..12 {
        for j in 0..12 {
            let map = &maps[&grid[[i, j]]];

            let borderless = map.slice(s![1..9, 1..9]);
            result
                .slice_mut(s![8 * i..8 * (i + 1), 8 * j..8 * (j + 1)])
                .assign(&borderless);
        }
    }

    result
}

fn find_sea_monsters(map: ArrayView2<char>) -> Array2<char> {
    let drawing = [
        "                  # ",
        "#    ##    ##    ###",
        " #  #  #  #  #  #   ",
    ];

    let mut spots = vec![];
    let mut output = map.to_owned();

    for (di, line) in enumerate(&drawing) {
        for (dj, c) in enumerate(line.chars()) {
            if c == '#' {
                spots.push((di, dj));
            }
        }
    }

    for ((i, j), _) in map.indexed_iter() {
        let mut valid = true;

        for &(di, dj) in &spots {
            valid &= map.get([i + di, j + dj]) == Some(&'#');
        }

        if !valid {
            continue;
        }

        for &(di, dj) in &spots {
            output[[i + di, j + dj]] = 'O';
        }
    }

    output
}

fn print_map(map: ArrayView2<char>) {
    for row in map.genrows() {
        for c in row {
            print!("{}", c);
        }
        println!();
    }
}

pub fn run() -> Result {
    let mut maps = parse_input(&read_input("day20")?)?;

    let grid = rearrange_maps(&mut maps);
    println!(
        "part A: {}",
        grid[[0, 0]] * grid[[11, 0]] * grid[[0, 11]] * grid[[11, 11]]
    );

    let result = reassemble_maps(&maps, grid.view());

    println!("result:");
    print_map(result.view());
    println!();

    for m in all_transformations(result.view()) {
        let result = find_sea_monsters(m);

        if all(&result, |&c| c != 'O') {
            continue;
        }

        let count = result.iter().filter(|&&c| c == '#').count();

        println!("part B: {}", count);
        print_map(result.view());
        println!();
    }

    Ok(())
}
