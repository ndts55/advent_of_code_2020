use std::collections::BTreeSet;
use std::iter::FromIterator;

use itertools::Itertools;

fn main() -> eyre::Result<()> {
    let contents = aoc_2020::input_paragraphs("06")?;
    let part_one: usize = contents
        .clone()
        .into_iter()
        .map(|s| s.replace("\n", "").chars().unique().count())
        .sum();

    println!("part one\n{}", part_one);

    let part_two: usize = contents
        .into_iter()
        .filter_map(|s| {
            s.split("\n")
                .into_iter()
                .map(|answer| BTreeSet::from_iter(answer.chars()))
                .fold1(|acc, b| acc.intersection(&b).cloned().collect())
                .map(|b| b.len())
        })
        .sum();

    println!("part two\n{}", part_two);

    Ok(())
}
