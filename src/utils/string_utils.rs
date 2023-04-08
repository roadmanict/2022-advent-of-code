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
