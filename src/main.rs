use std::{env, error::Error, process};

use minigrep::Config;

fn main() -> Result<(), Box<dyn Error>> {
    let cfg = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("An error occurred while parsing args: {err}");
        process::exit(1)
    });

    if let Err(e) = minigrep::run(cfg) {
        eprintln!("An error occurred: {}", e);
    }

    return Ok(());
}
