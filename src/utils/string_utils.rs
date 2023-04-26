use std::{num::ParseIntError, vec};

pub fn split_string_by_breakline(subject: &str) -> Vec<&str> {
    subject.split('\n').collect::<Vec<&str>>()
}

pub fn count_until_next_white_line(subject: &[&str], skip: Option<usize>) -> usize {
    let mut count: usize = 0;

    for line in subject.iter().skip(skip.unwrap_or(0)) {
        count += 1;

        if line.is_empty() {
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
        if !line.is_empty() {
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
    subject.parse::<u32>()
}

pub fn parse_string_vec_to_u32_vec(subject: Vec<&str>) -> Result<Vec<u32>, ParseIntError> {
    let mut u32_vec: Vec<u32> = Vec::with_capacity(subject.len());
    for raw_string in subject {
        let parsed_value = parse_string_to_u32(raw_string)?;
        u32_vec.push(parsed_value);
    }

    Ok(u32_vec)
}

pub fn count_white_lines_in_str_vec(subject: &[&str]) -> usize {
    let mut count: usize = 0;
    for line in subject.iter() {
        if line.is_empty() {
            count += 1;
        }
    }
    count
}
pub fn find_common_chars(a: &str, b: &str) -> Vec<char> {
    let mut common_chars: Vec<char> = vec![];
    for char in a.chars() {
        if b.contains(char) {
            common_chars.push(char);
        }
    }

    common_chars
}
pub fn count_char_in_string(ch: &char, subject: &str) -> usize {
    let mut amount: usize = 0;
    for subject_char in subject.chars() {
        if subject_char == *ch {
            amount += 1;
        }
    }

    amount
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_until_next_white_line() {
        let result = count_until_next_white_line(&["test", "test", "asdf", "", "jojo", ""], None);

        assert_eq!(result, 4)
    }

    #[test]
    fn test_count_until_next_white_line_second_whiteline() {
        let result =
            count_until_next_white_line(&["test", "test", "asdf", "", "jojo", ""], Some(4));

        assert_eq!(result, 2)
    }

    #[test]
    fn test_count_white_lines_in_str_vec() {
        let result = count_white_lines_in_str_vec(&["test", "", "asdf", "", "jojo", ""]);

        assert_eq!(result, 3)
    }

    #[test]
    fn test_find_common_chars() {
        let result = find_common_chars("aAbB", "cCaB");

        assert_eq!(result, vec!['a', 'B'])
    }
    #[test]
    fn test_count_chars_in_string() {
        let result = count_char_in_string(&'a', "baaabaaa");

        assert_eq!(result, 6);
    }
}
