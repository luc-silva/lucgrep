use std::process;

use lucgrep::{*};

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let cfg = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("Searching for {}", cfg.query);
    println!("In file {}", cfg.file_path);

    if let Err(e) = run(cfg) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
