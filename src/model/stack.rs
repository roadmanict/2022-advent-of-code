use std::str::FromStr;

#[derive(Debug)]
struct Stack {
    crates: Vec<char>,
}

impl FromStr for Stack {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}

#[derive(Debug)]
struct Supplies {
    pub stacks: Vec<Stack>,
}

impl FromStr for Supplies {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let test = s.lines().map(|line| {
            let chunks = line.len() / 4;
            for chunk in 0..chunks {
                let result = line.split_at(chunk * 4);
            }
        }).collect::<Vec<_>>();

        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_str() {
        let supplies = Supplies::from_str(
            "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 ",
        ).unwrap();

        assert_eq!(supplies.stacks.len(), 3)
    }
}
