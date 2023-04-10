use crate::utils::string_utils::find_common_chars;

pub struct Rucksack {
    pub compartment_one: String,
    pub compartment_two: String,
    pub item: RucksackItem,
}
impl Rucksack {
    pub fn new(compartment_one: String, compartment_two: String, item: RucksackItem) -> Rucksack {
        Rucksack {
            compartment_one,
            compartment_two,
            item,
        }
    }

    pub fn pack(input: &str) -> Rucksack {
        let (compartment_one, compartment_two) = input.split_at(input.len() / 2);
        let common_item = find_common_chars(compartment_one, compartment_two);

        Rucksack::new(
            compartment_one.to_owned(),
            compartment_two.to_owned(),
            RucksackItem::new(common_item[0]),
        )
    }
}

pub struct RucksackItem {
    value: char,
}
impl RucksackItem {
    pub fn new(value: char) -> RucksackItem {
        RucksackItem { value }
    }
    pub fn priority(&self) -> u32 {
        let value_as_number = self.value as u32;

        println!("{}", value_as_number);

        if value_as_number > 96 {
            return value_as_number - 96;
        } else {
            return value_as_number - 38;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rucksack_item_a() {
        let rucksack_item = RucksackItem::new('a');

        assert_eq!(rucksack_item.priority(), 1)
    }

    #[test]
    fn test_rucksack_item_z() {
        let rucksack_item = RucksackItem::new('z');

        assert_eq!(rucksack_item.priority(), 26)
    }

    #[test]
    fn test_rucksack_item_capital_a() {
        let rucksack_item = RucksackItem::new('A');

        assert_eq!(rucksack_item.priority(), 27)
    }

    #[test]
    fn test_rucksack_item_capital_z() {
        let rucksack_item = RucksackItem::new('Z');

        assert_eq!(rucksack_item.priority(), 52)
    }
}
