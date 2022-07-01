// Bringing the std::env module into scope with use so we can use its args function
// since args is nested in two levels of modules we can just bring the modules in with use so we can use other functions from std::env 


// bring in std::fs (filesystem) to handle files

use std::env;
use std::process;
use minigrip::Config;

fn main() {
    //Call env::args and use collect to turn the iterator into a vector containing all the values produced by the iterator. 
    //We explicitly annotate the type of args to specify that we want a vector of strings
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args); // displays the arguments we passed through
    //Saving the values of the two arguments into variables so we can use them throughout the rest of the program

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
        });


    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    if let Err(e) = minigrip::run(config) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}

