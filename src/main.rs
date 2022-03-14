use std::env;
use std::process;

//minigrep is this project and basically can be used to refer to lib.rs
use minigrep::Config;

//for order we will make a lib.rs file that will contain the most of the logic
//and functions while main.rs will parse arguments and make calls to them
fn main() {
    // .args will give us an iterator for the arguments passed in and 
    // .collect will turn that into a collection
    // this is why we specify the type though because collect needs to know
    // which type of collection to turn it into
    //pass arguments like: "cargo run [arg] [arg]"...
    //let args: Vec<String> = env::args().collect();
    // we will replace this with an iterator
    
    //we will have new take the iteratir env::args gives us to avoid having to use
    // the inneficient clone method
    let config: Config = Config::new(env::args()).unwrap_or_else( |err| {
        eprintln!("problem parsing arguments {}", err);
        process::exit(0);
    });

    //if the call to run results in the error variant then execute the following
    if let Err(e) = minigrep::run(config){
        //use eprintln!() to print to the standard error stream
        //what if someone pipes output into a file that file will be full of error
        // messages and they will not even see an error has occured
        eprintln!("Application error: {}", e);
        process::exit(0);
    };




    
}

