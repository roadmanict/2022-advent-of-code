use std::cmp::Ordering;

use crate::utils::string_utils::{count_char_in_string, find_common_chars};

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
    pub fn compare_size(&self, other: &Rucksack) -> Ordering {
        if self.size() > other.size() {
            return Ordering::Greater;
        } else if self.size() < other.size() {
            return Ordering::Less;
        }
        Ordering::Equal
    }
    pub fn size(&self) -> usize {
        self.compartment_one.len() + self.compartment_two.len()
    }
    pub fn contents(&self) -> String {
        let mut contents =
            String::with_capacity(self.compartment_one.len() + self.compartment_two.len());
        contents.push_str(&self.compartment_one);
        contents.push_str(&self.compartment_two);

        contents
    }
    pub fn contains_item(&self, item: char) -> bool {
        self.compartment_one.contains(item) || self.compartment_two.contains(item)
    }
    pub fn single_items(&self) -> Vec<char> {
        let mut single_items: Vec<char> = vec![];
        let contents = self.contents();
        let mut char_count: usize;
        for char in contents.chars() {
            char_count = count_char_in_string(&char, &contents);
            if char_count == 1 {
                single_items.push(char);
            }
        }

        single_items
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

        if value_as_number > 96 {
            value_as_number - 96
        } else {
            value_as_number - 38
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
