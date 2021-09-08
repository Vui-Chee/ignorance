use std::path::Path;

pub fn create_url(mut lang: String) -> String {
    if lang.is_empty() {
        panic!("lang should not be empty");
    }

    if !lang.is_ascii() {
        panic!("lang should be ascii");
    }

    // Make first letter uppercase
    if let Some(r) = lang.get_mut(0..1) {
        r.make_ascii_uppercase();
    }
    let domain = Path::new("https://raw.githubusercontent.com/github/gitignore/master/");
    let extension = ".gitignore";

    String::from(
        domain
            .join(format!("{}{}", lang, extension))
            .to_str()
            .unwrap(),
    )
}
