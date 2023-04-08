use advent::{day_1::Day1Exercise, utils::file_reader::FileReader};

#[test]
fn advent_of_code_2023() {
    let file_reader: FileReader = FileReader::new();
    let day_1_exercise: Day1Exercise = Day1Exercise::new(file_reader);

    test_day_1_part_1(day_1_exercise)
}

fn test_day_1_part_1(day_1_exercise: Day1Exercise) {
    let result = day_1_exercise.find_elf_carrying_most_calories();

    assert!(result.is_ok(), "{:?}", result.unwrap_err());
    assert_eq!(result.unwrap(), 69836);
}
