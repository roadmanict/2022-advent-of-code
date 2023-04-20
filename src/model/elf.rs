use std::{cmp::Ordering, str::FromStr};

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

    pub fn compare_calories(&self, compare: &Elf) -> Ordering {
        if self.total_calories() > compare.total_calories() {
            return Ordering::Greater;
        } else if self.total_calories() < compare.total_calories() {
            return Ordering::Less;
        }

        Ordering::Equal
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
