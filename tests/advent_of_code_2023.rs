use advent::{day_1::Day1Exercise, utils::{file_reader::FileReader, string_utils::split_string_by_breakline}};

#[test]
fn test_day_1_part_1() {
    let file_reader: FileReader = FileReader::new();
    let day_1_exercise: Day1Exercise = Day1Exercise::new(file_reader);

        let content = match  file_reader.read_file(&"resources/day_1.txt"){
            Ok(data) => data,
            Err(_) => panic!("Error reading file"),
        };
        let splitted_content = split_string_by_breakline(&content);

        let mut group_by_whiteline: Vec<Vec<&str>> = vec![];
        let mut temp_group: Vec<&str> = vec![];

        for line in splitted_content {
            if line.len() > 0 {
                temp_group.push(line);

                continue;
            }

            group_by_whiteline.push(temp_group);
            temp_group = vec![]
        }
        group_by_whiteline.push(temp_group);

        let mut parsed_group_by_whiteline: Vec<Vec<u32>> = vec![];
        for group in group_by_whiteline {
            let mut temp_group: Vec<u32> = vec![];
            for raw_string in group {
                let parsed_value = u32::from_str_radix(raw_string, 10)?;
                temp_group.push(parsed_value);
            }
            parsed_group_by_whiteline.push(temp_group);
        }

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

        Ok(most_calories)

    assert!(result.is_ok(), "{:?}", result.unwrap_err());
    assert_eq!(result.unwrap(), 69836);
}
