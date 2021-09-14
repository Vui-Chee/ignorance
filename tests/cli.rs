use assert_cmd::prelude::*;
use predicates::str::contains;

use std::fs::{read_to_string, remove_dir, remove_file, write};
use std::path::Path;
use std::process::Command;

use ignorance::url::{template_dirpath, template_filepath};

#[test]
fn cli_no_args() {
    Command::cargo_bin("ignorance").unwrap().assert().failure();
}

#[test]
fn cli_version() {
    Command::cargo_bin("ignorance")
        .unwrap()
        .args(&["-V"])
        .assert()
        .stdout(contains(env!("CARGO_PKG_VERSION")));
}

#[test]
fn cli_update_option_without_lang() {
    Command::cargo_bin("ignorance")
        .unwrap()
        .args(&["-u"])
        .assert()
        .failure();
}

#[test]
fn cli_non_existent_option() {
    Command::cargo_bin("ignorance")
        .unwrap()
        .args(&["-X", "fake-language"])
        .assert()
        .failure();
}

#[test]
fn cli_invalid_input_language() {
    Command::cargo_bin("ignorance")
        .unwrap()
        .args(&["fake-language"])
        .assert()
        .stderr("Language Not Found\n")
        .failure();
}

#[test]
fn cli_invalid_input_language_with_options() {
    Command::cargo_bin("ignorance")
        .unwrap()
        .args(&["-u", "fake-language"])
        .assert()
        .stderr("Language Not Found\n")
        .failure();
    Command::cargo_bin("ignorance")
        .unwrap()
        .args(&["-f", "fake-language"])
        .assert()
        .stderr("Language Not Found\n")
        .failure();
    Command::cargo_bin("ignorance")
        .unwrap()
        .args(&["-uf", "fake-language"])
        .assert()
        .stderr("Language Not Found\n")
        .failure();
}

/// Test multiple cases using the same directory.
///
/// Cases:
///   1) Creates .gitignore if no exist
///   2) Overwrites .gitignore with new contents (using -f)
///   3) Check contents are correct
#[test]
fn cli_integrated() -> std::io::Result<()> {
    let did_template_dir_exists = template_dirpath().exists();

    // Remember current .gitignore contents
    let mut previous_gitignore_contents = String::new();
    let did_gitignore_exist = Path::new(".gitignore").exists();
    if did_gitignore_exist {
        previous_gitignore_contents = read_to_string(".gitignore")?;
        remove_file(".gitignore")?;
    }

    // This should create template directory if no exist.
    Command::cargo_bin("ignorance")
        .unwrap()
        .args(&["opa"])
        .assert()
        .success()
        .stdout("Successfully generated .gitignore\n");

    // Check if template/gitignore files exist.
    assert!(template_filepath("opa").exists());
    assert!(Path::new(".gitignore").exists());

    // Check contents match opa language.
    let gitignore_contents = read_to_string(".gitignore")?;
    let template_contents = read_to_string(template_filepath("opa"))?;
    assert_eq!(template_contents, gitignore_contents);

    // Overwrite contents of gitignore with another language template.
    Command::cargo_bin("ignorance")
        .unwrap()
        .args(&["-f", "igorpro"])
        .assert()
        .success()
        .stdout("Successfully generated .gitignore\n");

    // Check contents match igorpro language.
    let gitignore_contents = read_to_string(".gitignore")?;
    let template_contents = read_to_string(template_filepath("igorpro"))?;
    assert_eq!(template_contents, gitignore_contents);

    // Restore previous gitignore.
    if did_gitignore_exist {
        write(".gitignore", previous_gitignore_contents)?;
    }

    // Finally clean up.
    remove_file(template_filepath("opa"))?;
    remove_file(template_filepath("igorpro"))?;
    if !did_template_dir_exists {
        remove_dir(template_dirpath())?;
    }

    Ok(())
}
