use std::{
    panic, error::Error,
};

use advent::{
    model::{elf::Elf, food::Food, food_bag::FoodBag},
    utils::{
        file_reader::FileReader,
        string_utils::{
            group_string_vector_by_empty_line, parse_string_vec_to_u32_vec,
            split_string_by_breakline,
        },
    },
};

fn parse_day_1_input() -> Result<Vec<Elf>, Box<dyn Error>> {
    let file_reader: FileReader = FileReader::new();

    let content = match file_reader.read_file(&"resources/day_1.txt") {
        Ok(data) => data,
        Err(_) => panic!("Error reading file"),
    };
    let splitted_content = split_string_by_breakline(&content);

    let group_by_whiteline = group_string_vector_by_empty_line(splitted_content);

    let mut parsed_group_by_whiteline: Vec<Vec<u32>> = vec![];
    for group in group_by_whiteline {
        parsed_group_by_whiteline.push(parse_string_vec_to_u32_vec(group)?);
    }

    let mut elfs: Vec<Elf> = vec![];
    for parsed_group in parsed_group_by_whiteline {
        let mut food_vec: Vec<Food> = vec![];
        for calories in parsed_group {
            food_vec.push(Food::new(calories));
        }
        elfs.push(Elf::new(FoodBag::new(food_vec)));
    }

    Ok(elfs)
}

#[test]
fn test_day_1_part_1() {
    let elfs = match parse_day_1_input() {
        Ok(it) => it,
        Err(_) => panic!("Error parsing day 1 input"),
    };
    let mut most_calories: u32 = 0;
    for elf in elfs.iter() {
        if elf.total_calories() > most_calories {
            most_calories = elf.total_calories();
        }
    }

    assert_eq!(most_calories, 69836);
}

#[test]
fn test_day_1_part_2() {
    let mut elfs = match parse_day_1_input() {
        Ok(it) => it,
        Err(_) => panic!("Error parsing day 1 input"),
    };

    elfs.sort_by(|a, b| a.compare_calories(b));
    let mut top_three_calories: u32 = 0;
    let (_, top_three_elfs_with_most_calories) = elfs.split_at(elfs.len() - 3);
    assert_eq!(top_three_elfs_with_most_calories.len(), 3);
    for elf in top_three_elfs_with_most_calories.iter() {
        top_three_calories += elf.total_calories();
    }
    assert_eq!(top_three_calories, 69836);
}
