use dirs::home_dir;
use std::path::{Path, PathBuf};

use crate::language::LANGUAGES_MAP;

static TEMPLATE_DIRNAME: &str = ".ignorance";

pub fn template_filename(lang: &str) -> Option<String> {
    let filename = LANGUAGES_MAP.get(&lang.to_ascii_lowercase() as &str);

    if let Some(filename) = filename {
        let extension = ".gitignore";

        return Some(format!("{}{}", filename, extension));
    }

    None
}

pub fn template_dirpath() -> PathBuf {
    home_dir().unwrap().join(TEMPLATE_DIRNAME)
}

pub fn template_filepath(lang: &str) -> PathBuf {
    template_dirpath().join(template_filename(lang).unwrap_or_else(|| "".to_owned()))
}

pub fn create_url(lang: &str) -> String {
    if lang.is_empty() {
        panic!("lang should not be empty");
    }

    if !lang.is_ascii() {
        panic!("lang should be ascii");
    }

    let domain = Path::new("https://raw.githubusercontent.com/github/gitignore/master/");

    domain
        .join(template_filename(lang).unwrap_or_else(|| "".to_owned()))
        .to_str()
        .unwrap()
        .to_string()
}
