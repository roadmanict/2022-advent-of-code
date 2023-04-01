use std::{
    fs::File,
    io::{self, Read},
    path::Path,
};

struct FileReader {
    file: Box<dyn FileWrapper>,
}

impl FileReader {
    fn nullable(file_contents: String) -> FileReader {
        FileReader {
            file: StubbedFile::new(file_contents),
        }
    }

    fn new() -> FileReader {
        FileReader {
            file: RealFile::new(),
        }
    }

    fn read_file(&self, path: &String) -> Result<String, io::Error> {
        let data_file_path = Path::new(&path);

        let mut data_file = self.file.open(&data_file_path)?;
        let mut content = String::new();
        data_file.read_to_string(&mut content)?;

        Ok(content)
    }
}

trait FileWrapper {
    fn open(&self, path: &Path) -> Result<File, io::Error>;
}

struct RealFile {}

impl RealFile {
    fn new() -> Box<RealFile> {
        Box::new(RealFile {})
    }
}
impl FileWrapper for RealFile {
    fn open(&self, path: &Path) -> Result<File, io::Error> {
        File::open(path)
    }
}

struct StubbedFile {
    file_contents: String,
}

impl StubbedFile {
    fn new(file_contents: String) -> Box<StubbedFile> {
        Box::new(StubbedFile { file_contents })
    }
}

impl FileWrapper for StubbedFile {
    fn open(&self, path: &Path) -> Result<File, io::Error> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_reader() {
        let file_reader = FileReader::nullable(String::from("Test content"));

        let result = file_reader.read_file(&String::from("some_path_to_file"));

        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            String::from("Test content")
        );
    }
}
