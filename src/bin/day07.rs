use eyre::eyre;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashSet;
use std::str::FromStr;

#[derive(Debug)]
struct Rules {
    bag_colors: Vec<String>,
    internal_bag_colors: Vec<Vec<String>>,
    internal_bag_counts: Vec<Vec<u32>>,
}

impl Rules {
    fn color_in(&self, color: &String) -> Vec<String> {
        // get all indices in color_mappings where color is contained
        self.internal_bag_colors
            .iter()
            .enumerate()
            .filter_map(|(i, cm)| {
                if cm.contains(color) {
                    self.bag_colors.get(i)
                } else {
                    None
                }
            })
            .cloned()
            .collect()
    }

    fn transitive(&self, start_color: &String) -> HashSet<String> {
        let mut colors: HashSet<String> = self.color_in(&start_color).into_iter().collect();

        let mut old_len = 0;
        while old_len != colors.len() {
            old_len = colors.len();
            colors = &colors
                | &(colors
                    .iter()
                    .flat_map(|c| self.color_in(c))
                    .collect::<HashSet<String>>());
        }

        colors
    }

    fn internal_bags_for(&self, color: &String) -> Option<(Vec<String>, Vec<u32>)> {
        let idx = self.bag_colors.iter().position(|c| c == color)?;
        self.internal_bag_colors
            .get(idx)
            .cloned()
            .zip(self.internal_bag_counts.get(idx).cloned())
    }

    fn internal_bag_count(&self, color: &String) -> u32 {
        self.internal_bags_for(color) // get internal bags and their counts for a color
            .map(|(a, b)| {
                a.into_iter()
                    .zip(b.into_iter()) // combine bag color with it's count
                    .map(|(s, n)| (self.internal_bag_count(&s) + 1) * n)
                    .sum()
            })
            .unwrap_or(0) // this happens when the color doesn't actually exist in the colors vec
    }
}

impl FromStr for Rules {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref COLOR_RE: Regex = Regex::new(r"(?P<color>^\w+\s\w+)").unwrap();
            static ref COLOR_COUNT_RE: Regex =
                Regex::new(r"(?P<count>\d+)\s(?P<color>\w+\s\w+)").unwrap();
        }

        let mut colors = Vec::new();
        let mut color_mappings = Vec::new();
        let mut count_mappings = Vec::new();

        for line in s.split("\n") {
            let color = String::from(
                COLOR_RE
                    .captures(&line)
                    .and_then(|c| c.name("color"))
                    .ok_or("error getting color")?
                    .as_str(),
            );

            let (color_mapping, count_mapping) = COLOR_COUNT_RE
                .captures_iter(&line)
                .map(|cap| {
                    cap.name("color").map(|m| String::from(m.as_str())).zip(
                        cap.name("count")
                            .and_then(|m| m.as_str().parse::<u32>().ok()),
                    )
                })
                .collect::<Option<Vec<(String, u32)>>>()
                .ok_or("error getting color counts")?
                .into_iter()
                .unzip::<String, u32, Vec<String>, Vec<u32>>();

            colors.push(color);
            color_mappings.push(color_mapping);
            count_mappings.push(count_mapping);
        }

        Ok(Rules {
            bag_colors: colors,
            internal_bag_colors: color_mappings,
            internal_bag_counts: count_mappings,
        })
    }
}

fn main() -> eyre::Result<()> {
    let rules = Rules::from_str(aoc_2020::input("07")?.as_ref()).map_err(|_| eyre!("error"))?;

    let start_color = String::from("shiny gold");

    let part_one_hs = rules.transitive(&start_color);
    let part_one = part_one_hs.len();
    println!("part one\n{}", part_one);

    let part_two = rules.internal_bag_count(&start_color);
    println!("part two\n{}", part_two);

    // part one
    // 119
    // part two
    // 155802

    Ok(())
}
