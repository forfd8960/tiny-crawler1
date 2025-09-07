use std::fs::File;
use std::io::Write;
use std::path::Path;

use crate::errors::Errors;

#[derive(Debug, Clone)]
pub struct Page {
    pub title: String,
    pub content: String,
    pub links: Vec<String>,
    pub depth: usize,
}

impl Page {
    pub fn new(title: String, content: String, links: Vec<String>, depth: usize) -> Self {
        Self {
            title,
            content,
            links,
            depth,
        }
    }
}

#[derive(Debug, Clone)]
pub struct DataStore {
    pub store_dir: String,
}

impl DataStore {
    pub fn new(dir: String) -> Result<Self, Errors> {
        let path = Path::new(&dir);
        std::fs::create_dir_all(path)?;
        Ok(Self { store_dir: dir })
    }

    pub fn save_page(&self, page: &Page) -> Result<(), Errors> {
        // Create the full file path
        let full_path = Path::new(&self.store_dir).join(page.title.clone());

        // Create or open the file
        let mut file = File::create(full_path)?;

        // Write content to file
        file.write_all(page.content.as_bytes())?;

        Ok(())
    }
}
