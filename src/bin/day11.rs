use eyre::eyre;
use fixedbitset::FixedBitSet;
use itertools::Itertools;
use std::{
    cmp::{max, min},
    fmt,
    str::FromStr,
};

// idea: use two fixed bit sets
// one for seats empty or not empty
// another for seat or floor

#[derive(Debug, Clone)]
struct Deck {
    is_seat: FixedBitSet,
    is_occupied: FixedBitSet,
    height: usize,
    width: usize,
}

impl Deck {
    fn next_generation(&self) -> Self {
        let mut is_occupied = self.is_occupied.clone();

        for index in 0..is_occupied.len() {
            if !self.is_seat[index] {
                continue;
            }
            let occupation_count = self.occupation_count(index);

            // if seat empty and no occupied around -> occupied
            if !self.is_occupied[index] && occupation_count == 0 {
                is_occupied.put(index);
            }
            // if seat occupied and >= 4 occupied around it -> empty
            if self.is_occupied[index] && occupation_count >= 4 {
                is_occupied.set(index, false);
            }
        }

        Deck {
            is_seat: self.is_seat.clone(),
            is_occupied,
            height: self.height,
            width: self.width,
        }
    }

    fn surrounding_indices(&self, index: usize) -> Vec<usize> {
        let row = index / self.width;
        let col = index % self.height;
        [-1, 0, 1]
            .iter()
            .cartesian_product([-1, 0, 1].iter())
            .map(|(x, y)| {
                let r = max(min(row as i32 + x, self.height as i32 - 1), 0) as usize;
                let c = max(min(col as i32 + y, self.width as i32 - 1), 0) as usize;
                r * self.width + c
            })
            .filter(|&idx| idx != index)
            .unique()
            .collect_vec()
    }

    fn occupation_count(&self, index: usize) -> usize {
        self.surrounding_indices(index)
            .into_iter()
            .filter(|&idx| self.is_occupied[idx])
            .count()
    }
}

impl FromStr for Deck {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // split lines
        // create two FixedBitSets with appropriate size
        // for each line do:
        // L / # -> seats = 1, occupied = 1
        let lines = s.split("\n").collect_vec();
        let height = lines.len();
        let width = lines.get(0).ok_or("empty line")?.len();
        let mut is_seat = FixedBitSet::with_capacity(height * width);
        let mut is_occupied = is_seat.clone();
        for (row, line) in lines.into_iter().enumerate() {
            for (column, c) in line.char_indices() {
                let index = row * width + column;
                if c == 'L' {
                    is_seat.put(index);
                    is_occupied.put(index);
                }
            }
        }

        Ok(Deck {
            is_seat,
            is_occupied,
            height,
            width,
        })
    }
}

impl fmt::Display for Deck {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = (0..self.is_occupied.len())
            .into_iter()
            .map(|idx| {
                format!(
                    "{}{}",
                    match (self.is_seat[idx], self.is_occupied[idx]) {
                        (false, _) => ".",
                        (true, false) => "L",
                        (true, true) => "#",
                    },
                    if (idx + 1) % self.width == 0 {
                        "\n"
                    } else {
                        ""
                    }
                )
            })
            .collect::<String>();

        write!(f, "{}", s)
    }
}

fn main() -> eyre::Result<()> {
    let deck = Deck::from_str(&aoc_2020::input("11")?).map_err(|e| eyre!(e))?;
    println!("{}", deck);
    println!("{}, {}", deck.height, deck.width);
    // part one
    // println!("part one\n{}", part_one(&deck));

    // part two

    Ok(())
}

fn part_one(deck: &Deck) -> usize {
    let mut deck = deck.clone();
    let mut next_deck = deck.next_generation();
    while are_different(&deck, &next_deck) {
        let tmp = next_deck.next_generation();
        deck = next_deck;
        next_deck = tmp;
    }

    deck.is_occupied.count_ones(..)
}

fn are_different(d1: &Deck, d2: &Deck) -> bool {
    d1.is_occupied.difference(&d2.is_occupied).next().is_some()
        || d2.is_occupied.difference(&d1.is_occupied).next().is_some()
}

#[test]
fn deck_test() {
    let test_data = "L.LL.LL.LL\nLLLLLLL.LL\nL.L.L..L..\nLLLL.LL.LL\nL.LL.LL.LL\nL.LLLLL.LL\n..L.L.....\nLLLLLLLLLL\nL.LLLLLL.L\nL.LLLLL.LL";
    let deck0 = Deck::from_str(test_data).unwrap();
    println!("deck0\n{}", deck0);
    let deck1 = deck0.next_generation();
    println!("deck1\n{}", deck1);
    let deck2 = deck1.next_generation();
    println!("deck2\n{}", deck2);
    let deck3 = deck2.next_generation();
    println!("deck3\n{}", deck3);
    let deck4 = deck3.next_generation();
    println!("deck4\n{}", deck4);
    let deck5 = deck4.next_generation();
    println!("deck5\n{}", deck5);

    assert!(are_different(&deck0, &deck1), "0 and 1");
    assert!(are_different(&deck1, &deck2), "1 and 2");
    assert!(are_different(&deck2, &deck3), "2 and 3");
    assert!(are_different(&deck3, &deck4), "3 and 4");
    assert!(!are_different(&deck4, &deck5), "not 4 and 5");

    let expected =  [String::from("#.##.##.##\n#######.##\n#.#.#..#..\n####.##.##\n#.##.##.##\n#.#####.##\n..#.#.....\n##########\n#.######.#\n#.#####.##\n"),
    String::from("#.LL.L#.##\n#LLLLLL.L#\nL.L.L..L..\n#LLL.LL.L#\n#.LL.LL.LL\n#.LLLL#.##\n..L.L.....\n#LLLLLLLL#\n#.LLLLLL.L\n#.#LLLL.##\n"),
    String::from("#.##.L#.##\n#L###LL.L#\nL.#.#..#..\n#L##.##.L#\n#.##.LL.LL\n#.###L#.##\n..#.#.....\n#L######L#\n#.LL###L.L\n#.#L###.##\n"),
    String::from("#.#L.L#.##\n#LLL#LL.L#\nL.L.L..#..\n#LLL.##.L#\n#.LL.LL.LL\n#.LL#L#.##\n..L.L.....\n#L#LLLL#L#\n#.LLLLLL.L\n#.#L#L#.##\n"),
    String::from("#.#L.L#.##\n#LLL#LL.L#\nL.#.L..#..\n#L##.##.L#\n#.#L.LL.LL\n#.#L#L#.##\n..L.L.....\n#L#L##L#L#\n#.LLLLLL.L\n#.#L#L#.##\n")
    ];
    assert_eq!(expected[0], format!("{}", deck0));
    assert_eq!(expected[1], format!("{}", deck1));
    assert_eq!(expected[2], format!("{}", deck2));
    assert_eq!(expected[3], format!("{}", deck3));
    assert_eq!(expected[4], format!("{}", deck4));
    assert_eq!(expected[4], format!("{}", deck5));
    assert_eq!(expected[4], format!("{}", deck5.next_generation()));

    assert_eq!(37, deck5.is_occupied.count_ones(..));
}
