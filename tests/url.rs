use dirs::home_dir;

use ignorance::language::LANGUAGES_MAP;
use ignorance::url::{create_url, template_filename, template_filepath};

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
        template_filepath("c++"),
        home_dir().unwrap().join(".ignorance").join("C++.gitignore")
    );
}

#[test]
fn invalid_template_filepath() {
    assert_eq!(
        template_filepath("asdf"),
        home_dir().unwrap().join(".ignorance")
    );
}

fn check_lang_url(lang: &str) {
    let expected_domain = "https://raw.githubusercontent.com/github/gitignore/master/";
    let gitignore_ext = ".gitignore";
    let url = create_url(lang);
    let url_len = url.len();
    let lang_filename = LANGUAGES_MAP.get(lang).unwrap();

    // check domain
    assert_eq!(&url[..expected_domain.len()], expected_domain);
    // check extension
    assert_eq!(&url[url_len - gitignore_ext.len()..], gitignore_ext);
    // check full url created
    assert_eq!(
        url,
        format!(
            "https://raw.githubusercontent.com/github/gitignore/master/{}.gitignore",
            lang_filename
        )
    );
}

#[test]
fn get_valid_url_for_lang() {
    check_lang_url("c++");
    check_lang_url("igorpro");
    check_lang_url("craftcms");
    check_lang_url("episerver");
}

#[test]
#[should_panic(expected = "lang should not be empty")]
fn empty_string() {
    create_url("");
}

#[test]
#[should_panic(expected = "lang should be ascii")]
fn non_ascii() {
    create_url(".网络");
}
