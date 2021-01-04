use std::env;
use std::process;

fn main() {

    let config = minigrep::Config::new(env::args()).unwrap_or_else( |err| {
        println!("The error is {}", err);
        process::exit(1);
    });
    
    if let Err(e) = minigrep::run(config) {
        eprintln!("The application error is {}", e);
        process::exit(1);
    };
}

