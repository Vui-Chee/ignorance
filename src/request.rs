use std::path::Path;

pub fn create_url(filename: &str) -> String {
    if filename.is_empty() {
        panic!("filename should not be empty");
    }

    if !filename.is_ascii() {
        panic!("filename should be ascii");
    }

    Path::new("https://raw.githubusercontent.com/github/gitignore/master/")
        .join(filename)
        .to_str()
        .unwrap()
        .to_string()
}

pub async fn fetch_template(filename: &str) -> Result<reqwest::Response, reqwest::Error> {
    let gitignore_url = create_url(filename);
    let client = reqwest::Client::new();

    client.get(gitignore_url).send().await
}
