use advent::{
    model::rucksack::{Rucksack, RucksackItem},
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

    let mut sum_group_priority: u32 = 0;
    let groups = rucksacks.len() / 3;
    assert_eq!(groups, 100);

    let mut changed: bool;
    let mut group_0: String;
    let mut group_1: String;
    let mut group_2: String;
    let mut priority: u32;

    for group in rucksacks.chunks(3) {
        assert_eq!(group.len(), 3);
        changed = false;

        group_0 = group[0].contents();
        group_1 = group[1].contents();
        group_2 = group[2].contents();

        for char in group_0.chars() {
            if group_1.contains(char) && group_2.contains(char) {
                priority = RucksackItem::new(char).priority();
                sum_group_priority += priority;

                changed = true;
                break;
            }
        }
        if !changed {
            panic!("Did not find a common item type");
        }
    }

    assert_eq!(sum_group_priority, 2697);
}
