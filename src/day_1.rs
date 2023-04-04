use std::error::Error;

use crate::{
    model::{elf::Elf, food::Food, food_bag::FoodBag},
    utils::file_reader::FileReader,
};

struct Day1Excercise {
    file_reader: FileReader,
}

impl Day1Excercise {
    fn new(file_reader: FileReader) -> Day1Excercise {
        Day1Excercise { file_reader }
    }

    fn find_elf_carrying_most_calories(&self) -> Result<u32, Box<dyn Error>> {
        let content = self.file_reader.read_file(&"resources/day_1.txt")?;
        let content_split_by_breakline = content.split("\n");
        let content_vector = content_split_by_breakline.collect::<Vec<&str>>();

        let mut group_by_whiteline: Vec<Vec<&str>> = vec![];
        let mut temp_group: Vec<&str> = vec![];

        for line in content_vector {
            if line.len() > 0 {
                temp_group.push(line);

                continue;
            }

            group_by_whiteline.push(temp_group);
            temp_group = vec![]
        }
        group_by_whiteline.push(temp_group);
        println!("group_by_whiteline: {}", group_by_whiteline.len());
        let mut parsed_group_by_whiteline: Vec<Vec<u32>> = vec![];
        for group in group_by_whiteline {
            let mut temp_group: Vec<u32> = vec![];
            for raw_string in group {
                let parsed_value = u32::from_str_radix(raw_string, 10)?;
                temp_group.push(parsed_value);
            }
            parsed_group_by_whiteline.push(temp_group);
        }
        println!("{:?}", parsed_group_by_whiteline);
        let mut elfs: Vec<Elf> = vec![];
        for parsed_group in parsed_group_by_whiteline {
            let mut food_vec: Vec<Food> = vec![];
            for calories in parsed_group {
                food_vec.push(Food::new(calories));
            }
            elfs.push(Elf::new(FoodBag::new(food_vec)));
        }
        let mut most_calories: u32 = 0;
        for elf in elfs.iter() {
            if elf.total_calories() > most_calories {
                most_calories = elf.total_calories();
            }
        }

        println!("{:?}", elfs.len());
        Ok(most_calories)
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::file_reader::FileReader;

    use super::*;

    #[test]
    fn test_parsing_multiline_string() {
        let file_reader = FileReader::nullable(
            "9057
8878
2753
7027

2331
3785
1505
3355
3353
4416",
        );

        let day_1_exercise = Day1Excercise::new(file_reader);

        let result = day_1_exercise.find_elf_carrying_most_calories();

        assert!(result.is_ok(), "{:?}", result.unwrap_err());
        assert_eq!(result.unwrap(), 27715);
    }

    #[test]
    fn day_1_excercise() {
        let file_reader = FileReader::new();
        let day_1_exercise = Day1Excercise::new(file_reader);

        let result = day_1_exercise.find_elf_carrying_most_calories();

        assert!(result.is_ok(), "{:?}", result.unwrap_err());
        assert_eq!(result.unwrap(), 69836);
    }
}
