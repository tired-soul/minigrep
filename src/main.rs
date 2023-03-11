use minigrep::{run, Config};
use std::env;
use std::process::exit;
fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        exit(1);
    });
    //print query and filename
    println!("Searching for \"{}\"", config.query);
    println!("In file \"{}\"", config.filename);

    // run(config).unwrap_or_else(|err| {
    //     println!("File error: {}", err);  // <--- This works, but prefer down below 
    //     exit(1);                          // because run returns () when successful. smh.
    // });
    if let Err(e) = run(config) {
        println!("Application error: {}", e);
        exit(1);
    }
}
