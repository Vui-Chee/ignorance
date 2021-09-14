use crate::url::create_url;

pub async fn fetch_template(lang: &str) -> Result<reqwest::Response, reqwest::Error> {
    let gitignore_url = create_url(lang);
    let client = reqwest::Client::new();

    client.get(gitignore_url).send().await
}
