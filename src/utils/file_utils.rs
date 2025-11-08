use std::path::{Path, PathBuf};
use anyhow::Result;
use walkdir::WalkDir;

pub struct FileUtils;

impl FileUtils {
    pub fn find_files(root: &Path, extension: &str) -> Result<Vec<PathBuf>> {
        let mut files = Vec::new();
        
        for entry in WalkDir::new(root)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.path().is_file())
        {
            if entry.path().extension().map_or(false, |ext| ext == extension) {
                files.push(entry.path().to_path_buf());
            }
        }
        
        Ok(files)
    }

    pub fn read_file(path: &Path) -> Result<String> {
        Ok(std::fs::read_to_string(path)?)
    }

    pub fn write_file(path: &Path, content: &str) -> Result<()> {
        std::fs::write(path, content)?;
        Ok(())
    }
}
