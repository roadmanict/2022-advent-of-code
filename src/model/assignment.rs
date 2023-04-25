use std::{fmt::Display, num::ParseIntError, ops::RangeInclusive, str::FromStr};

pub struct Assignment {
    pub range: RangeInclusive<u8>,
}
impl Assignment {
    pub fn new(start: u8, end: u8) -> Self {
        Self { range: start..=end }
    }

    pub fn fully_contains(&self, assignment: &Assignment) -> bool {
        let min = assignment.range.start();
        let max = assignment.range.end();

        self.range.contains(min) && self.range.contains(max)
    }

    pub fn partial_contains(&self, assignment: &Assignment) -> bool {
        let min = assignment.range.start();
        let max = assignment.range.end();

        self.range.contains(min) || self.range.contains(max)
    }
}

#[derive(Debug)]
pub enum ParseAssignmentError {
    InvalidArgument(String),
    ParseIntError(ParseIntError),
}
impl From<ParseIntError> for ParseAssignmentError {
    fn from(value: ParseIntError) -> Self {
        ParseAssignmentError::ParseIntError(value)
    }
}
impl Display for ParseAssignmentError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseAssignmentError::InvalidArgument(str) => write!(f, "{}", str),
            ParseAssignmentError::ParseIntError(err) => write!(f, "{}", err),
        }
    }
}

impl std::error::Error for ParseAssignmentError {}

impl FromStr for Assignment {
    type Err = ParseAssignmentError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (start, end) = s
            .split_once('-')
            .ok_or(ParseAssignmentError::InvalidArgument(format!(
                "Error parsing Assignment string: {}",
                s
            )))?;

        Ok(Self::new(u8::from_str(start)?, u8::from_str(end)?))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fully_contains_true() {
        let a = Assignment::new(2, 3);
        let b = Assignment::new(2, 3);

        assert!(a.fully_contains(&b));
    }

    #[test]
    fn test_fully_contains_start_false() {
        let a = Assignment::new(2, 3);
        let b = Assignment::new(1, 3);

        assert!(!a.fully_contains(&b));
    }

    #[test]
    fn test_fully_contains_end_false() {
        let a = Assignment::new(2, 3);
        let b = Assignment::new(2, 4);

        assert!(!a.fully_contains(&b));
    }

    #[test]
    fn test_from_str() {
        let ass = Assignment::from_str("4-8").unwrap();

        assert_eq!(ass.range, Assignment::new(4, 8).range);
    }
}
