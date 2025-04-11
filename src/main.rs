use std::env;
use std::process;
use md_parser::Config;

fn main() {
    let config = match Config::build(env::args()) {
        Ok(val) => val,
        Err(e) => {
            println!("Error parsing arguments: {e}");
            process::exit(1);
        }
    };

    match md_parser::run(config) {
        Ok(()) => { process::exit(0); },
        Err(e) => {
            println!("Parser error: {e}");
            process::exit(1);
        }
    }
}
