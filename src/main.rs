use std::env;
use std::process;

use minigrep::Config;

fn main() {
    let args:Vec<String> = env::args().collect();

    // command parse error handle
    let config = Config::build(&args).unwrap_or_else(|err|{
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    // command run error handle
    if let Err(e) = config.run(){
        eprintln!("Application error: {e}");
        process::exit(1);
    }
     
}


