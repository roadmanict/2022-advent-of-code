use crate::model::food::Food;

pub struct FoodBag {
    pub items: Vec<Food>,
}

impl FoodBag {
    pub fn new(items: Vec<Food>) -> Self {
        Self { items }
    }

    pub fn total_calories(&self) -> u32 {
        let mut total: u32 = 0;
        for food in self.items.iter() {
            total = total + food.calories;
        }

        total
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total_calories_single_item() {
        let food_bag = FoodBag::new(vec![Food::new(1000)]);

        assert_eq!(food_bag.total_calories(), 1000)
    }

    #[test]
    fn test_total_calories_multiple_item() {
        let food_bag = FoodBag::new(vec![Food::new(1000), Food::new(1000), Food::new(1000)]);

        assert_eq!(food_bag.total_calories(), 3000)
    }
}
