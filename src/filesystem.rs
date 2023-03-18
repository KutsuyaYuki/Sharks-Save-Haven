use std::fs::{self};
use std::path::{Path};

pub struct Filesystem {}

impl Filesystem {
    // This function creates a new instance of the Filesystem struct
    pub fn new() -> Self {
        Self {}
    }

    /// Copies all the files in a folder to another folder and creates it if it doesn't exist.
    ///
    /// # Arguments
    ///
    /// * `source_dir` - The path to the source directory.
    /// * `dest_dir` - The path to the destination directory.
    ///
    /// # Errors
    ///
    /// This function will return an error if it fails to create the destination directory or if it
    /// fails to copy any of the files.
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

    pub fn delete_files(&self, dir: &Path) -> Result<(), std::io::Error> {
        // Iterate over files in source directory
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_file() {
                // Delete file
                fs::remove_file(&path)?;
            } else if path.is_dir() {
                // Recursively delete subdirectory
                let _= &self.delete_files(&path).expect("Failed to delete files");
            }
        }

        Ok(())
    }
}
