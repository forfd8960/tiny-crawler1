use std::fs::File;
use std::io::Write;
use std::path::Path;

use crate::errors::Errors;

#[derive(Debug)]
pub struct DataStore<'a> {
    pub store_dir: &'a Path,
}

impl<'a> DataStore<'a> {
    pub fn new(dir: &'a Path) -> Self {
        Self { store_dir: dir }
    }

    pub fn save_data(&self, file_name: &str, content: &str) -> Result<(), Errors> {
        // Ensure the directory exists
        std::fs::create_dir_all(self.store_dir)?;

        // Create the full file path
        let full_path = Path::new(&self.store_dir).join(file_name);

        // Create or open the file
        let mut file = File::create(full_path)?;

        // Write content to file
        file.write_all(content.as_bytes())?;

        Ok(())
    }
}
