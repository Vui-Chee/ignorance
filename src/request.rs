use std::path::Path;

use crate::url::template_filename;

pub fn create_url(lang: &str) -> String {
    if lang.is_empty() {
        panic!("lang should not be empty");
    }

    if !lang.is_ascii() {
        panic!("lang should be ascii");
    }

    let domain = Path::new("https://raw.githubusercontent.com/github/gitignore/master/");

    domain
        .join(template_filename(lang).unwrap())
        .to_str()
        .unwrap()
        .to_string()
}

pub async fn fetch_template(lang: &str) -> Result<reqwest::Response, reqwest::Error> {
    let gitignore_url = create_url(lang);
    let client = reqwest::Client::new();

    client.get(gitignore_url).send().await
}
