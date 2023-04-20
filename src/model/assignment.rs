use std::ops::RangeInclusive;

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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fully_contains_true() {
        let a = Assignment::new(2, 3);
        let b = Assignment::new(2, 3);

        assert_eq!(a.fully_contains(&b), true);
    }

    #[test]
    fn test_fully_contains_start_false() {
        let a = Assignment::new(2, 3);
        let b = Assignment::new(1, 3);

        assert_eq!(a.fully_contains(&b), false);
    }

    #[test]
    fn test_fully_contains_end_false() {
        let a = Assignment::new(2, 3);
        let b = Assignment::new(2, 4);

        assert_eq!(a.fully_contains(&b), false);
    }
}
