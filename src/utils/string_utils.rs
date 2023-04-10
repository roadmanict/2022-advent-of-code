use std::num::ParseIntError;

pub fn split_string_by_breakline(subject: &String) -> Vec<&str> {
    subject.split("\n").collect::<Vec<&str>>()
}

pub fn count_until_next_white_line(subject: &Vec<&str>, skip: Option<usize>) -> usize {
    let mut count: usize = 0;

    for line in subject.iter().skip(skip.unwrap_or(0)) {
        count += 1;

        if line.len() == 0 {
            break;
        }
    }

    count
}

pub fn group_string_vector_by_empty_line(subject: Vec<&str>) -> Vec<Vec<&str>> {
    let group_capacity = count_white_lines_in_str_vec(&subject);
    let mut group_by_whiteline: Vec<Vec<&str>> = Vec::with_capacity(group_capacity);

    let mut temp_group_capacity = count_until_next_white_line(&subject, None);
    let mut temp_group: Vec<&str> = Vec::with_capacity(temp_group_capacity);

    for (index, line) in subject.iter().enumerate() {
        if line.len() > 0 {
            temp_group.push(line);

            continue;
        }

        group_by_whiteline.push(temp_group);
        temp_group_capacity = count_until_next_white_line(&subject, Some(index));
        temp_group = Vec::with_capacity(temp_group_capacity);
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

pub fn count_white_lines_in_str_vec(subject: &Vec<&str>) -> usize {
    let mut count: usize = 0;
    for line in subject.iter() {
        if line.len() == 0 {
            count += 1;
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_until_next_white_line() {
        let result =
            count_until_next_white_line(&vec!["test", "test", "asdf", "", "jojo", ""], None);

        assert_eq!(result, 4)
    }

    #[test]
    fn test_count_until_next_white_line_second_whiteline() {
        let result =
            count_until_next_white_line(&vec!["test", "test", "asdf", "", "jojo", ""], Some(4));

        assert_eq!(result, 2)
    }

    #[test]
    fn test_count_white_lines_in_str_vec() {
        let result = count_white_lines_in_str_vec(&vec!["test", "", "asdf", "", "jojo", ""]);

        assert_eq!(result, 3)
    }
}
