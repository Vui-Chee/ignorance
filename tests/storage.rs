use tempfile::tempdir;

use std::fs::{create_dir, metadata, read_to_string, remove_dir, remove_dir_all, File};
use std::io::prelude::*;

use ignorance::file::Storage;

#[test]
fn new_storage() -> std::io::Result<()> {
    let temp_dir = tempdir()?;
    remove_dir(&temp_dir)?;
    let storage = Storage::new(temp_dir.path())?;

    assert!(temp_dir.path().exists());
    assert!(metadata(storage.path())?.is_dir());

    Ok(())
}

#[test]
fn directory_alrdy_exists() -> std::io::Result<()> {
    let temp_dir = tempdir()?;

    assert!(Storage::new(temp_dir.path()).is_err());

    Ok(())
}

#[test]
fn returns_path() -> std::io::Result<()> {
    let temp_dir = tempdir()?;
    remove_dir(&temp_dir)?;
    let storage = Storage::new(temp_dir.path())?;

    assert!(storage.path().exists());
    assert_eq!(storage.path(), temp_dir.path());

    Ok(())
}

#[test]
fn get_template() -> std::io::Result<()> {
    let temp_dir = tempdir()?;
    remove_dir(&temp_dir)?;
    let storage = Storage::new(temp_dir.path())?;
    let mut file = File::create(&temp_dir.path().join("C++.gitignore"))?;
    let contents = "# Prerequisites\n*.d\n# Compiled Object files\n*.slo\n*.lo\n*.o\n*.obj";
    file.write(contents.as_bytes())?;
    let template = storage.get_template("C++".to_owned())?;

    assert_eq!(template, contents);

    Ok(())
}

#[test]
fn add_template() -> std::io::Result<()> {
    let temp_dir = tempdir()?;
    remove_dir(&temp_dir)?;
    let storage = Storage::new(temp_dir.path())?;
    let path_to_template = temp_dir.path().join("C++.gitignore");

    assert!(!path_to_template.exists());

    let lang = "C++";
    let contents = "# Prerequisites\n*.d\n# Compiled Object files\n*.slo\n*.lo\n*.o\n*.obj";
    storage.add_template(lang.to_owned(), contents)?;

    assert!(path_to_template.exists());
    assert_eq!(read_to_string(path_to_template).unwrap(), contents);

    Ok(())
}

#[test]
fn replace_template() -> std::io::Result<()> {
    let temp_dir = tempdir()?;
    remove_dir(&temp_dir)?;
    let storage = Storage::new(temp_dir.path())?;
    let path_to_template = temp_dir.path().join("C++.gitignore");
    // Create template file with contents to be overwritten.
    let before_contents = "# Prerequisites\n*.d\n# Compiled Object files\n*.slo\n*.lo\n*.o\n*.obj";
    let mut file = File::create(&path_to_template)?;
    file.write(before_contents.as_bytes())?;
    let contents = "# Prerequisites\n*.d\n# Compiled Object files\n*.slo\n*.lo\n*.o\n*.obj\n# Compiled Static libraries\n*.lai";
    storage.add_template("C++".to_owned(), contents)?;

    assert_eq!(read_to_string(path_to_template).unwrap(), contents);

    Ok(())
}
