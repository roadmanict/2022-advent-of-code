use std::{fs::File, io, path::Path};

struct FileReader {
    file: Box<dyn FileWrapper>,
}

impl FileReader {
    fn nullable() -> FileReader {
        FileReader {
            file: StubbedFile::new(),
        }
    }

    fn new() -> FileReader {
        FileReader {
            file: RealFile::new(),
        }
    }

    fn open(&self, path: &String) -> Result<String, io::Error> {
        let data_file_path = Path::new(&path);

        let mut data_file = match self.file.open(&data_file_path) {
            Err(e) => return Err(e),
            Ok(file) => file,
        };
        let mut content = String::new();
        data_file.read_to_string(&mut content);

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

struct StubbedFile {}

impl StubbedFile {
    fn new() -> Box<StubbedFile> {
        Box::new(StubbedFile {})
    }
}

impl FileWrapper for StubbedFile {
    fn open(&self, path: &Path) -> Result<File, io::Error> {
        todo!()
    }
}
