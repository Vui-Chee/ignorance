mod url;

use clap::{App, Arg};

use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use url::create_url;

// TODO: Check if .gitignore already exists, if so, ask perm before overwriting.
// TODO: cache gitignore contents for each language
// TODO: edit gitignore file with editor
// TODO: force update request
// TODO: write tests to check if request still works
// TODO: write tests for command line cli

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = App::new("ignorance")
        .version("1.0")
        .author("Vui Chee <vuicheesiew@gmail.com>")
        .about("generates ignore files for you")
        .arg(
            Arg::with_name("lang")
                .help("Fetch <LANGUAGE> .gitignore file.")
                .value_name("LANGUAGE")
                .required(true),
        )
        .get_matches();

    if let Some(lang) = matches.value_of("lang") {
        let gitignore_url = create_url(lang.to_owned());
        let client = reqwest::Client::new();
        let res = client.get(gitignore_url).send().await?;

        let ignore_filepath = Path::new(".gitignore");
        let mut file = match File::create(&ignore_filepath) {
            Err(why) => panic!("couldn't create {}: {}", ignore_filepath.display(), why),
            Ok(file) => file,
        };

        let text = res.text().await?;
        match file.write_all(text.as_bytes()) {
            Err(why) => panic!("couldn't write to {}: {}", ignore_filepath.display(), why),
            Ok(_) => println!("successfully wrote to {}", ignore_filepath.display()),
        }
    }

    Ok(())
}
