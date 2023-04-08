use std::num::ParseIntError;

pub fn split_string_by_breakline(subject: &String) -> Vec<&str> {
    subject.split("\n").collect::<Vec<&str>>()
}

pub fn group_string_vector_by_empty_line(subject: Vec<&str>) -> Vec<Vec<&str>> {
    let mut group_by_whiteline: Vec<Vec<&str>> = vec![];
    let mut temp_group: Vec<&str> = vec![];

    for line in subject {
        if line.len() > 0 {
            temp_group.push(line);

            continue;
        }

        group_by_whiteline.push(temp_group);
        temp_group = vec![]
    }
    group_by_whiteline.push(temp_group);
    group_by_whiteline
}

pub fn parse_string_to_u32(subject: &str) -> Result<u32, ParseIntError> {
    u32::from_str_radix(subject, 10)
}

pub fn parse_string_vec_to_u32_vec(subject: Vec<&str>) -> Result<Vec<u32>, ParseIntError> {
    let mut u32_vec: Vec<u32> = vec![];
    for raw_string in subject {
        let parsed_value = parse_string_to_u32(raw_string)?;
        u32_vec.push(parsed_value);
    }

    Ok(u32_vec)
}
