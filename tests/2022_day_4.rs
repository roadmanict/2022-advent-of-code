use advent::{
    model::assignment::Assignment,
    utils::{file_reader::FileReader, string_utils::split_string_by_breakline},
};

#[test]
fn test_day_4() {
    let file_reader: FileReader = FileReader::new();
    let input = match file_reader.read_file(&"resources/day_4.txt") {
        Ok(data) => data,
        Err(_) => panic!("Error reading file"),
    };
    let input_split_by_breakline = split_string_by_breakline(&input);
    assert_eq!(input_split_by_breakline.len(), 1001);

    let mut pairs: Vec<(Assignment, Assignment)> =
        Vec::with_capacity(input_split_by_breakline.len() / 2);

    let mut assignments: (Assignment, Assignment);
    for raw_pair in input_split_by_breakline.iter() {
        if raw_pair.len() == 0 {
            continue;
        }
        let assignments = raw_pair
            .split(|ch| ch == '-' || ch == ',')
            .collect::<Vec<&str>>()
            .iter()
            .map(|ch| -> u8 {
                match u8::from_str_radix(ch, 10) {
                    Ok(parsed) => return parsed,
                    Err(_) => panic!("Invalid raw_pair: {}", raw_pair),
                }
            })
            .collect::<Vec<u8>>();
        assert_eq!(assignments.len(), 4);
    }
}
