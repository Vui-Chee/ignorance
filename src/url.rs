use std::path::Path;

use crate::language::LANGUAGES_MAP;

pub fn template_filename(lang: &str) -> String {
    let filename = LANGUAGES_MAP.get(lang).unwrap();
    let extension = ".gitignore";

    format!("{}{}", filename, extension)
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
        .join(template_filename(lang))
        .to_str()
        .unwrap()
        .to_string()
}
