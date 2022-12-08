use std::env;
use std::process;

use minigrep::Config;

fn main() {
    let arguments: Vec<String> = env::args().collect();

    let config: Config = Config::build(&arguments).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!(
        "Searching for {0} in file {1}",
        config.query, config.file_path
    );

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application Error: {e}");
        process::exit(1);
    };
}
