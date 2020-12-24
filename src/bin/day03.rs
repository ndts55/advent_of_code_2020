use std::iter;

fn main() -> eyre::Result<()> {
    let forest = aoc_2020::input_lines("03")?;

    let part_one = tree_count(&forest, 3, 1);
    println!("part one\n{}", part_one);

    let part_two: u32 = vec![(1, 1), (5, 1), (7, 1), (1, 2)]
        .into_iter()
        .map(|(right, down)| tree_count(&forest, right, down))
        .product::<u32>()
        * part_one;

    println!("part two\n{}", part_two);

    Ok(())
}

fn tree_count(forest: &Vec<String>, right: usize, down: usize) -> u32 {
    let mut x = 0;
    let mut y = 0;

    let height = forest.len();
    if height <= 0 {
        return 0u32;
    }
    let width = forest.get(0).unwrap().len();
    // let width = forest.get(0).unwrap().len();

    iter::from_fn(move || {
        if y < height {
            let n = Some((x, y));
            x = (x + right) % width;
            y += down;
            n
        } else {
            None
        }
    })
    .filter_map(|(col, row)| {
        // if character at specified position is a tree map to 1
        forest
            .get(row as usize)
            .and_then(|line| line.chars().nth(col as usize))
            .filter(|&c| c == '#')
            .map(|_| 1u32)
    })
    .sum()
}
