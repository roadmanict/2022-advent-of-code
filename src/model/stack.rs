use std::str::FromStr;

#[derive(Debug)]
struct Crate(char);

impl FromStr for Crate {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        let mut iterator = s.chars();
        iterator.next();

        Ok(Crate(iterator.next().ok_or("Invalid crate")?))
    }
}

#[derive(Debug)]
struct Stack {
    crates: Vec<Crate>,
}

impl Stack {
    fn new() -> Self {
        Self { crates: vec![] }
    }

    fn add_crate(&mut self, crte: Crate) {
        self.crates.push(crte);
    }
}

#[derive(Debug)]
struct Supplies {
    pub stacks: Vec<Stack>,
}

impl FromStr for Supplies {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut stacks: Vec<Stack> = vec![];

        let lines = s.lines();
        for (index, line) in lines.enumerate() {
            if index == 0 {
                let len_stacks = (line.len() + 1) / 4;
                println!("len {}, stack {}", line.len(), len_stacks);
                for _ in 0..len_stacks {
                    stacks.push(Stack::new())
                }
            }

            let mut line = line;
            let mut stack_index = 0;
            while line.len() >= 4 {
                let stack = stacks.get_mut(stack_index).ok_or("Stack not available")?;
                let (raw_crate, rest) = line.split_at(4);
                let crte = Crate::from_str(raw_crate);
                if let Ok(crte) = crte {
                    stack.add_crate(crte);
                }

                line = rest;
                stack_index += 1;
            }
        }

        Ok(Supplies { stacks })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_crate_from_str() {
        assert_eq!("    ".split_at(4), ("    ", ""));
        let crte = Crate::from_str("[E] ").expect("Crate string could be parsed");

        assert_eq!(crte.0, 'E')
    }

    #[test]
    fn test_crate_from_str_invalid_input() {
        let error = Crate::from_str("    ").expect_err("from_str to return error");

        assert_eq!(error, "Invalid crate");
    }

    #[test]
    fn test_supplies_from_str() {
        let supplies = Supplies::from_str(
            "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 ",
        )
        .expect("Expect Supplies string to be parsed");
        println!("{:?}", supplies);

        assert_eq!(supplies.stacks.len(), 3)
    }
}
