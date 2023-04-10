use advent::{
    model::rucksack::Rucksack,
    utils::{file_reader::FileReader, string_utils::split_string_by_breakline},
};

#[test]
fn test_day_3_part_1() {
    let file_reader: FileReader = FileReader::new();
    let input = match file_reader.read_file(&"resources/day_3.txt") {
        Ok(data) => data,
        Err(_) => panic!("Error reading file"),
    };
    let input_split_by_breakline = split_string_by_breakline(&input);
    assert_eq!(input_split_by_breakline.len(), 301);

    let mut rucksacks: Vec<Rucksack> = Vec::with_capacity(input_split_by_breakline.len());
    for input_line in input_split_by_breakline.iter() {
        if input_line.len() == 0 {
            continue;
        }
        rucksacks.push(Rucksack::pack(input_line));
    }

    let mut sum_priority: u32 = 0;
    for rucksack in rucksacks.iter() {
        sum_priority += rucksack.item.priority();
    }
    assert_eq!(sum_priority, 7737);
}
