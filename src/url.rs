use std::path::Path;

use crate::language::LANGUAGES_MAP;

pub fn create_url(mut lang: &str) -> String {
    if lang.is_empty() {
        panic!("lang should not be empty");
    }

    if !lang.is_ascii() {
        panic!("lang should be ascii");
    }

    lang = LANGUAGES_MAP.get(lang).unwrap();
    let domain = Path::new("https://raw.githubusercontent.com/github/gitignore/master/");
    let extension = ".gitignore";

    String::from(
        domain
            .join(format!("{}{}", lang, extension))
            .to_str()
            .unwrap(),
    )
}
