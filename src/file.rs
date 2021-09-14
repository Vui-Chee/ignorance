use std::fs::{create_dir_all, File};
use std::io::prelude::*;
use std::path::{Path, PathBuf};

#[derive(Debug, PartialEq)]
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

    /// Returns template filename in the form of `<LANG>.gitignore`.
    pub fn template_filename(&self, lang: String) -> String {
        format!("{}.gitignore", lang)
    }

    /// Writes contents to a template file.
    ///
    /// Opens a new file is template file does not exist, otherwise
    /// overwrites existing template file.
    pub fn add_template(&self, lang: String, contents: &str) -> std::io::Result<()> {
        let template_path = self.path().join(self.template_filename(lang));
        let mut file = File::create(&template_path)?;
        eprintln!(
            "INSIDE ADD_TEMPLATE {:?}, exists {}",
            file,
            template_path.exists()
        );
        file.write_all(contents.as_bytes())?;

        Ok(())
    }
}
