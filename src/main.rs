use md_parser::config::Config;
use std::env;
use std::process;

fn main() {
    let config = match Config::build(env::args()) {
        Ok(val) => val,
        Err(e) => {
            println!("Error parsing arguments: {e}");
            process::exit(1);
        }
    };

    match md_parser::run(config) {
        Ok(()) => {
            process::exit(0);
        }
        Err(e) => {
            println!("Parser error: {e}");
            process::exit(1);
        }
    }
}
