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

fn batch_string(s: &str, size: usize) -> &[&str] {
    let length = s.len();
    let mut start = 0;
    while start < length {
        s.split_at(size);
    }
    todo!()
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
        let mut crate_indexes: Vec<usize> = vec![];
        for (index, line) in lines.rev().enumerate() {
            if index == 0 {
                for (index, ch) in line.chars().enumerate() {
                    if ch.is_whitespace() {
                        continue;
                    }

                    crate_indexes.push(index);
                    stacks.push(Stack::new());

                    println!("index {}, part {}", index, ch);
                }

                continue;
            }

            println!("indexes: {:?}", crate_indexes);
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

        assert_eq!(supplies.stacks.len(), 3);
        assert_eq!(supplies.stacks[0].crates[0].0, 'Z');
        assert_eq!(supplies.stacks[0].crates[1].0, 'N');
        assert_eq!(supplies.stacks[1].crates[0].0, 'M');
        assert_eq!(supplies.stacks[1].crates[1].0, 'C');
        assert_eq!(supplies.stacks[1].crates[2].0, 'D');
        assert_eq!(supplies.stacks[2].crates[0].0, 'P');
    }
}
