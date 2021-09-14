#[macro_use]
extern crate lazy_static;

mod file;
mod language;
mod request;
mod url;

use clap::{App, Arg};

use std::fs::copy;
use std::process::exit;

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
            Arg::with_name("lang")
                .help("Create <LANGUAGE> .gitignore file.")
                .value_name("LANGUAGE")
                .required(true),
        )
        .get_matches();

    let storage_dirpath = template_dirpath();
    let storage = Storage::new(storage_dirpath.as_path()); // create template dir at home path.

    if let Some(lang) = matches.value_of("lang") {
        let filepath = template_filepath(lang);

        // Fetch from api if any of the two conditions are met:
        // 1. force_update option is applied
        // 2. template file does not exist
        if matches.is_present("update") || !filepath.exists() {
            let response = fetch_template(lang).await?;

            if response.status() >= reqwest::StatusCode::BAD_REQUEST {
                eprintln!("Language Not Found");
                exit(1);
            }

            let template = response.text().await?;
            storage?.add_template(lang.to_owned(), &template)?;
        }

        // Otherwise, read contents from template filepath.
        // NOTE: copy will create .gitignore if it does not exist.
        match copy(filepath, ".gitignore") {
            Ok(_) => {}
            Err(_err) => {
                eprintln!("Not a valid language");
                exit(1);
            }
        }
    }

    Ok(())
}
