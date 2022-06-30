// Bringing the std::env module into scope with use so we can use its args function
// since args is nested in two levels of modules we can just bring the modules in with use so we can use other functions from std::env 
use std::env;

// bring in std::fs (filesystem) to handle files
use std::fs;
fn main() {
    //Call env::args and use collect to turn the iterator into a vector containing all the values produced by the iterator. 
    //We explicitly annotate the type of args to specify that we want a vector of strings
    let args: Vec<String> = env::args().collect();

    //Saving the values of the two arguments into variables so we can use them throughout the rest of the program

    let query = &args[1]; //first argument is the string we're searching for 
    let filename = &args[2]; //second argument is the filename

    println!("{:?}", args); // displays the arguments we passed through

    println!("Searching for {}", query);
    println!("In file {}", filename);

    //take filename, open file and return Result<String> of the file's contents
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    //checking that program works at this point
    println!("With text:\n{}", contents);


}
