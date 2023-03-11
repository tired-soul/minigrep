use std::env;
use std::fs;
fn main() {
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let filename = &args[2];

    //print query and filename
    println!("Searching for \"{}\"", query);
    println!("In file \"{}\"", filename);

    let content = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    println!("\nWith text:\n{} ", content);
}
