use minigrep::Config;
use std::env;
use std::fs;
fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap();
    //print query and filename
    println!("Searching for \"{}\"", config.query);
    println!("In file \"{}\"", config.filename);

    let content =
        fs::read_to_string(config.filename).expect("Something went wrong reading the file");

    println!("\nWith text:\n{} ", content);
}
