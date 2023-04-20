use std::str::FromStr;

use advent::{
    model::{
        play::{Play, PlayResultStrategy},
        strategy::Strategy,
    },
    utils::{file_reader::FileReader, string_utils::split_string_by_breakline},
};

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

        strategies.push( Strategy::from_str(raw_strategy).unwrap());
    }
    assert_eq!(strategies.len(), 2500);

    let mut score: usize = 0;
    for strategy in strategies.iter() {
        score += strategy.answer.score();
        score += strategy.answer.versus(&strategy.opponent).score();
    }

    assert_eq!(score, 14375);

    score = 0;
    for strategy in strategies.iter() {
        let hand = strategy
            .play_result_strategy
            .should_play(&strategy.opponent);
        score += hand.score();
        let result = hand.versus(&strategy.opponent);
        score += result.score();
    }

    assert_eq!(score, 10274);
}
