use std::str::FromStr;

#[derive(Debug)]
struct Datastream {
    field: String,
}

impl Datastream {
    pub fn find_marker_position(&self) -> Result<usize, &str> {
        for index in 0..self.field.len() {
            if index + 3 > self.field.len() {
                return Err("Cannot find marker");
            }

            let substr = &self.field[index..=index + 3];
            let result = substr
                .chars()
                .map(|ch| substr.chars().filter(|chb| ch == *chb).count())
                .all(|count| count == 1);
            if result {
                return Ok(index + 4);
            }
        }
        todo!()
    }
}

impl FromStr for Datastream {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Datastream {
            field: s.to_owned(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_marker_position() {
        let datastream =
            Datastream::from_str("bvwbjplbgvbhsrlpgdmjqwftvncz").expect("Cannot parse datastream");

        assert_eq!(
            datastream.find_marker_position().expect("Can find marker"),
            5
        );
    }
}
