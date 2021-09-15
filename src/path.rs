use std::path::{Path, PathBuf};

use crate::language::LANGUAGES_MAP;

#[derive(Debug, PartialEq)]
pub struct TemplatePath<P: AsRef<Path> + AsRef<std::ffi::OsStr>> {
    home_path: P,
    dirname: String,
}

impl<P: AsRef<Path> + AsRef<std::ffi::OsStr>> TemplatePath<P> {
    // This way, home_dir is call only once
    pub fn new(home_path: P, dirname: &str) -> Self {
        TemplatePath {
            home_path,
            dirname: dirname.to_owned(),
        }
    }

    pub fn filename(&self, lang: &str) -> Option<String> {
        let filename = LANGUAGES_MAP.get(&lang.to_ascii_lowercase() as &str);

        if let Some(filename) = filename {
            let extension = ".gitignore";

            return Some(format!("{}{}", filename, extension));
        }

        None
    }

    pub fn dirpath(&self) -> PathBuf {
        PathBuf::from(&self.home_path).join(&self.dirname)
    }

    pub fn filepath(&self, lang: &str) -> Option<PathBuf> {
        self.filename(lang)
            .map(|filename| self.dirpath().join(filename))
    }
}
