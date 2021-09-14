use std::io::{stdin, stdout, Write};
use std::path::Path;
use std::process::exit;

pub fn prompt_user_before_overwrite() -> std::io::Result<()> {
    // Prompt user before overwriting existing .gitignore
    if Path::new(".gitignore").exists() {
        let mut s = String::new();
        print!("Overwrite existing .gitignore? [y/n]: ");
        stdout().flush()?;
        stdin()
            .read_line(&mut s)
            .expect("Did not enter a correct string");

        // check if valid Y
        if s != "y\n" && s != "Y\n" {
            eprintln!("Overwrite cancelled");
            exit(1);
        }
    }

    Ok(())
}
