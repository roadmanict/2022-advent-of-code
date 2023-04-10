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

    let mut sum_group_priority: u32 = 0;
    let groups = rucksacks.len() / 3;
    assert_eq!(groups, 100);

    for n in 0..groups {
        let index = n * 3;
        println!("{}, {}", n, index);
        let mut group = vec![
            &rucksacks[index],
            &rucksacks[index + 1],
            &rucksacks[index + 2],
        ];
        group.sort_by(|a, b| a.compare_size(b));
        println!(
            "{}, {}, {}",
            group[0].contents(),
            group[1].contents(),
            group[2].contents()
        );
        println!("New Group");
        let group_0_single_items = group[0].single_items();
        let group_1_single_items = group[1].single_items();
        let group_2_single_items = group[2].single_items();
        println!(
            "{:?}, {:?}, {:?}",
            group_0_single_items, group_1_single_items, group_2_single_items
        );
        for char in group_0_single_items.iter() {
            if group_1_single_items.contains(char) && group_2_single_items.contains(char) {
                println!("char {}, priority {}", char, group[1].item.priority());
                sum_group_priority += group[1].item.priority();

                break;
            } else {
                println!("does not contain char {}", char);
            }
        }
    }

    assert_eq!(sum_group_priority, 1000);
}
