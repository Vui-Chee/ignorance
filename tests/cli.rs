use assert_cmd::prelude::*;

use std::process::Command;

#[test]
fn cli_no_args() {
    Command::cargo_bin("ignorance").unwrap().assert().failure();
}
