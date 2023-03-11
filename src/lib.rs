use std::error::Error;
use std::fs;
pub fn parse_config(args: &[String]) -> Config {
    //Parsing the command line arguments
    let query = args[1].clone();
    let filename = args[2].clone();

    Config { query, filename }
}

pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        //Checking if the user has provided enough arguments
        if args.len() < 3 {
            return Err("Not enough arguments");
        } else if args.len() > 3 {
            return Err("Too many arguments");
        }

        //Parsing the command line arguments
        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(config.filename)?;
    println!("\nWith text:\n{} ", content);

    Ok(())
}
