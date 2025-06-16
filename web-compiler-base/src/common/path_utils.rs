use std::path::{Path, PathBuf};

pub fn resolve_file_path_patern(pattern: &String) -> Result<Vec<PathBuf>, Box<dyn std::error::Error>> {
    let patterns = vec![pattern.to_owned()];
    resolve_file_path_paterns(patterns.as_slice())
}

pub fn resolve_file_path_paterns(patterns: &[String]) -> Result<Vec<PathBuf>, Box<dyn std::error::Error>> {
    fn resolve_entry_as_glob(pattern: &str) -> Result<Vec<PathBuf>, Box<dyn std::error::Error>> {
        let mut results = Vec::<PathBuf>::new();
        for pattern in glob::glob(pattern)? {
            match pattern {
                Ok(path) => {
                    results.push(path);
                    continue;
                }
                Err(error) => return Err(Box::new(error)),
            }
        }
        Ok(results)
    }
    fn resolve_entry(pattern: &str) -> Result<Vec<PathBuf>, Box<dyn std::error::Error>> {
        if let Ok(results) = resolve_entry_as_glob(pattern) {
            return Ok(results)
        }
        let path = PathBuf::from(pattern);
        return Ok(vec![path])
    }
    let mut results = Vec::<PathBuf>::new();
    for pattern in patterns {
        match resolve_entry(&pattern) {
            Ok(paths) => {
                results.extend(paths);
            }
            Err(error) => {
                return Err(error)
            }
        }
    }
    Ok(results)
}

pub fn write_output_file_smart(file_path: impl AsRef<Path>, content: impl AsRef<[u8]>) {
    let file_path = file_path.as_ref();
    let new_bytes = content.as_ref();
    // let new_bytes = contents.as_bytes();

    let file_needs_write = match std::fs::read(file_path) {
        Ok(existing_bytes) => {
            existing_bytes != new_bytes
        },
        Err(_) => true, // file doesn't exist or can't be read
    };

    if file_needs_write {
        if let Some(parent_dir) = file_path.parent() {
            std::fs::create_dir_all(parent_dir).unwrap();
        }
        println!("> writing {file_path:?}");
        std::fs::write(file_path, new_bytes).unwrap();
    }
}

#[derive(Debug, Clone)]
pub struct WriteOrSymlinkOutput<'a> {
    pub output_file: &'a Path,
    pub source_file: &'a Path,
    pub contents: &'a [u8],
}

impl<'a> WriteOrSymlinkOutput<'a> {
    pub fn execute(&self) {
        let source_contents = std::fs::read(self.source_file).unwrap();
        if source_contents == self.contents {
            let result = self.write_symlink();
            if result.is_ok() {
                return
            }
        }
        write_output_file_smart(self.output_file, self.contents);
    }
    pub fn write_symlink(&self) -> Result<(), Box<dyn std::error::Error>> {
        let status = super::symlink::create_relative_symlink(self.source_file, self.output_file)?;
        if status.is_updated() {
            println!("> linking {:?} ⬌ {:?}", self.source_file, self.output_file);
        }
        Ok(())
    }
}

