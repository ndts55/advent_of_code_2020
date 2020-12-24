use eyre::eyre;
use fixedbitset::FixedBitSet;
use std::str::FromStr;

#[derive(Debug)]
struct State(usize, i32);

impl State {
    fn jmp(&self, pc: i32) -> State {
        State((self.0 as i32 + pc) as usize, self.1)
    }

    fn acc(&self, acc: i32) -> State {
        State(self.0 + 1, self.1 + acc)
    }

    fn nop(&self) -> State {
        State(self.0 + 1, self.1)
    }
}

#[derive(Debug, Clone, PartialEq)]
enum Op {
    Acc(i32),
    Jmp(i32),
    Nop(i32),
}

impl Op {
    fn execute(&self, state: &State) -> State {
        match *self {
            Op::Acc(n) => state.acc(n),
            Op::Jmp(n) => state.jmp(n),
            Op::Nop(_) => state.nop(),
        }
    }

    fn flipped(&self) -> Op {
        match *self {
            Op::Nop(n) => Op::Jmp(n),
            Op::Jmp(n) => Op::Nop(n),
            _ => self.clone(),
        }
    }
}

impl FromStr for Op {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // "acc +22"
        // "jmp -1"
        // do I actually want to regex the line?
        let s: Vec<&str> = s.split_ascii_whitespace().collect();
        if s.len() != 2 {
            return Err("invalid line");
        }
        let (op, n) = (s[0], s[1]);
        // get sign from n
        let (sign, n) = n.split_at(1);
        let n = n.parse::<i32>().map_err(|_| "error parsing number")?
            * match sign {
                "-" => -1,
                "+" => 1,
                _ => return Err("error parsing sign"),
            };

        Ok(match op {
            "acc" => Op::Acc(n),
            "jmp" => Op::Jmp(n),
            "nop" => Op::Nop(n),
            _ => return Err("error parsing op"),
        })
    }
}

#[derive(Debug, Clone)]
struct Program {
    instructions: Vec<Op>,
}

impl Program {
    fn execute(&self, state: State) -> State {
        self.instructions
            .get(state.0)
            .map(|op| op.execute(&state))
            .unwrap_or(state)
    }

    fn run(&self) -> (State, bool) {
        let mut state = State(0, 0);
        let mut execution_marker = FixedBitSet::with_capacity(self.instructions.len());

        while state.0 < self.instructions.len() && !execution_marker.put(state.0) {
            state = self.execute(state);
        }

        let success = state.0 >= self.instructions.len();
        (state, success)
    }

    fn with_flipped_at(&self, pc: usize) -> Option<Program> {
        if pc >= self.instructions.len() {
            return None;
        }

        if let Op::Acc(_) = self.instructions[pc] {
            return None;
        }

        let mut new_program = self.clone();
        new_program.instructions[pc] = new_program.instructions[pc].flipped();

        Some(new_program)
    }
}

impl FromStr for Program {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.split("\n")
            .map(Op::from_str)
            .collect::<Result<Vec<Op>, &str>>()
            .map(|ops| Program { instructions: ops })
    }
}

fn main() -> eyre::Result<()> {
    // parse input to program struct or whatever
    let program = Program::from_str(&aoc_2020::input("08")?).map_err(|e| eyre!(e))?;

    // part one
    // Your puzzle answer was 1930.
    println!("part one\n{}", program.run().0 .1);

    // part two
    // Your puzzle answer was 1688.
    let s = (0..program.instructions.len())
        .find_map(|i| {
            let (state, success) = program.with_flipped_at(i)?.run();
            if success {
                Some(state)
            } else {
                None
            }
        })
        .ok_or(eyre!("no valid program found"))?;

    println!("part two\n{}", s.1);

    Ok(())
}
