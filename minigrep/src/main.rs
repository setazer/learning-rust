use minigrep::{Config, run};

use std::{env, process};

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Unable to parse config: {}", err);
        process::exit(1);
    });

    if let Err(e) = run(config) {
        eprintln!("Program error: {}", e);
        process::exit(1);
    };
}
