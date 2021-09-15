use dirs::home_dir;

// use std::path::PathBuf;

use ignorance::path::TemplatePath;

#[test]
fn create_template_path() -> std::io::Result<()> {
    let dir = home_dir().unwrap();
    let _template_path = TemplatePath::new(dir.as_path(), ".ignorance");

    // assert_eq!(
    // template_path,
    // TemplatePath {
    // home_path: dir.path(),
    // dirname: ".ignorance".to_owned()
    // }
    // );

    Ok(())
}

#[test]
fn get_dirpath() {
    let dir = home_dir().unwrap();
    let template_path = TemplatePath::new(dir.as_path(), ".ignorance");

    assert_eq!(template_path.dirpath(), dir.join(".ignorance"));
}
