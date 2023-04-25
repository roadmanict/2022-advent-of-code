use std::{cmp::Ordering};

use super::food_bag::FoodBag;

pub struct Elf {
    pub food_bag: FoodBag,
}

impl Elf {
    pub fn new(food_bag: FoodBag) -> Self {
        Self { food_bag }
    }

    pub fn total_calories(&self) -> u32 {
        self.food_bag.total_calories
    }

    pub fn compare_calories(&self, other: &Elf) -> Ordering {
        self.total_calories().cmp(&other.total_calories())
    }
}

#[cfg(test)]
mod tests {
    use crate::model::{food::Food, food_bag::FoodBag};

    use super::*;

    #[test]
    fn test_total_calories() {
        let elf = Elf::new(FoodBag::new(vec![Food::new(1000)]));

        assert_eq!(elf.total_calories(), 1000);
    }
}
