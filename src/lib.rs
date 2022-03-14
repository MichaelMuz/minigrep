
use std::fs;
use std::error::Error;
use std::env;

//dyn Error part basically means return any type of error
pub fn run(config: Config) -> Result<(), Box<dyn Error>>{

    let contents = fs::read_to_string(config.filename)?;

    //no ; after means it is being 'returned' into results
    let results: Vec<&str> = if config.case_sensitive{
        search(&config.query, &contents)

    } else{
        search_case_insensitive(&config.query, &contents)
    };

    
    for line in results{
        println!("{}", line);
    }
    

    Ok(())

}

pub struct Config{
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}
impl Config{
    //we will take env::Args rather than a vector of the arguments
    // env::Args is a specific type, the env::args() function returns an Args struct
    // the Args struct implements the iterator trait, the stuct functions as an 
    // iterator of the arguments of a process
    //the error message is a string literal that willl live as long as the program runs
    // thus we can set its lifetime to static
    pub fn new(mut args: env::Args) -> Result<Config, &'static str>{
        //gets the first arg out of the way since we do not need it
        args.next();

        let query = match args.next(){
            Some(x) => x,
            None => return Err("Did not get a query string"),
            //return will return to overall function not into query, it overrides
        };

        let filename = match args.next(){
            Some(arg) => arg,
            None => return Err("Did not get a filename string"),
        };

        if args.len() < 3 {
            return Err("Not Enough Arguments");
        }
    
        // query and filename own their string



        /*
        let query = match args.get(1){
            Some(x) => x,
            None => return Err("issue getting query"),
        }.to_string();

        let filename = match args.get(2){
            Some(x) => x,
            None => return Err("issue getting filename"),
        }.to_string();

        */

        //var function in enviroment module takes a key to an enviroment variable 
        // and returns a result type if the key exists and it is set then the
        // result will be Ok() containing the result value, otherwise the result
        // will be an error then we call .is_err() which will return a boolean
        // that will say if it is an error or not
        //basically if that key is set we get true if it is not then we get false
        //we can set the enviroment variable by typing 
        // "set CASE_INSENSITIVE=true" before running the program 
        // replace "set" with "export" for linux, unix, mac. set is for windows
        //can unset the variable using "unset", remember it only checks if the 
        // enviromental variable is set not if it is true or false
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
        

        Ok(Config{query, filename, case_sensitive})
    }
}

//need lifetimes anytime you return a reference from a function
//we want the lifetime to be tied to the lifetime of contents because the
// returned will be lines of contents
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str>{

    //more efficiency with iterators:
    //for more understanding check out iterators.rs in our rust_tchek
    contents.lines().filter(|li| li.contains(query)).collect()


    /*
    let mut result: Vec<&str> = Vec::new();

    for line in contents.lines() {

        if line.contains(&query){
            result.push(line.trim());
        }
    }
    result

    */
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str>{
    let mut result: Vec<&str> = Vec::new();

    for line in contents.lines() {

        if line.to_lowercase().contains(&query.to_lowercase()[..]){
            result.push(line.trim());
        }
    }
    result
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn case_sensitive(){
        let query = "duct";
        let contents = "\
        Rust:
        safe, fast, productive.
        Pick three.
        Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive(){
        let query = "rUsT";
        let contents = "\
        Rust:
        safe, fast, productive.
        Pick three.
        Trust me.";

        assert_eq!(vec!["Rust:", "Trust me."], search_case_insensitive(query, contents));
    }
}