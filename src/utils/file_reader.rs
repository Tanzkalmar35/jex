use std::{fs::File, io::Read};

pub(crate) struct FileReader;

impl FileReader {
    pub(crate) fn read(path: &str) -> Result<Vec<u8>, std::io::Error> {
        let mut file = File::open(path)?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;

        Ok(buffer)
    }
}
