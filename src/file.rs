use std::fs::{create_dir_all, File};
use std::io::prelude::*;
use std::path::Path;

#[derive(Debug)]
pub struct Storage {
    dirname: String,
}

impl Storage {
    /// Returns a `Storage`.
    ///
    /// Also creates the directory where all templates will be stored.
    pub fn new(dirpath: &Path) -> std::io::Result<Self> {
        create_dir_all(&dirpath)?;

        Ok(Storage {
            dirname: dirpath.to_str().unwrap().to_owned(),
        })
    }

    /// Writes contents to a template file.
    ///
    /// Opens a new file is template file does not exist, otherwise
    /// overwrites existing template file.
    pub fn add_template<P: AsRef<Path>>(
        &self,
        template_path: P,
        contents: &str,
    ) -> std::io::Result<P> {
        let mut file = File::create(&template_path)?;

        file.write_all(contents.as_bytes())?;

        Ok(template_path)
    }
}
