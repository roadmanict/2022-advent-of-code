use std::{error::Error, panic};

use advent::{
    model::{elf::Elf, food::Food, food_bag::FoodBag, play::Play, strategy::Strategy},
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

    let mut parsed_group_by_whiteline: Vec<Vec<u32>> = Vec::with_capacity(group_by_whiteline.len());
    for group in group_by_whiteline {
        parsed_group_by_whiteline.push(parse_string_vec_to_u32_vec(group)?);
    }

    let mut elfs: Vec<Elf> = Vec::with_capacity(parsed_group_by_whiteline.len());
    for parsed_group in parsed_group_by_whiteline {
        let mut food_vec: Vec<Food> = Vec::with_capacity(parsed_group.len());
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
    assert_eq!(top_three_calories, 207968);
}

#[test]
fn test_day_2_part_1() {
    let file_reader: FileReader = FileReader::new();
    let day_2_input = match file_reader.read_file(&"resources/day_2.txt") {
        Ok(data) => data,
        Err(_) => panic!("Error reading file"),
    };
    let day_2_input_split_by_breakline = split_string_by_breakline(&day_2_input);
    assert_eq!(day_2_input_split_by_breakline.len(), 2501);

    let mut strategies: Vec<Strategy> = Vec::with_capacity(day_2_input_split_by_breakline.len());
    for raw_strategy in day_2_input_split_by_breakline {
        if raw_strategy.len() == 0 {
            continue;
        }
        let (input, answer) = match raw_strategy.split_once(&" ") {
            Some(it) => it,
            None => panic!(
                "Invalid rock paper scissors strategy string {}",
                raw_strategy
            ),
        };
        strategies.push(Strategy::new(
            Play::from_opponent(input),
            Play::from_answer(answer),
        ));
    }
    assert_eq!(strategies.len(), 2500);
}
