use std::{num::ParseIntError, str::FromStr};

#[derive(Debug)]
pub struct MoveCrate {
    amount: usize,
    from: usize,
    to: usize,
}

impl MoveCrate {
    fn new(amount: usize, from: usize, to: usize) -> Self {
        Self { amount, from, to }
    }
}

#[derive(Debug)]
pub enum MoveCrateFromStrError {
    Generic(String),
    ParseIntError(ParseIntError),
}

impl From<String> for MoveCrateFromStrError {
    fn from(value: String) -> Self {
        MoveCrateFromStrError::Generic(value)
    }
}
impl From<ParseIntError> for MoveCrateFromStrError {
    fn from(value: ParseIntError) -> Self {
        MoveCrateFromStrError::ParseIntError(value)
    }
}

impl FromStr for MoveCrate {
    type Err = MoveCrateFromStrError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split_string = s.split_whitespace().collect::<Vec<_>>();
        let amount = split_string[1].parse::<usize>()?;
        let from = split_string[3].parse::<usize>()?;
        let to = split_string[5].parse::<usize>()?;

        Ok(MoveCrate { amount, from, to })
    }
}

#[derive(Debug)]
struct Crate(char);

#[derive(Debug)]
pub struct Stack {
    crates: Vec<Crate>,
}

impl Stack {
    fn new() -> Self {
        Self { crates: vec![] }
    }

    fn add_crate(&mut self, crte: Crate) {
        self.crates.push(crte);
    }

    fn remove_crates(&mut self, amount: usize) -> Vec<Crate> {
        let range = self.crates.len() - amount..self.crates.len();

        self.crates.drain(range).rev().collect::<Vec<_>>()
    }
}

#[derive(Debug)]
pub struct Supplies {
    pub stacks: Vec<Stack>,
}

impl Supplies {
    pub fn move_crate(&mut self, move_crate: MoveCrate) {
        let crates_to_move = self.stacks[move_crate.from - 1].remove_crates(move_crate.amount);

        for ele in crates_to_move {
            self.stacks[move_crate.to - 1].add_crate(ele);
        }
    }

    pub fn get_top_crates(&self) -> String {
        self.stacks.iter().map(|stack| stack.crates[stack.crates.len() - 1].0.to_string()).collect::<Vec<_>>().concat::<_>()
    }
}

impl FromStr for Supplies {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut stacks: Vec<Stack> = vec![];

        let lines = s.lines();
        let mut crate_indexes: Vec<usize> = vec![];
        for (index, line) in lines.rev().enumerate() {
            if index == 0 {
                for (index, ch) in line.chars().enumerate() {
                    if ch.is_whitespace() {
                        continue;
                    }

                    crate_indexes.push(index);
                    stacks.push(Stack::new());
                }

                continue;
            }

            let chars = line.chars().collect::<Vec<_>>();

            for (index, crate_index) in crate_indexes.iter().enumerate() {
                let ch = chars[*crate_index];
                if ch.is_whitespace() {
                    continue;
                }

                stacks[index].add_crate(Crate(ch));
            }
        }

        Ok(Supplies { stacks })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_supplies_from_str() {
        let supplies = Supplies::from_str(
            "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 ",
        )
        .expect("Expect Supplies string to be parsed");

        assert_eq!(supplies.stacks.len(), 3);
        assert_eq!(supplies.stacks[0].crates[0].0, 'Z');
        assert_eq!(supplies.stacks[0].crates[1].0, 'N');
        assert_eq!(supplies.stacks[1].crates[0].0, 'M');
        assert_eq!(supplies.stacks[1].crates[1].0, 'C');
        assert_eq!(supplies.stacks[1].crates[2].0, 'D');
        assert_eq!(supplies.stacks[2].crates[0].0, 'P');
    }

    fn test_move_crate_from_str() {
        let move_crate = MoveCrate::from_str("move 1 from 2 to 1").unwrap();

        assert_eq!(move_crate.amount, 1);
        assert_eq!(move_crate.from, 2);
        assert_eq!(move_crate.to, 2);
    }

    fn test_remove_from_crate() {
        let mut stack = Stack::new();
        stack.add_crate(Crate('S'));
        stack.add_crate(Crate('A'));
        stack.add_crate(Crate('D'));
        stack.add_crate(Crate('F'));

        let removed_crates = stack.remove_crates(2);

        assert_eq!(removed_crates.len(), 2);
        assert_eq!(removed_crates[0].0, Crate('F').0);
        assert_eq!(removed_crates[1].0, Crate('D').0);

        assert_eq!(stack.crates.len(), 2);
        assert_eq!(stack.crates[0].0, Crate('S').0);
        assert_eq!(stack.crates[1].0, Crate('A').0);
    }
}
