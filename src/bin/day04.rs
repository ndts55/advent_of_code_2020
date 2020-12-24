#[derive(Debug, Clone, Copy, PartialEq)]
enum Height {
    Cm(u32),
    In(u32),
    Raw(u32),
}

impl Height {
    fn is_valid(&self) -> bool {
        match self {
            Height::Cm(n) => (150..=193).contains(n),
            Height::In(n) => (59..=76).contains(n),
            _ => false,
        }
    }
}

impl From<&str> for Height {
    fn from(value: &str) -> Self {
        let number = value
            .chars()
            .filter(char::is_ascii_digit)
            .collect::<String>()
            .parse::<u32>()
            .unwrap_or(0);

        if value.contains("cm") {
            Height::Cm(number)
        } else if value.contains("in") {
            Height::In(number)
        } else {
            Height::Raw(number)
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
struct HairColor(String);

impl HairColor {
    fn is_valid(&self) -> bool {
        // #[0-9a-f]{6}
        self.0.len() == 7
            && self.0.chars().nth(0).filter(|&c| c == '#').is_some()
            && self.0.chars().skip(1).all(|c| c.is_ascii_hexdigit())
    }
}

#[derive(Debug, Clone, PartialEq)]
struct PID(String);

impl PID {
    fn is_valid(&self) -> bool {
        self.0.len() == 9 && self.0.chars().all(|c| c.is_ascii_digit())
    }
}

#[derive(Debug, Clone, PartialEq)]
struct EyeColor(String);

impl EyeColor {
    fn is_valid(&self) -> bool {
        // amb blu brn gry grn hzl oth
        match self.0.as_ref() {
            "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => true,
            _ => false,
        }
    }
}

#[derive(Debug, PartialEq)]
struct Passport {
    byr: Option<u32>,
    iyr: Option<u32>,
    eyr: Option<u32>,
    hgt: Option<Height>,
    hcl: Option<HairColor>,
    ecl: Option<EyeColor>,
    pid: Option<PID>,
}

impl Passport {
    fn is_valid_1(&self) -> bool {
        self.byr.is_some()
            && self.iyr.is_some()
            && self.eyr.is_some()
            && self.hgt.is_some()
            && self.hcl.is_some()
            && self.ecl.is_some()
            && self.pid.is_some()
    }

    fn is_valid_2(&self) -> bool {
        self.is_valid_1()
            && self
                .byr
                .filter(|n| (1920..=2002).contains(n))
                .and(self.iyr)
                .filter(|n| (2010..=2020).contains(n))
                .and(self.eyr)
                .filter(|n| (2020..=2030).contains(n))
                .and(self.hgt)
                .filter(Height::is_valid)
                .and(self.hcl.as_ref())
                .filter(|h| h.is_valid())
                .and(self.ecl.as_ref())
                .filter(|e| e.is_valid())
                .and(self.pid.as_ref())
                .filter(|p| p.is_valid())
                .is_some()
    }
}

impl From<String> for Passport {
    fn from(value: String) -> Self {
        let entries: Vec<&str> = value.split_ascii_whitespace().collect();

        let extract_entry = |name| {
            entries
                .iter()
                .find(|e| e.starts_with(name))
                .map(|e| e.split(":").nth(1))
                .flatten()
        };

        let as_u32 = |v: &str| v.parse::<u32>().ok();

        let byr = extract_entry("byr").and_then(as_u32);
        let iyr = extract_entry("iyr").and_then(as_u32);
        let eyr = extract_entry("eyr").and_then(as_u32);
        let hgt = extract_entry("hgt").map(|e| Height::from(e));
        let hcl = extract_entry("hcl").map(|e| HairColor(e.to_owned()));
        let ecl = extract_entry("ecl").map(|e| EyeColor(e.to_owned()));
        let pid = extract_entry("pid").map(|e| PID(e.to_owned()));

        Passport {
            byr,
            iyr,
            eyr,
            hgt,
            hcl,
            ecl,
            pid,
        }
    }
}

fn main() -> eyre::Result<()> {
    let passports: Vec<Passport> = aoc_2020::input_paragraphs("04")?
        .into_iter()
        .map(Passport::from)
        .collect();

    let part_one = passports.iter().filter(|p| p.is_valid_1()).count();

    println!("part one\n{}", part_one);

    let part_two = passports.iter().filter(|p| p.is_valid_2()).count();

    println!("part two\n{}", part_two);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn pp_valid() {
        let passport = Passport {
            byr: Some(1937),
            iyr: Some(2017),
            eyr: Some(2020),
            hgt: Some(Height::Cm(183)),
            hcl: Some(HairColor(String::from("#fffffd"))),
            ecl: Some(EyeColor(String::from("gry"))),
            pid: Some(PID(String::from("860033327"))),
        };

        assert!(passport.is_valid_1());
    }

    #[test]
    fn from_valid_string() {
        let s = String::from(
            "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd\nbyr:1937 iyr:2017 cid:147 hgt:183cm",
        );
        let expected = Passport {
            byr: Some(1937),
            iyr: Some(2017),
            eyr: Some(2020),
            hgt: Some(Height::Cm(183)),
            hcl: Some(HairColor(String::from("#fffffd"))),
            ecl: Some(EyeColor(String::from("gry"))),
            pid: Some(PID(String::from("860033327"))),
        };
        let actual = Passport::from(s);
        assert_eq!(expected, actual);
    }

    #[test]
    fn missing_hgt() {
        let passport = Passport {
            byr: Some(1937),
            iyr: Some(2013),
            eyr: Some(2023),
            hgt: None,
            hcl: Some(HairColor(String::from("#cfa07d"))),
            ecl: Some(EyeColor(String::from("amb"))),
            pid: Some(PID(String::from("028048884"))),
        };

        assert!(!passport.is_valid_1());
    }

    #[test]
    fn from_missing_hgt() {
        let s =
            String::from("iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884\nhcl:#cfa07d byr:1929");
        let expected = Passport {
            byr: Some(1929),
            iyr: Some(2013),
            eyr: Some(2023),
            hgt: None,
            hcl: Some(HairColor(String::from("#cfa07d"))),
            ecl: Some(EyeColor(String::from("amb"))),
            pid: Some(PID(String::from("028048884"))),
        };
        let actual = Passport::from(s);

        assert_eq!(expected, actual);
    }

    #[test]
    fn missing_cid() {
        let passport = Passport {
            byr: Some(1931),
            iyr: Some(2013),
            eyr: Some(2024),
            hgt: Some(Height::Cm(179)),
            hcl: Some(HairColor(String::from("#ae17e1"))),
            ecl: Some(EyeColor(String::from("brn"))),
            pid: Some(PID(String::from("760753108"))),
        };

        assert!(passport.is_valid_1());
    }

    #[test]
    fn from_missing_cid() {
        let s = String::from(
            "hcl:#ae17e1 iyr:2013\neyr:2024\necl:brn pid:760753108 byr:1931\nhgt:179cm",
        );
        let expected = Passport {
            byr: Some(1931),
            iyr: Some(2013),
            eyr: Some(2024),
            hgt: Some(Height::Cm(179)),
            hcl: Some(HairColor(String::from("#ae17e1"))),
            ecl: Some(EyeColor(String::from("brn"))),
            pid: Some(PID(String::from("760753108"))),
        };
        let actual = Passport::from(s);
        assert_eq!(expected, actual);
    }

    #[test]
    fn missing_byr() {
        let passport = Passport {
            byr: None,
            iyr: Some(2011),
            eyr: Some(2025),
            hgt: Some(Height::In(59)),
            hcl: Some(HairColor(String::from("#cfa07d"))),
            ecl: Some(EyeColor(String::from("brn"))),
            pid: Some(PID(String::from("166559648"))),
        };

        assert!(!passport.is_valid_1());
    }

    #[test]
    fn from_missing_byr() {
        let s = String::from("hcl:#cfa07d eyr:2025 pid:166559648\niyr:2011 ecl:brn hgt:59in");
        let expected = Passport {
            byr: None,
            iyr: Some(2011),
            eyr: Some(2025),
            hgt: Some(Height::In(59)),
            hcl: Some(HairColor(String::from("#cfa07d"))),
            ecl: Some(EyeColor(String::from("brn"))),
            pid: Some(PID(String::from("166559648"))),
        };
        let actual = Passport::from(s);
        assert_eq!(expected, actual);
    }

    #[test]
    fn from_string() {
        let s = String::from(
            "eyr:2029 byr:1931 hcl:z cid:128\necl:amb hgt:150cm iyr:2015 pid:148714704",
        );
        let passport = Passport::from(s);

        let expected = Passport {
            byr: Some(1931),
            iyr: Some(2015),
            eyr: Some(2029),
            hgt: Some(Height::Cm(150)),
            hcl: Some(HairColor(String::from("z"))),
            ecl: Some(EyeColor(String::from("amb"))),
            pid: Some(PID(String::from("148714704"))),
        };

        assert_eq!(expected, passport);
    }
}
