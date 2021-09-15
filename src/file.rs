use std::fs::{create_dir_all, File};
use std::io::prelude::*;
use std::path::{Path, PathBuf};

use crate::language::LANGUAGES_MAP;

/// The `Storage` manages how the .gitignore templates are stored in
/// your filesystem.
///
/// The template folder is located at the respective OS home path
/// (using dirs::home_dir). It will have whatever name (dirname)
/// you initialize on construction.
///
/// Inside the template folder, the template files are named as such:
/// [language].gitignore
///
/// NOTE: The [language] refers to the github repo filename as seen
/// in https://github.com/github/gitignore.
///
/// This filename is retrieved via an in-memory map called
/// LANGUAGES_MAP.
pub struct Storage<P: AsRef<Path> + AsRef<std::ffi::OsStr>> {
    home_path: P,    // root directory where the `dirname` folder is stored
    dirname: String, // name of the folder storing the template files
}

impl<P: AsRef<Path> + AsRef<std::ffi::OsStr>> Storage<P> {
    /// Returns a `Storage`.
    ///
    /// Also creates the directory where all templates will be stored.
    pub fn new(home_path: P, dirname: &str) -> std::io::Result<Self> {
        let dirpath = PathBuf::from(&home_path).join(dirname);

        create_dir_all(&dirpath)?;

        Ok(Storage {
            home_path,
            dirname: dirname.to_owned(),
        })
    }

    /// Returns the filename of that language based on LANGUAGES_MAP.
    pub fn filename(&self, lang: &str) -> Option<String> {
        let filename = LANGUAGES_MAP.get(&lang.to_ascii_lowercase() as &str);

        if let Some(filename) = filename {
            let extension = ".gitignore";

            return Some(format!("{}{}", filename, extension));
        }

        None
    }

    /// Returns the path where all template files are stored.
    pub fn dirpath(&self) -> PathBuf {
        PathBuf::from(&self.home_path).join(&self.dirname)
    }

    /// Returns a PathBuf of the location where the template file is stored.
    ///
    /// Storage will store the contents of that .gitignore
    /// under this path.
    pub fn filepath(&self, lang: &str) -> Option<PathBuf> {
        self.filename(lang)
            .map(|filename| self.dirpath().join(filename))
    }

    /// Writes contents to a template file.
    ///
    /// Opens a new file is template file does not exist, otherwise
    /// overwrites existing template file.
    pub fn add_template(&self, lang: &str, contents: &str) -> std::io::Result<PathBuf> {
        let template_path = self.filepath(lang).unwrap();
        let mut file = File::create(&template_path)?;

        file.write_all(contents.as_bytes())?;

        Ok(template_path)
    }
}
