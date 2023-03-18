use std::fs::{self};
use std::path::{Path};

pub struct Filesystem {}

impl Filesystem {
    // This function creates a new instance of the Filesystem struct
    pub fn new() -> Self {
        Self {}
    }

    // This function copies all the files in a folder to an other folder and if that folder doesn't exist it creates it
    pub fn copy_files(&self, source_dir: &Path, dest_dir: &Path) -> Result<(), std::io::Error> {
        // Create destination directory if it doesn't exist
        if !dest_dir.exists() {
            fs::create_dir_all(dest_dir)?;
        }

        // Iterate over files in source directory
        for entry in fs::read_dir(source_dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_file() {
                // Copy file to destination directory
                let dest_file = dest_dir.join(path.file_name().unwrap());
                fs::copy(&path, &dest_file)?;
            } else if path.is_dir() {
                // Recursively copy subdirectory to destination directory
                let dest_subdir = dest_dir.join(path.file_name().unwrap());
                let _= &self.copy_files(&path, &dest_subdir).expect("Failed to copy files");
            }
        }

        Ok(())
    }
}
