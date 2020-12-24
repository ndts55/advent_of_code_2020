use aoc_2020::input_lines;
use itertools::iproduct;
use itertools::Itertools;

fn main() -> eyre::Result<()> {
    let numbers: Vec<u32> = input_lines("01")?
        .iter()
        .filter_map(|n| n.parse::<u32>().ok())
        .collect();

    let part_one = numbers
        .iter()
        .cartesian_product(numbers.clone())
        .find(|&(&a, b)| a + b == 2020)
        .map(|(&a, b)| a * b)
        .unwrap();

    println!("part one\n{}", part_one);

    let part_two = iproduct!(numbers.clone(), numbers.clone(), numbers)
        .find(|&(a, b, c)| a + b + c == 2020)
        .map(|(a, b, c)| a * b * c)
        .unwrap();

    println!("part two\n{}", part_two);

    Ok(())
}
