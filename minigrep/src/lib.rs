// Bringing the std::env module into scope with use so we can use its args function
// since args is nested in two levels of modules we can just bring the modules in with use so we can use other functions from std::env 
use std::error::Error;
// bring in std::fs (filesystem) to handle files
use std::fs;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    //take filename, open file and return Result<String> of the file's contents
    let contents = fs::read_to_string(config.filename)?;
    //checking that program works at this point
    println!("With text:\n{}", contents);
    Ok(())


}
pub struct Config {
    query: String,
    filename: String,
}
// pass args to parse_config, pull out query and filename and return them as a tuple
impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3{
            return Err("not enough arguments");
        }
        let query = args[1].clone(); //first argument is the string we're searching for 
        let filename = args[2].clone(); //second argument is the filename

        Ok(Config { query, filename })
    }
}