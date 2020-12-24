use eyre::eyre;
use itertools::{Itertools, MinMaxResult};
use std::collections::BTreeSet;
use std::iter::FromIterator;
use std::num::ParseIntError;

fn main() -> eyre::Result<()> {
    let numbers = aoc_2020::input_lines("09")?
        .into_iter()
        .map(|s| s.parse::<i64>())
        .collect::<Result<Vec<i64>, ParseIntError>>()?;

    // part one
    let part_one = numbers
        .iter()
        .skip(25)
        .enumerate()
        .find_map(|(i, &x)| {
            // call is_misfit with previous 25 numbers and n
            if is_misfit(&numbers[i..i + 25], x) {
                Some(x)
            } else {
                None
            }
        })
        .ok_or(eyre!("failed to find misfit"))?;
    // Your puzzle answer was 25918798.
    println!("part one\n{}", part_one);

    // part two
    let (start, end) = find_vulnerable_range(&numbers, part_one)
        .ok_or(eyre!("failed to find vulnerable range"))?;
    let part_two = p2_sum(&numbers, start, end).ok_or(eyre!("failed to calculate p2 sum"))?;
    // Your puzzle answer was 3340942.
    println!("part two\n{}", part_two);

    Ok(())
}

fn is_misfit(ns: &[i64], x: i64) -> bool {
    // create set from ns
    let ns = BTreeSet::from_iter(ns.iter().cloned());
    ns.iter().find(|n| ns.contains(&(x - (*n)))).is_none()
}

fn find_vulnerable_range(numbers: &Vec<i64>, target: i64) -> Option<(usize, usize)> {
    for start in 0..numbers.len() {
        for end in start + 2..numbers.len() + 1 {
            let s: i64 = numbers[start..end].iter().sum();
            if s > target {
                break;
            }
            if s == target {
                return Some((start, end));
            }
        }
    }

    None
}

fn p2_sum(numbers: &Vec<i64>, start: usize, end: usize) -> Option<i64> {
    let minmax = numbers
        .iter()
        .skip(start)
        .take(end - start)
        .cloned()
        .minmax();
    Some(match minmax {
        MinMaxResult::MinMax(min, max) => min + max,
        MinMaxResult::OneElement(n) => n + n,
        _ => return None,
    })
}
