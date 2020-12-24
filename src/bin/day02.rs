use aoc_2020::input_lines;
use lazy_static::lazy_static;
use regex::Regex;
use std::convert::TryFrom;

struct Rule {
    minimum: u32,
    maximum: u32,
    letter: char,
    password: String,
}

impl Rule {
    fn is_policy_1_valid(&self) -> bool {
        // get letter count
        let letter_count = self.password.chars().filter(|&c| c == self.letter).count() as u32;
        // check if letter count is in range
        (self.minimum <= letter_count) && (letter_count <= self.maximum)
    }

    fn is_policy_2_valid(&self) -> bool {
        let eq_letter = |c: &char| (*c) == self.letter;
        // get letters at minimum and maximum and xor == letter
        self.char_at(self.minimum as usize - 1)
            .filter(eq_letter)
            .xor(self.char_at(self.maximum as usize - 1).filter(eq_letter))
            .is_some()
    }

    fn char_at(&self, position: usize) -> Option<char> {
        self.password.chars().nth(position)
    }
}

impl TryFrom<String> for Rule {
    type Error = &'static str;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        lazy_static! {
            static ref RE: Regex = Regex::new(
                r"(?P<minimum>\d+)-(?P<maximum>\d+)\s(?P<letter>[a-z]):\s(?P<password>\w+)"
            )
            .unwrap();
        }
        // regex s
        let caps = RE.captures(&s).ok_or("malformed string")?;
        // extract captures
        let minimum = caps
            .name("minimum")
            .ok_or("no minimum")?
            .as_str()
            .parse::<u32>()
            .map_err(|_| "error parsing minimum")?;
        let maximum = caps
            .name("maximum")
            .ok_or("no maximum")?
            .as_str()
            .parse::<u32>()
            .map_err(|_| "error parsing maximum")?;
        let letter = caps
            .name("letter")
            .ok_or("no letter")?
            .as_str()
            .parse::<char>()
            .map_err(|_| "error parsing char")?;
        let password = caps
            .name("password")
            .ok_or("no password")?
            .as_str()
            .to_owned();
        // return value
        Ok(Rule {
            minimum,
            maximum,
            letter,
            password,
        })
    }
}

fn main() -> eyre::Result<()> {
    let rules: Vec<Rule> = input_lines("02")?
        .into_iter()
        .filter_map(|s| Rule::try_from(s).ok())
        .collect();

    let part_one = rules.iter().filter(|r| r.is_policy_1_valid()).count();

    println!("part one\n{}", part_one);

    let part_two = rules.iter().filter(|r| r.is_policy_2_valid()).count();

    println!("part two\n{}", part_two);

    Ok(())
}
