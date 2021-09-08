use dirs::home_dir;
use std::fs::{create_dir, File};
use std::io::prelude::*;
use std::path::PathBuf;

#[derive(Debug, PartialEq)]
pub struct Storage {
    dirname: String,
}

impl Storage {
    /// Returns a `Storage`.
    ///
    /// Also creates the directory where all templates will be stored.
    pub fn new(dirname: String) -> std::io::Result<Self> {
        if let Some(home) = home_dir() {
            let dirpath = home.join(&dirname);
            create_dir(dirpath)?;
        }

        Ok(Storage { dirname })
    }

    /// Returns the directory path where all the template files are stored.
    pub fn path(&self) -> Option<PathBuf> {
        if let Some(home) = home_dir() {
            return Some(home.join(&self.dirname));
        }

        None
    }

    /// Returns template filename in the form of `<LANG>.gitignore`.
    pub fn template_filename(&self, lang: String) -> String {
        format!("{}.gitignore", lang)
    }

    /// Returns a contents of the template file (as String) for that language.
    pub fn get_template(&self, lang: String) -> std::io::Result<String> {
        let template_path = self.path().unwrap().join(self.template_filename(lang));
        let mut file = File::open(template_path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        Ok(contents)
    }

    /// Writes contents to a template file.
    ///
    /// Opens a new file is template file does not exist, otherwise
    /// overwrites existing template file.
    pub fn add_template(&self, lang: String, contents: &str) -> std::io::Result<()> {
        let template_path = self.path().unwrap().join(self.template_filename(lang));
        let mut file = File::create(template_path)?;
        file.write_all(contents.as_bytes())?;

        Ok(())
    }
}
