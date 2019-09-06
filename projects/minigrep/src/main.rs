extern crate minigrep;

use std::env;
use std::process;
use minigrep::Config;


fn main() {
    // Use `collect` to convert the arguments from cmdline into a collection
    // in this case, a vector
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("There was a problem parsing the arguments: {}", err);
        process::exit(1);
    });
    // if let style instead of unwrap_or_else above as run does not return
    // anything that we can unwrap.
    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }

}
