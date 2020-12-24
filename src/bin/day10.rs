use eyre::eyre;
use itertools::Itertools;
use std::collections::BTreeMap;
use std::iter;
use std::num::ParseIntError;

fn main() -> eyre::Result<()> {
    let joltages = aoc_2020::input_lines("10")?
        .iter()
        .map(|s| s.parse::<i32>())
        .collect::<Result<Vec<i32>, ParseIntError>>()?;

    let device_joltage = joltages
        .iter()
        .max()
        .cloned()
        .ok_or(eyre!("joltages is empty!"))?
        + 3;

    let output_joltage = 0;

    // part one
    // Your puzzle answer was 1755.
    println!(
        "part one\n{}",
        part_one(&joltages, device_joltage, output_joltage)
    );

    // part two
    // Your puzzle answer was 4049565169664.
    println!(
        "part two\n{}",
        part_two(&joltages, device_joltage, output_joltage)
    );

    Ok(())
}

fn part_one(joltages: &Vec<i32>, device_joltage: i32, output_joltage: i32) -> i32 {
    let joltages: Vec<i32> = joltages.into_iter().sorted().cloned().collect();

    let j0 = joltages
        .clone()
        .into_iter()
        .chain(iter::once(device_joltage));
    let j1 = iter::once(output_joltage).chain(joltages.into_iter());
    let diffs = j0.zip(j1).map(|(a, b)| a - b).collect_vec();
    let ones = diffs.iter().filter(|&&d| d == 1).count();
    let threes = diffs.iter().filter(|&&d| d == 3).count();
    (ones * threes) as i32
}

fn part_two(joltages: &Vec<i32>, device_joltage: i32, output_joltage: i32) -> i64 {
    // traverse joltages backwards and build up cache mapping of number
    // of possible paths from that joltage
    let joltages = joltages
        .into_iter()
        .cloned()
        .map_into::<i64>()
        .collect_vec();
    let device_joltage = device_joltage as i64;
    let output_joltage = output_joltage as i64;
    joltages
        .iter()
        .chain(iter::once(&output_joltage))
        .sorted()
        .rev()
        .fold(BTreeMap::new(), |mut acc, &joltage| {
            if device_joltage - joltage <= 3 {
                acc.insert(joltage, 1);
                return acc;
            }
            let number_of_paths: i64 = joltages
                .iter()
                .filter_map(|j| {
                    let d = j - joltage;
                    // get possible next joltages
                    if 0 < d && d <= 3 {
                        // get number of paths for these joltages from acc
                        acc.get(j)
                    } else {
                        None
                    }
                })
                .cloned()
                .sum(); // that's how many paths are possible from this joltage

            // create a new entry in the acc
            acc.insert(joltage, number_of_paths);

            acc
        })
        .get(&output_joltage)
        .cloned()
        .unwrap_or(0)
}
