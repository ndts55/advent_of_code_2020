#[derive(Debug, Clone, Copy)]
struct MinMax(u32, u32);

fn main() -> eyre::Result<()> {
    let start = (MinMax(0, 127), MinMax(0, 7));
    let seat_ids: Vec<u32> = aoc_2020::input_lines("05")?
        .into_iter()
        .map(|s| seat_id(s.chars().fold(start, rf)))
        .collect();

    let part_one = seat_ids.iter().max().cloned().unwrap_or(0);
    println!("part one\n{}", part_one);

    let minimum = seat_ids.iter().min().cloned().unwrap_or(0);

    let part_two = (minimum..part_one)
        .find(|n| !seat_ids.contains(n))
        .unwrap_or(0);

    println!("part two\n{}", part_two);

    Ok(())
}

fn rf(acc: (MinMax, MinMax), c: char) -> (MinMax, MinMax) {
    let (lower_upper, left_right) = acc;
    let left_right_middle = middle(left_right.0, left_right.1);
    let lower_upper_middle = middle(lower_upper.0, lower_upper.1);

    match c {
        'F' => (
            MinMax(lower_upper.0, lower_upper_middle.floor() as u32),
            left_right,
        ),
        'B' => (
            MinMax(lower_upper_middle.ceil() as u32, lower_upper.1),
            left_right,
        ),
        'L' => (
            lower_upper,
            MinMax(left_right.0, left_right_middle.floor() as u32),
        ),
        'R' => (
            lower_upper,
            MinMax(left_right_middle.ceil() as u32, left_right.1),
        ),
        _ => acc,
    }
}

fn middle(a: u32, b: u32) -> f64 {
    (a as f64 / 2f64) + (b as f64 / 2f64)
}

fn seat_id(mms: (MinMax, MinMax)) -> u32 {
    let row = mms.0 .0;
    let col = mms.1 .0;

    row * 8 + col
}
