use std::fs::{create_dir_all, File};
use std::io::prelude::*;
use std::path::{Path, PathBuf};

use crate::path::template_filename;

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

    /// Returns the directory path where all the template files are stored.
    pub fn path(&self) -> PathBuf {
        PathBuf::from(self.dirname.to_owned())
    }

    /// Writes contents to a template file.
    ///
    /// Opens a new file is template file does not exist, otherwise
    /// overwrites existing template file.
    pub fn add_template(&self, lang: String, contents: &str) -> std::io::Result<PathBuf> {
        let filename = template_filename(&lang).unwrap_or_default();
        let template_path = self.path().join(filename);
        let mut file = File::create(&template_path)?;

        file.write_all(contents.as_bytes())?;

        Ok(template_path)
    }
}
