use ignorance::language::LANGUAGES_MAP;
use ignorance::request::create_url;

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
