use ignorance::url::create_url;

#[test]
fn valid_url() {
    let expected_domain = "https://raw.githubusercontent.com/github/gitignore/master/";
    let gitignore_ext = ".gitignore";
    let url = create_url("c++".to_owned());
    let url_len = url.len();

    // check domain
    assert_eq!(&url[..expected_domain.len()], expected_domain);
    // check extension
    assert_eq!(&url[url_len - gitignore_ext.len()..], gitignore_ext);
    // check full url created
    assert_eq!(
        url,
        "https://raw.githubusercontent.com/github/gitignore/master/C++.gitignore"
    );
}

#[test]
fn first_letter_uppercase() {
    let lang = "python";
    let url = create_url(lang.to_owned());
    let len = url.len();
    let first_letter_index = len - lang.len() - 10;

    assert_eq!(
        &url[first_letter_index..first_letter_index + 1],
        &lang[0..1].to_uppercase()
    );
}

#[test]
#[should_panic(expected = "lang should not be empty")]
fn empty_string() {
    create_url("".to_owned());
}

#[test]
#[should_panic(expected = "lang should be ascii")]
fn non_ascii() {
    create_url(".网络".to_owned());
}
