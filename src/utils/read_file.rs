use std::error::Error;
use std::{fs::File, path::Path};

pub fn read_file(path: String) -> Result<String, > {
    let data_file_path = Path::new(&path);
    let mut data_file = File::open(&data_file_path);
    let mut data_file = match File::open(data_file_path) {
        Err(e) => return Err(e),
        Ok(file) => file,
    };
    let mut content = String::new();
    match data_file.read_to_string(&mut content) {
        Err(e) => return Err(e),
        Ok(_) => {}
    }

    Ok(content)
}
