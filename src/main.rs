#[macro_use]
extern crate lazy_static;

#[cfg(not(debug_assertions))]
mod loader;

mod file;
mod language;
mod prompt;
mod request;
mod url;

use clap::{App, Arg};

use std::fs::copy;
use std::io::{stdout, Write};
use std::process::exit;

#[cfg(not(debug_assertions))]
use loader::display_loader;

use prompt::prompt_user_before_overwrite;

use file::Storage;
use request::fetch_template;
use url::{template_dirpath, template_filepath};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = App::new("ignorance")
        .version(env!("CARGO_PKG_VERSION"))
        .author("Vui Chee <vuicheesiew@gmail.com>")
        .about("generates ignore files for you")
        .arg(
            Arg::with_name("update")
                .short("u")
                .help("Refetch template file from source"),
        )
        .arg(
            Arg::with_name("force")
                .short("f")
                .help("Just force overwrite if .gitignore exists"),
        )
        .arg(
            Arg::with_name("lang")
                .help("Create <LANGUAGE> .gitignore file.")
                .value_name("LANGUAGE")
                .required(true),
        )
        .get_matches();

    if let Some(lang) = matches.value_of("lang") {
        let filepath = template_filepath(lang);
        let storage_dirpath = template_dirpath();

        if filepath == storage_dirpath {
            eprintln!("Language Not Found");
            exit(1);
        }

        // Prompt user before overwriting existing .gitignore
        if !matches.is_present("force") {
            prompt_user_before_overwrite()?;
        }

        // create template dir at home path.
        let storage = Storage::new(storage_dirpath.as_path());

        #[cfg(not(debug_assertions))]
        let child = display_loader(10);

        // Fetch from api if any of the two conditions are met:
        // 1. force_update option is applied
        // 2. template file does not exist
        if matches.is_present("update") || !filepath.exists() {
            let response = fetch_template(lang).await?;

            #[cfg(not(debug_assertions))]
            child.join().unwrap();

            if response.status() >= reqwest::StatusCode::BAD_REQUEST {
                eprintln!("Language Not Found");
                exit(1);
            }

            let template = response.text().await?;
            storage?.add_template(lang.to_owned(), &template)?;
        } else {
            #[cfg(not(debug_assertions))]
            child.join().unwrap();
        }

        stdout().flush()?;

        // Otherwise, read contents from template filepath.
        // NOTE: copy will create .gitignore if it does not exist.
        match copy(filepath, ".gitignore") {
            Ok(_) => {
                println!("Successfully generated .gitignore");
            }
            Err(_err) => {
                eprintln!("Cannot find language template");
                exit(1);
            }
        }
    }

    Ok(())
}
