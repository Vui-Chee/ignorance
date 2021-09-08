use dirs::home_dir;
use std::fs::{create_dir, metadata, read_to_string, remove_dir, remove_dir_all, File};
use std::io::prelude::*;
use std::path::PathBuf;

use ignorance::file::Storage;

static STORAGE_DIR: &str = ".ignorance-test";

fn create_storage_name(testname: &str) -> String {
    format!("{}-{}", STORAGE_DIR, testname)
}

#[test]
fn new_storage() -> std::io::Result<()> {
    let test_storage_name = create_storage_name("new_storage");
    let storage = Storage::new(test_storage_name.to_owned())?;

    assert!(PathBuf::from(home_dir().unwrap().join(&test_storage_name)).exists());
    assert!(metadata(storage.path().unwrap())?.is_dir());

    remove_dir(home_dir().unwrap().join(test_storage_name))?;
    Ok(())
}

#[test]
fn directory_alrdy_exists() {
    let test_storage_name = create_storage_name("directory_alrdy_exists");
    let storage_path = home_dir().unwrap().join(&test_storage_name);
    create_dir(&storage_path).unwrap();

    assert!(Storage::new(test_storage_name).is_err());

    remove_dir(storage_path).unwrap();
}

#[test]
fn returns_path() -> std::io::Result<()> {
    let test_storage_name = create_storage_name("returns_path");
    let storage = Storage::new(test_storage_name.to_owned())?;

    assert!(storage.path().is_some());
    assert_eq!(
        storage.path().unwrap(),
        PathBuf::from(home_dir().unwrap().join(test_storage_name))
    );

    remove_dir(storage.path().unwrap())?;
    Ok(())
}

#[test]
fn get_template() -> std::io::Result<()> {
    let test_storage_name = create_storage_name("get_template");
    let storage = Storage::new(test_storage_name.to_owned())?;
    let storage_path = home_dir().unwrap().join(test_storage_name);
    let mut file = File::create(&storage_path.join("C++.gitignore"))?;
    let contents = "# Prerequisites\n*.d\n# Compiled Object files\n*.slo\n*.lo\n*.o\n*.obj";
    file.write(contents.as_bytes())?;
    let template = storage.get_template("C++".to_owned())?;

    assert_eq!(template, contents);

    remove_dir_all(storage_path).unwrap();
    Ok(())
}

#[test]
fn add_template() -> std::io::Result<()> {
    let test_storage_name = create_storage_name("add_template");
    let storage_path = home_dir().unwrap().join(&test_storage_name);
    let path_to_template = storage_path.join("C++.gitignore");

    assert!(!path_to_template.exists());

    let storage = Storage::new(test_storage_name)?;
    let lang = "C++";
    let contents = "# Prerequisites\n*.d\n# Compiled Object files\n*.slo\n*.lo\n*.o\n*.obj";
    storage.add_template(lang.to_owned(), contents)?;

    assert!(path_to_template.exists());
    assert_eq!(read_to_string(path_to_template).unwrap(), contents);

    remove_dir_all(storage_path).unwrap();
    Ok(())
}

#[test]
fn replace_template() -> std::io::Result<()> {
    let test_storage_name = create_storage_name("replace_template");
    let storage_path = home_dir().unwrap().join(&test_storage_name);
    let path_to_template = storage_path.join("C++.gitignore");
    let storage = Storage::new(test_storage_name)?;
    // Create template file with contents to be overwritten.
    let before_contents = "# Prerequisites\n*.d\n# Compiled Object files\n*.slo\n*.lo\n*.o\n*.obj";
    let mut file = File::create(&path_to_template)?;
    file.write(before_contents.as_bytes())?;
    let contents = "# Prerequisites\n*.d\n# Compiled Object files\n*.slo\n*.lo\n*.o\n*.obj\n# Compiled Static libraries\n*.lai";
    storage.add_template("C++".to_owned(), contents)?;

    assert_eq!(read_to_string(path_to_template).unwrap(), contents);

    remove_dir_all(storage_path).unwrap();
    Ok(())
}
