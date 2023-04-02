use crate::model::food::Food;

pub struct FoodBag {
    items: Vec<Food>,
    total_calories: i32,
}

impl FoodBag {
    pub fn new(items: Vec<Food>) -> Self {
        let mut total_calories: u32 = 0;
        for food in items.iter() {
            total_calories = total_calories + food.calories;
        }

        Self {
            items,
            total_calories,
        }
    }

    pub fn total_calories(&self) -> u16 {
        let mut total: u16 = 0;
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
