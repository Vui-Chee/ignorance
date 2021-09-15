use dirs::home_dir;

use ignorance::path::{template_filename, template_filepath};

#[test]
fn valid_template_filename() {
    let filename = template_filename("c++");
    assert_eq!(filename.unwrap(), "C++.gitignore");
    let filename = template_filename("igorpro");
    assert_eq!(filename.unwrap(), "IGORPro.gitignore");
    let filename = template_filename("craftcms");
    assert_eq!(filename.unwrap(), "CraftCMS.gitignore");
    let filename = template_filename("episerver");
    assert_eq!(filename.unwrap(), "EPiServer.gitignore");
}

#[test]
fn invalid_template_filename() {
    assert_eq!(template_filename("asdf"), None);
}

#[test]
fn valid_template_filepath() {
    assert_eq!(
        template_filepath("c++").unwrap(),
        home_dir().unwrap().join(".ignorance").join("C++.gitignore")
    );
}

#[test]
fn invalid_template_filepath() {
    assert_eq!(template_filepath("asdf"), None);
}
