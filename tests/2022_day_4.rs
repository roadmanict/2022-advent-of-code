use std::str::FromStr;

use advent::{
    model::assignment::Assignment,
    utils::{file_reader::FileReader, string_utils::split_string_by_breakline},
};

#[test]
fn test_day_4() {
    let file_reader: FileReader = FileReader::new();
    let input = file_reader.read_file(&"resources/day_4.txt").unwrap();
    let input_split_by_breakline = split_string_by_breakline(&input);
    assert_eq!(input_split_by_breakline.len(), 1001);

    let mut fully_contains_count = 0;
    let mut partial_contains_count = 0;
    for raw_pair in input_split_by_breakline.iter() {
        if raw_pair.len() == 0 {
            continue;
        }
        let (left, right) = raw_pair.split_once(',').unwrap();
        let left_assignment = Assignment::from_str(left).unwrap();
        let right_assignment = Assignment::from_str(right).unwrap();
        if left_assignment.fully_contains(&right_assignment)
            || right_assignment.fully_contains(&left_assignment)
        {
            fully_contains_count += 1;
        }
        if left_assignment.partial_contains(&right_assignment)
            || right_assignment.partial_contains(&left_assignment)
        {
            partial_contains_count += 1;
        }
    }
    assert_eq!(fully_contains_count, 644);
    assert_eq!(partial_contains_count, 926);
}
