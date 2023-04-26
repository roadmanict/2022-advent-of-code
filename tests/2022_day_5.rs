use std::str::FromStr;

use advent::{
    model::stack::{MoveCrate, Supplies},
    utils::file_reader::FileReader,
};

#[test]
fn test_day_5() {
    let file_reader: FileReader = FileReader::new();
    let input = file_reader.read_file("resources/day_5.txt").unwrap();

    let (supplies, moves) = input
        .split_once("\n\n")
        .expect("Cannot split by double newline");
    let mut supplies = Supplies::from_str(supplies).expect("Cannot parse supplies");

    let moves = moves
        .lines()
        .map(|line| MoveCrate::from_str(line).expect("Cannot parse movecrate"));

    for move_crate in moves {
        supplies.move_crate(move_crate);
    }

    assert_eq!(supplies.get_top_crates(), "");
}
