use std::fs::{create_dir, File};
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
    #[allow(dead_code)]
    pub fn new(dirpath: &Path) -> std::io::Result<Self> {
        create_dir(&dirpath)?;

        Ok(Storage {
            dirname: dirpath.to_str().unwrap().to_owned(),
        })
    }

    /// Returns the directory path where all the template files are stored.
    #[allow(dead_code)]
    pub fn path(&self) -> PathBuf {
        PathBuf::from(self.dirname.to_owned())
    }

    /// Returns template filename in the form of `<LANG>.gitignore`.
    #[allow(dead_code)]
    pub fn template_filename(&self, lang: String) -> String {
        format!("{}.gitignore", lang)
    }

    /// Returns a contents of the template file (as String) for that language.
    #[allow(dead_code)]
    pub fn get_template(&self, lang: String) -> std::io::Result<String> {
        let template_path = self.path().join(self.template_filename(lang));
        let mut file = File::open(template_path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        Ok(contents)
    }

    /// Writes contents to a template file.
    ///
    /// Opens a new file is template file does not exist, otherwise
    /// overwrites existing template file.
    #[allow(dead_code)]
    pub fn add_template(&self, lang: String, contents: &str) -> std::io::Result<()> {
        let template_path = self.path().join(self.template_filename(lang));
        let mut file = File::create(template_path)?;
        file.write_all(contents.as_bytes())?;

        Ok(())
    }
}
